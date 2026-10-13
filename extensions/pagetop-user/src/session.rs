//! Gestión de sesiones de usuario (creación, carga, prórroga y destrucción).
//!
//! Incluye también los permisos del rol "anonymous", que se aplican a las peticiones sin sesión.

use std::sync::{Arc, PoisonError, RwLock};
use std::time::Instant;

use pagetop::prelude::*;
use pagetop_seaorm::db::query::{Expr, Query};
use pagetop_seaorm::db::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use pagetop_seaorm::db::{Condition, DbErr, Set, dbconn};

use crate::account::{Account, PermissionSet, UserStatus};
use crate::config::SETTINGS;
use crate::entity::{role_permission, session, user, user_role};
use crate::{ANONYMOUS_ROLE_ID, AUTHENTICATED_ROLE_ID};

// **< Generación de session ID >*******************************************************************

/// Genera un session ID de 64 caracteres hex (32 bytes aleatorios vía OsRng).
pub fn generate_sid() -> String {
    use rand_core::{OsRng, RngCore};
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes_to_hex(&bytes)
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push(char::from(HEX[usize::from(b >> 4)]));
        s.push(char::from(HEX[usize::from(b & 0x0f)]));
    }
    s
}

// **< Cookie helpers >*****************************************************************************

/// Construye el valor de la cabecera `Set-Cookie` para la cookie de sesión.
pub fn build_cookie(sid: &str, remember: bool) -> String {
    let mut cookie = util::join!(
        &SETTINGS.session.cookie_name,
        "=",
        sid,
        "; HttpOnly; SameSite=Lax; Path=/"
    );
    if SETTINGS.session.secure_cookie {
        cookie.push_str("; Secure");
    }
    if remember {
        cookie.push_str("; Max-Age=");
        cookie.push_str(&SETTINGS.session.remember_idle_secs.to_string());
    }
    cookie
}

/// Construye la cookie de expiración (Max-Age=0) para borrar la sesión del navegador.
pub fn expiry_cookie() -> String {
    util::join!(
        &SETTINGS.session.cookie_name,
        "=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0"
    )
}

/// Extrae el session ID de las cabeceras HTTP de la petición, si existe y tiene el formato de los
/// que genera [`generate_sid`].
pub fn extract_sid(headers: Option<&web::http::HeaderMap>) -> Option<&str> {
    let name = SETTINGS.session.cookie_name.as_str();
    // Con HTTP/2 el navegador puede repartir las cookies en varias cabeceras `cookie`.
    headers?
        .get_all(web::http::header::COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|cookies| cookies.split(';'))
        .filter_map(|part| part.trim().strip_prefix(name)?.strip_prefix('='))
        .map(str::trim)
        // Descarta valores que no pueden ser un session ID sin llegar a consultar la base de datos.
        .find(|sid| sid.len() == 64 && sid.bytes().all(|b| b.is_ascii_hexdigit()))
}

// **< resolve_session >****************************************************************************

/// Lee la cookie de sesión de las cabeceras y resuelve la terna
/// `(CurrentUser, Option<Account>, Option<String>)` de [`load_user_from_session`].
///
/// Si no hay una cookie de sesión válida, o [`load_user_from_session`] no resuelve la sesión,
/// devuelve `(CurrentUser::anonymous(), None, None)`. Se llama desde el middleware de sesión.
pub async fn resolve_session(
    headers: &web::http::HeaderMap,
) -> (CurrentUser, Option<Account>, Option<String>) {
    let Some(sid) = extract_sid(Some(headers)) else {
        return (CurrentUser::anonymous(), None, None);
    };
    load_user_from_session(sid).await
}

// **< load_user_from_session >*********************************************************************

