//! Lógica de autenticación: login, logout, registro, semilla inicial.

use pagetop::datetime::{Duration, NaiveDateTime, Utc};

use pagetop_seaorm::db::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, PaginatorTrait,
    QueryFilter, Set, dbconn,
};

use crate::account::UserStatus;
use crate::config::SETTINGS;
use crate::entity::{user, user_role};
use crate::error::AuthError;
use crate::password;
use crate::session;

// **< login >**************************************************************************************

/// Valida las credenciales y crea una sesión. Devuelve el session ID.
///
/// Acepta nombre de usuario o email como primer parámetro. Aplica la política de bloqueo
/// por intentos fallidos y rehashea la contraseña si los parámetros de coste han cambiado.
pub async fn login(
    username_or_email: &str,
    plain_password: &str,
    remember: bool,
) -> Result<String, AuthError> {
    let now = Utc::now().naive_utc();

    // Buscar usuario por username o email.
    let user_model = user::Entity::find()
        .filter(
            Condition::any()
                .add(user::Column::Username.eq(username_or_email))
                .add(user::Column::Email.eq(username_or_email)),
        )
        .one(dbconn())
        .await?
        .ok_or(AuthError::InvalidCredentials)?;

    // Comprobar bloqueo temporal por intentos fallidos.
    if let Some(locked_until) = user_model.locked_until
        && locked_until > now
    {
        return Err(AuthError::AccountLocked);
    }

    // Comprobar estado de la cuenta.
    match UserStatus::from_i16(user_model.status) {
        UserStatus::Blocked => return Err(AuthError::AccountBlocked),
        UserStatus::Pending => return Err(AuthError::AccountPending),
        UserStatus::Active => {}
    }

    // Verificar contraseña.
    if !password::verify_password(plain_password, &user_model.password_hash) {
        register_failed_login(&user_model, now).await?;
        return Err(AuthError::InvalidCredentials);
    }

    // Actualizar last_login_at y resetear contador de fallos.
    let user_id = user_model.id;
    let needs_rehash = password::needs_rehash(&user_model.password_hash);
    let mut active: user::ActiveModel = user_model.into();
    active.last_login_at = Set(Some(now));
    active.last_access_at = Set(Some(now));
    active.failed_login_count = Set(0);
    active.locked_until = Set(None);
    if needs_rehash {
        let new_hash = password::hash_password(plain_password)?;
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
/// Valida: longitud de contraseña, coincidencia de confirmación, y unicidad de username y email.
pub async fn register(
    username: &str,
    email: &str,
    plain_password: &str,
    confirm_password: &str,
) -> Result<i32, AuthError> {
    password::validate_strength(plain_password)?;
    password::passwords_match(plain_password, confirm_password)?;

    // Comprobar unicidad de username y email.
    if user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(dbconn())
        .await?
        .is_some()
    {
        return Err(AuthError::UsernameTaken);
    }
    if user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(dbconn())
        .await?
        .is_some()
    {
        return Err(AuthError::EmailTaken);
    }

    let hash = password::hash_password(plain_password)?;
    let now = Utc::now().naive_utc();
    let status = if SETTINGS.require_email_verification {
        UserStatus::Pending
    } else {
        UserStatus::Active
    };

    let new_user = user::ActiveModel {
        id: ActiveValue::NotSet,
        username: Set(username.to_owned()),
        email: Set(email.to_owned()),
        email_verified_at: Set(None),
        password_hash: Set(hash),
        status: Set(status.as_i16()),
        language: Set(None),
        timezone: Set(None),
        display_name: Set(None),
        last_login_at: Set(None),
        last_access_at: Set(None),
        failed_login_count: Set(0),
        locked_until: Set(None),
        is_admin: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
    };
    let result = user::Entity::insert(new_user).exec(dbconn()).await?;
    let user_id = result.last_insert_id;

    assign_role(user_id, crate::AUTHENTICATED_ROLE_ID).await?;

    Ok(user_id)
}

// **< assign_role >********************************************************************************

/// Asigna un rol a un usuario (sin error si ya está asignado).
pub async fn assign_role(user_id: i32, role_id: i32) -> Result<(), AuthError> {
    let already = user_role::Entity::find()
        .filter(user_role::Column::UserId.eq(user_id))
        .filter(user_role::Column::RoleId.eq(role_id))
        .one(dbconn())
        .await?;

    if already.is_none() {
        user_role::Entity::insert(user_role::ActiveModel {
            user_id: Set(user_id),
            role_id: Set(role_id),
        })
        .exec(dbconn())
        .await?;
    }
    Ok(())
}

// **< register_failed_login >**********************************************************************

async fn register_failed_login(
    user_model: &user::Model,
    now: NaiveDateTime,
) -> Result<(), AuthError> {
    let new_count = user_model.failed_login_count + 1;
    let lock_at = if new_count >= SETTINGS.max_failed_logins {
        Some(now + Duration::seconds(SETTINGS.locked_for_secs))
    } else {
        None
    };
    user::ActiveModel {
        id: Set(user_model.id),
        failed_login_count: Set(new_count),
        locked_until: Set(lock_at),
        updated_at: Set(now),
        ..Default::default()
    }
    .update(dbconn())
    .await?;
    Ok(())
}

// **< seed_initial_data >**************************************************************************

/// Crea el usuario administrador inicial si no existe ningún usuario en la base de datos.
///
/// Se llama desde `Extension::initialize()`. Si la tabla está vacía, crea el administrador
/// con las credenciales configuradas en `[user.seed]`. La contraseña se genera aleatoriamente
/// si no está configurada, y se imprime por stdout una sola vez para que el operador la recoja.
pub(crate) async fn seed_initial_data() {
    do_seed().await;
}

async fn do_seed() {
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

    let now = Utc::now().naive_utc();
    let new_admin = user::ActiveModel {
        id: ActiveValue::NotSet,
        username: Set(cfg.admin_username.clone()),
        email: Set(cfg.admin_email.clone()),
        email_verified_at: Set(Some(now)),
        password_hash: Set(hash),
        status: Set(UserStatus::Active.as_i16()),
        language: Set(None),
        timezone: Set(None),
        display_name: Set(Some("Administrator".into())),
        last_login_at: Set(None),
        last_access_at: Set(None),
        failed_login_count: Set(0),
        locked_until: Set(None),
        is_admin: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
    };

    match user::Entity::insert(new_admin).exec(dbconn()).await {
        Ok(result) => {
            if let Err(e) = assign_role(result.last_insert_id, crate::AUTHENTICATED_ROLE_ID).await {
                eprintln!("pagetop-user seed error: {}", e);
            }
            if generated {
                println!(
                    "\npagetop-user: admin account created.\n  username: {}\n  password: {}\n",
                    cfg.admin_username, admin_password
                );
            }
        }
        Err(e) => eprintln!("pagetop-user seed error: {}", e),
    }
}
