//! Lógica de autenticación: login, logout, registro, cambio de contraseña, semilla inicial.

use pagetop::prelude::*;
use pagetop_seaorm::db::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, Set, dbconn,
};

use crate::account::UserStatus;
use crate::config::SETTINGS;
use crate::entity::{role_permission, user};
use crate::error::AuthError;
use crate::identity::{self, Email, Username};
use crate::password;
use crate::permission::UserPermission;
use crate::service::user_admin::{ensure_email_available, ensure_username_available};
use crate::session;

// **< login >**************************************************************************************

/// Valida las credenciales y crea una sesión. Devuelve el session ID.
///
/// Acepta nombre de usuario o email como primer parámetro. Aplica la política de bloqueo
/// por intentos fallidos y rehashea la contraseña si los parámetros de coste han cambiado.
///
/// Sólo con la contraseña correcta se informa de que la cuenta está bloqueada
/// ([`AuthError::AccountBlocked`]) o pendiente de verificar ([`AuthError::AccountPending`]): sin
/// ella, cualquiera sabría así qué cuentas existen y en qué estado están. Por lo mismo, sin cuenta
/// se tarda lo mismo que con una contraseña errónea.
pub async fn login(
    username_or_email: &str,
    plain_password: &str,
    remember: bool,
) -> Result<String, AuthError> {
    let now = Utc::now();

    // Buscar usuario por email o por username, sin distinguir mayúsculas. Un nombre de usuario
    // nunca contiene `@`, así que la entrada sólo puede ser uno de los dos.
    let filter = if username_or_email.contains('@') {
        user::Column::EmailKey.eq(identity::email_key(username_or_email))
    } else {
        user::Column::UsernameKey.eq(identity::username_key(username_or_email))
    };
    let Some(user_model) = user::Entity::find().filter(filter).one(dbconn()).await? else {
        password::verify_password_without_account(plain_password).await;
        return Err(AuthError::InvalidCredentials);
    };

    // Comprobar bloqueo temporal por intentos fallidos.
    if let Some(locked_until) = user_model.locked_until
        && locked_until > now
    {
        return Err(AuthError::AccountLocked);
    }

    // Verificar contraseña.
    if !password::verify_password_async(plain_password, &user_model.password_hash).await {
        register_failed_login(&user_model, now).await?;
        return Err(AuthError::InvalidCredentials);
    }

    // Comprobar estado de la cuenta.
    match UserStatus::from_i16(user_model.status) {
        UserStatus::Blocked => return Err(AuthError::AccountBlocked),
        UserStatus::Pending => return Err(AuthError::AccountPending),
        UserStatus::Active => {}
    }

    // Actualizar last_login_at y resetear contador de fallos.
    let user_id = user_model.id;
    let needs_rehash = password::needs_rehash(&user_model.password_hash);
    let mut active = user_model.into_active_model();
    active.last_login_at = Set(Some(now));
    active.last_access_at = Set(Some(now));
    active.failed_login_count = Set(0);
    active.failed_login_at = Set(None);
    active.locked_until = Set(None);
    if needs_rehash {
        let new_hash = password::hash_password_async(plain_password).await?;
        active.password_hash = Set(new_hash);
    }
    active.update(dbconn()).await?;

    // Crear sesión y devolver el SID.
    session::create_session(user_id, remember)
        .await
        .map_err(AuthError::Database)
}

// **< logout >*************************************************************************************

/// Destruye la sesión indicada.
pub async fn logout(sid: &str) -> Result<(), AuthError> {
    session::destroy_session(sid)
        .await
        .map_err(AuthError::Database)
}

// **< register >***********************************************************************************