/// Carga el usuario de una sesión a partir de su session ID y, si ha pasado al menos
/// `write_interval_secs` desde la última vez, prorroga la sesión por haberse usado.
///
/// Devuelve la terna `(CurrentUser, Option<Account>, Option<String>)`, cuyo tercer elemento es la
/// cabecera `Set-Cookie` que renueva la cookie persistente cuando se prorroga una sesión iniciada
/// con "recordarme". Devuelve `(CurrentUser::anonymous(), None, None)` si la sesión no existe o ha
/// expirado, si la cuenta no está activa (p. ej. bloqueada tras iniciar la sesión) o si falla la
/// consulta, que se registra como aviso.
pub async fn load_user_from_session(sid: &str) -> (CurrentUser, Option<Account>, Option<String>) {
    let now = Utc::now();

    // Sesión y usuario en una sola consulta. La sesión debe seguir vigente y el usuario, activo.
    let found = session::Entity::find_by_id(sid)
        .find_also_related(user::Entity)
        .one(dbconn())
        .await;
    let (sess, mut user_model) = match found {
        Ok(Some((sess, Some(user_model)))) => (sess, user_model),
        Ok(_) => return (CurrentUser::anonymous(), None, None),
        Err(err) => {
            trace::warn!("session: failed to load session: {err}");
            return (CurrentUser::anonymous(), None, None);
        }
    };
    if sess.expires_at < now || UserStatus::from_i16(user_model.status) != UserStatus::Active {
        return (CurrentUser::anonymous(), None, None);
    }

    // Un administrador tiene todos los permisos (ver `Account::has_permission`): no se cargan.
    // Si no, se leen en una sola consulta los de sus roles más los del implícito "authenticated",
    // que no se asigna en `user_role` pero se aplica siempre. "anonymous" se excluye aunque figure
    // en `user_role` (p. ej. insertado directamente en la base de datos): sus permisos nunca llegan
    // a una cuenta.
    let permissions = if user_model.is_admin {
        PermissionSet::default()
    } else {
        let keys = role_permission::Entity::find()
            .select_only()
            .column(role_permission::Column::PermissionKey)
            .filter(
                Condition::any()
                    .add(
                        role_permission::Column::RoleId.in_subquery(
                            Query::select()
                                .column(user_role::Column::RoleId)
                                .from(user_role::Entity)
                                .and_where(user_role::Column::UserId.eq(sess.user_id))
                                .and_where(user_role::Column::RoleId.ne(ANONYMOUS_ROLE_ID))
                                .to_owned(),
                        ),
                    )
                    .add(role_permission::Column::RoleId.eq(AUTHENTICATED_ROLE_ID)),
            )
            .into_tuple::<String>()
            .all(dbconn())
            .await;
        match keys {
            Ok(keys) => PermissionSet::new(keys),
            Err(err) => {
                trace::warn!("session: failed to load permissions: {err}");
                return (CurrentUser::anonymous(), None, None);
            }
        }
    };

    let mut renewed_cookie = None;
    if let Some(expires_at) = renewed_expiry(
        sess.last_activity_at,
        now,
        idle_period(sess.remember),
        Duration::seconds(SETTINGS.session.write_interval_secs),
    ) {
        if sess.remember {
            renewed_cookie = Some(build_cookie(sid, true));
        }
        // Un `UPDATE` directo, sin devolver la fila: `ActiveModel::update()` la recupera con
        // `RETURNING` o, en MySQL, con una consulta adicional.
        let result = session::Entity::update_many()
            .col_expr(session::Column::LastActivityAt, Expr::value(now))
            .col_expr(session::Column::ExpiresAt, Expr::value(expires_at))
            .filter(session::Column::Sid.eq(sid))
            .exec(dbconn())
            .await;
        if let Err(err) = result {
            trace::warn!("session: failed to renew expiry: {err}");
        }
    }

    let language = user_model.language.take();
    let timezone = user_model.timezone.take();
    let theme = user_model.theme.take();
    let account = Account::new(user_model, permissions);
    let current_user = CurrentUser::authenticated(account.id(), account.display())
        .with_language(language.as_deref())
        .with_timezone(timezone.as_deref())
        .with_theme(theme.as_deref());

    (current_user, Some(account), renewed_cookie)
}

// Tiempo sin uso tras el que caduca una sesión, según se iniciara o no con "recordarme".
fn idle_period(remember: bool) -> Duration {
    Duration::seconds(if remember {
        SETTINGS.session.remember_idle_secs
    } else {
        SETTINGS.session.idle_secs
    })
}

// Nueva caducidad de una sesión usada en `now`, o `None` si se prorrogó hace menos de `interval`.
fn renewed_expiry(
    last_activity: Option<DateTime<Utc>>,
    now: DateTime<Utc>,
    idle: Duration,
    interval: Duration,
) -> Option<DateTime<Utc>> {
    match last_activity {
        Some(t) if now - t < interval => None,
        _ => Some(now + idle),
    }
}

// **< create_session >*****************************************************************************

/// Crea una nueva sesión en base de datos y devuelve el session ID.
pub async fn create_session(user_id: i32, remember: bool) -> Result<String, DbErr> {
    let sid = generate_sid();
    let now = Utc::now();

    let new_session = session::ActiveModel {
        sid: Set(sid.clone()),
        user_id: Set(user_id),
        data: Set("{}".into()),
        remember: Set(remember),
        last_activity_at: Set(Some(now)),
        expires_at: Set(now + idle_period(remember)),
        created_at: Set(now),
    };
    session::Entity::insert(new_session).exec(dbconn()).await?;
    Ok(sid)
}

/// Destruye la sesión indicada (logout).
pub async fn destroy_session(sid: &str) -> Result<(), DbErr> {
    session::Entity::delete_by_id(sid).exec(dbconn()).await?;
    Ok(())
}

/// Destruye todas las sesiones de un usuario (p. ej. al cambiar contraseña).
pub async fn destroy_user_sessions(user_id: i32) -> Result<(), DbErr> {
    session::Entity::delete_many()
        .filter(session::Column::UserId.eq(user_id))
        .exec(dbconn())
        .await?;
    Ok(())
}

/// Destruye todas las sesiones de un usuario salvo `keep_sid` (p. ej. cuando el propio usuario
/// cambia su contraseña, para no cerrarle la sesión desde la que lo hace).
pub async fn destroy_other_sessions(user_id: i32, keep_sid: &str) -> Result<(), DbErr> {
    session::Entity::delete_many()
        .filter(session::Column::UserId.eq(user_id))
        .filter(session::Column::Sid.ne(keep_sid))
        .exec(dbconn())
        .await?;
    Ok(())
}

// **< anonymous_permissions >**********************************************************************

// Los permisos de "anonymous" son los mismos para todas las peticiones sin sesión y cambian muy
// poco: se guardan en memoria para no consultarlos en cada una. Al cambiarlos se invalidan en esta
// instancia; en las demás (o si se editan directamente en la base de datos) caducan tras
// `ANONYMOUS_TTL`. La generación evita guardar una carga que empezó antes de una invalidación.
const ANONYMOUS_TTL: std::time::Duration = std::time::Duration::from_secs(60);

struct AnonymousCache {
    generation: u64,
    loaded: Option<(Instant, Arc<PermissionSet>)>,
}

static ANONYMOUS_CACHE: RwLock<AnonymousCache> = RwLock::new(AnonymousCache {
    generation: 0,
    loaded: None,
});

/// Devuelve los permisos del rol "anonymous", que se aplican a las peticiones sin sesión.
///
/// Un usuario autenticado no los hereda: recibe los de "authenticated" y los de sus roles. Si la
/// consulta falla, devuelve un conjunto vacío (no concede nada), lo registra como aviso y lo vuelve
/// a intentar en la siguiente petición.
pub(crate) async fn anonymous_permissions() -> Arc<PermissionSet> {
    let generation = {
        let cache = ANONYMOUS_CACHE
            .read()
            .unwrap_or_else(PoisonError::into_inner);
        if let Some((loaded_at, permissions)) = &cache.loaded
            && loaded_at.elapsed() < ANONYMOUS_TTL
        {
            return Arc::clone(permissions);
        }
        cache.generation
    };

    let keys = role_permission::Entity::find()
        .select_only()
        .column(role_permission::Column::PermissionKey)
        .filter(role_permission::Column::RoleId.eq(ANONYMOUS_ROLE_ID))
        .into_tuple::<String>()
        .all(dbconn())
        .await;
    let permissions = match keys {
        Ok(keys) => Arc::new(PermissionSet::new(keys)),
        Err(err) => {
            trace::warn!("session: failed to load anonymous permissions: {err}");
            return Arc::default();
        }
    };

    let mut cache = ANONYMOUS_CACHE
        .write()
        .unwrap_or_else(PoisonError::into_inner);
    if cache.generation == generation {
        cache.loaded = Some((Instant::now(), Arc::clone(&permissions)));
    }
    permissions
}