/// Registra un nuevo usuario. Devuelve el `user_id` asignado.
///
/// Valida: username según la política de identidad y email (ver
/// [`IdentityPolicy`](crate::config::IdentityPolicy)), longitud de contraseña, coincidencia de
/// confirmación, y que ni el username (o uno que se confunda con él) ni el email estén en uso.
pub async fn register(
    username: &str,
    email: &str,
    plain_password: &str,
    confirm_password: &str,
) -> Result<i32, AuthError> {
    let username = identity::username(username, SETTINGS.identity_policy, None)?;
    let email = identity::email(email)?;
    password::validate_strength(plain_password)?;
    password::passwords_match(plain_password, confirm_password)?;
    ensure_username_available(&username, None).await?;
    ensure_email_available(&email, None).await?;

    let hash = password::hash_password_async(plain_password).await?;
    let now = Utc::now();
    let status = if SETTINGS.require_email_verification {
        UserStatus::Pending
    } else {
        UserStatus::Active
    };

    let mut new_user = user::ActiveModel {
        id: ActiveValue::NotSet,
        email_verified_at: Set(None),
        password_hash: Set(hash),
        status: Set(status.as_i16()),
        language: Set(None),
        timezone: Set(None),
        theme: Set(None),
        display_name: Set(None),
        about: Set(None),
        last_login_at: Set(None),
        last_access_at: Set(None),
        failed_login_count: Set(0),
        locked_until: Set(None),
        is_admin: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    username.set_on(&mut new_user);
    email.set_on(&mut new_user);
    let result = user::Entity::insert(new_user).exec(dbconn()).await?;
    let user_id = result.last_insert_id;

    Ok(user_id)
}

// **< change_own_password >************************************************************************

/// Cambia la contraseña del propio usuario tras comprobar la actual, y cierra el resto de sus
/// sesiones abiertas conservando la indicada en `current_sid` (la de quien hace el cambio).
///
/// Devuelve [`AuthError::InvalidCredentials`] si la contraseña actual no es correcta.
pub(crate) async fn change_own_password(
    user_id: i32,
    current_password: &str,
    new_password: &str,
    current_sid: Option<&str>,
) -> Result<(), AuthError> {
    let user_model = user::Entity::find_by_id(user_id)
        .one(dbconn())
        .await?
        .ok_or(AuthError::UserNotFound)?;
    if !password::verify_password_async(current_password, &user_model.password_hash).await {
        return Err(AuthError::InvalidCredentials);
    }
    password::validate_strength(new_password)?;
    let hash = password::hash_password_async(new_password).await?;

    user::ActiveModel {
        id: Set(user_id),
        password_hash: Set(hash),
        updated_at: Set(Utc::now()),
        ..Default::default()
    }
    .update(dbconn())
    .await?;

    match current_sid {
        Some(sid) => session::destroy_other_sessions(user_id, sid).await?,
        None => session::destroy_user_sessions(user_id).await?,
    }
    Ok(())
}

// **< register_failed_login >**********************************************************************

async fn register_failed_login(
    user_model: &user::Model,
    now: DateTime<Utc>,
) -> Result<(), AuthError> {
    let (count, locked_until) = failed_login_outcome(
        user_model.failed_login_count,
        user_model.failed_login_at,
        now,
        FailedLoginPolicy {
            max: SETTINGS.max_failed_logins,
            window: Duration::seconds(SETTINGS.failed_login_window_secs),
            locked_for: Duration::seconds(SETTINGS.locked_for_secs),
        },
    );
    user::ActiveModel {
        id: Set(user_model.id),
        failed_login_count: Set(count),
        failed_login_at: Set(Some(now)),
        locked_until: Set(locked_until),
        updated_at: Set(now),
        ..Default::default()
    }
    .update(dbconn())
    .await?;
    Ok(())
}

struct FailedLoginPolicy {
    max: i32,
    window: Duration,
    locked_for: Duration,
}

// Nuevo recuento tras un intento fallido en `now` y, si alcanza el máximo, hasta cuándo queda
// bloqueada la cuenta. Un fallo cuenta junto a los anteriores sólo si el último fue hace menos de
// la ventana; si no, el recuento empieza de nuevo. Al bloquear, el recuento vuelve a cero para
// que, pasado el bloqueo, se disponga otra vez de todos los intentos. Con un máximo de `0` nunca
// se bloquea.
fn failed_login_outcome(
    count: i32,
    last_failed: Option<DateTime<Utc>>,
    now: DateTime<Utc>,
    policy: FailedLoginPolicy,
) -> (i32, Option<DateTime<Utc>>) {
    let count = match last_failed {
        Some(t) if now - t < policy.window => count.saturating_add(1),
        _ => 1,
    };
    if policy.max > 0 && count >= policy.max {
        (0, Some(now + policy.locked_for))
    } else {
        (count, None)
    }
}

// **< seed_initial_data >**************************************************************************

/// Prepara una instalación nueva si no existe ningún usuario en la base de datos.
///
/// Se llama desde `Extension::initialize()`. Si la tabla está vacía, crea el administrador
/// con las credenciales configuradas en `[user.seed]`. La contraseña se genera aleatoriamente
/// si no está configurada, y se imprime por stdout una sola vez para que el operador la recoja.
///
/// Concede además al rol "authenticated" los permisos que todo usuario espera tener sobre su
/// propia cuenta: editar su perfil y cambiar su contraseña. Sólo en la instalación nueva; después
/// los gestiona el administrador desde la interfaz.
///
/// Falla con [`AuthError::InvalidSetting`] si el nombre de usuario o el email configurados no son
/// válidos, aunque ya existan usuarios, como cualquier otro ajuste no válido.
pub(crate) async fn seed_initial_data() -> Result<(), AuthError> {
    let cfg = &SETTINGS.seed;
    let username = identity::username(&cfg.admin_username, SETTINGS.identity_policy, None)
        .map_err(|err| invalid_setting("user.seed.admin_username", err))?;
    let email = identity::email(&cfg.admin_email)
        .map_err(|err| invalid_setting("user.seed.admin_email", err))?;
    do_seed(username, email).await;
    Ok(())
}

fn invalid_setting(key: &'static str, source: AuthError) -> AuthError {
    AuthError::InvalidSetting {
        key,
        source: Box::new(source),
    }
}

async fn do_seed(username: Username, email: Email) {
    let count = user::Entity::find().count(dbconn()).await.unwrap_or(1);
    if count > 0 {
        return;
    }

    let cfg = &SETTINGS.seed;
    let (admin_password, generated) = match &cfg.admin_password {
        Some(p) if !p.is_empty() => (p.clone(), false),
        _ => {
            // Generar contraseña aleatoria de 20 caracteres.
            use base64ct::{Base64UrlUnpadded, Encoding};
            use rand_core::{OsRng, RngCore};
            let mut bytes = [0u8; 15];
            OsRng.fill_bytes(&mut bytes);
            (Base64UrlUnpadded::encode_string(&bytes), true)
        }
    };

    let hash = match password::hash_password(&admin_password) {
        Ok(h) => h,
        Err(e) => {
            eprintln!(
                "pagetop-user seed error: failed to hash admin password: {}",
                e
            );
            return;
        }
    };

    let now = Utc::now();
    let admin_username = username.value().to_owned();
    let mut new_admin = user::ActiveModel {
        id: ActiveValue::NotSet,
        email_verified_at: Set(Some(now)),
        password_hash: Set(hash),
        status: Set(UserStatus::Active.as_i16()),
        language: Set(None),
        timezone: Set(None),
        theme: Set(None),
        display_name: Set(Some("Administrator".into())),
        about: Set(None),
        last_login_at: Set(None),
        last_access_at: Set(None),
        failed_login_count: Set(0),
        locked_until: Set(None),
        is_admin: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    username.set_on(&mut new_admin);
    email.set_on(&mut new_admin);

    match user::Entity::insert(new_admin).exec(dbconn()).await {
        Ok(_) => {
            if generated {
                println!(
                    "\npagetop-user: admin account created.\n  username: {}\n  password: {}\n",
                    admin_username, admin_password
                );
            }
        }
        Err(e) => {
            eprintln!("pagetop-user seed error: {}", e);
            return;
        }
    }

    let defaults = [
        UserPermission::EditOwnProfile,
        UserPermission::ChangeOwnPassword,
    ];
    let rows = defaults.map(|perm| role_permission::ActiveModel {
        role_id: Set(crate::AUTHENTICATED_ROLE_ID),
        permission_key: Set(perm.key().into_owned()),
        granted_at: Set(now),
    });
    if let Err(e) = role_permission::Entity::insert_many(rows)
        .exec(dbconn())
        .await
    {
        eprintln!(
            "pagetop-user seed error: failed to grant default permissions: {}",
            e
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(secs: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(1_800_000_000 + secs, 0).unwrap()
    }

    fn outcome(count: i32, last: Option<i64>, now: i64) -> (i32, Option<DateTime<Utc>>) {
        let policy = FailedLoginPolicy {
            max: 3,
            window: Duration::seconds(900),
            locked_for: Duration::seconds(600),
        };
        failed_login_outcome(count, last.map(at), at(now), policy)
    }

    #[pagetop::test]
    async fn failures_within_the_window_accumulate() {
        assert_eq!(outcome(0, None, 0), (1, None));
        assert_eq!(outcome(1, Some(0), 100), (2, None));
    }

    #[pagetop::test]
    async fn reaching_the_maximum_locks_and_resets_the_count() {
        assert_eq!(outcome(2, Some(0), 100), (0, Some(at(700))));
    }

    #[pagetop::test]
    async fn zero_maximum_never_locks() {
        let policy = FailedLoginPolicy {
            max: 0,
            window: Duration::seconds(900),
            locked_for: Duration::seconds(600),
        };
        assert_eq!(
            failed_login_outcome(9, Some(at(0)), at(1), policy),
            (10, None)
        );
    }

    #[pagetop::test]
    async fn a_failure_after_the_window_starts_a_new_count() {
        assert_eq!(outcome(2, Some(0), 900), (1, None));
    }
}