/// Descarta los permisos de "anonymous" guardados en memoria, para que la siguiente petición sin
/// sesión los vuelva a leer. Se llama al cambiar los permisos de ese rol.
pub(crate) fn invalidate_anonymous_permissions() {
    let mut cache = ANONYMOUS_CACHE
        .write()
        .unwrap_or_else(PoisonError::into_inner);
    cache.generation = cache.generation.wrapping_add(1);
    cache.loaded = None;
}

// **< Tests >**************************************************************************************

#[cfg(test)]
mod tests {
    use super::{
        DateTime, Duration, SETTINGS, Utc, bytes_to_hex, extract_sid, generate_sid, renewed_expiry,
        web,
    };

    const IDLE_SECS: i64 = 7_200;
    const INTERVAL_SECS: i64 = 60;

    fn at(secs: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(1_800_000_000 + secs, 0).unwrap()
    }

    fn renew(last_activity: Option<i64>, now: i64, interval: i64) -> Option<DateTime<Utc>> {
        renewed_expiry(
            last_activity.map(at),
            at(now),
            Duration::seconds(IDLE_SECS),
            Duration::seconds(interval),
        )
    }

    #[test]
    fn renews_session_without_recorded_activity() {
        assert_eq!(renew(None, 0, INTERVAL_SECS), Some(at(IDLE_SECS)));
    }

    #[test]
    fn skips_renewal_within_interval() {
        assert_eq!(renew(Some(0), 30, INTERVAL_SECS), None);
        assert_eq!(renew(Some(0), INTERVAL_SECS - 1, INTERVAL_SECS), None);
        assert_eq!(
            renew(Some(0), INTERVAL_SECS, INTERVAL_SECS),
            Some(at(INTERVAL_SECS + IDLE_SECS))
        );
    }

    #[test]
    fn zero_interval_renews_on_every_request() {
        assert_eq!(renew(Some(0), 0, 0), Some(at(IDLE_SECS)));
    }

    #[test]
    fn expiry_counts_from_last_use_not_from_login() {
        // A session used again after the interval gets a full idle period from that use.
        let used = IDLE_SECS - 10;
        assert_eq!(
            renew(Some(0), used, INTERVAL_SECS),
            Some(at(used + IDLE_SECS))
        );
    }

    #[test]
    fn encodes_bytes_as_lowercase_hex() {
        assert_eq!(bytes_to_hex(&[0x00, 0x0f, 0xa5, 0xff]), "000fa5ff");
    }

    #[test]
    fn generated_sid_has_the_format_extract_sid_accepts() {
        let sid = generate_sid();
        assert_eq!(sid.len(), 64);
        assert!(
            sid.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        );
    }

    const SID: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    fn cookie_headers(values: &[&str]) -> web::http::HeaderMap {
        let mut headers = web::http::HeaderMap::new();
        for value in values {
            let value = web::http::HeaderValue::from_str(value).unwrap();
            headers.append(web::http::header::COOKIE, value);
        }
        headers
    }

    #[test]
    fn extracts_sid_among_other_cookies() {
        let name = &SETTINGS.session.cookie_name;
        let headers = cookie_headers(&[&format!("a=1; {name}={SID}; b=2")]);
        assert_eq!(extract_sid(Some(&headers)), Some(SID));
    }

    #[test]
    fn extracts_sid_from_split_cookie_headers() {
        // HTTP/2 clients may send each cookie in its own header field.
        let name = &SETTINGS.session.cookie_name;
        let headers = cookie_headers(&["a=1", &format!("{name}={SID}")]);
        assert_eq!(extract_sid(Some(&headers)), Some(SID));
    }

    #[test]
    fn skips_cookies_with_a_longer_name_or_invalid_value() {
        let name = &SETTINGS.session.cookie_name;
        let headers = cookie_headers(&[&format!("{name}x={SID}; {name}=bogus")]);
        assert_eq!(extract_sid(Some(&headers)), None);

        let headers = cookie_headers(&[&format!("{name}=bogus; {name}={SID}")]);
        assert_eq!(extract_sid(Some(&headers)), Some(SID));
    }
}
