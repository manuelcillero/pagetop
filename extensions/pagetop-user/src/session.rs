//! Gestión de sesiones de usuario (creación, carga, destrucción).

use pagetop::auth::CurrentUser;
use pagetop::datetime::{Duration, Utc};
use pagetop::web::http::{HeaderMap, header};
use pagetop_seaorm::db::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, Set, dbconn,
};

use crate::AUTHENTICATED_ROLE_ID;
use crate::account::{Account, PermissionSet, UserStatus};
use crate::config::SETTINGS;
use crate::entity::{role, role_permission, session, user, user_role};

// **< Generación de session ID >*******************************************************************

/// Genera un session ID de 64 caracteres hex (32 bytes aleatorios via OsRng).
pub fn generate_sid() -> String {
    use rand_core::{OsRng, RngCore};
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes_to_hex(&bytes)
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        write!(s, "{:02x}", b).unwrap();
    }
    s
}

// **< Cookie helpers >*****************************************************************************

/// Construye el valor de la cabecera `Set-Cookie` para la cookie de sesión.
pub fn build_cookie(sid: &str, remember: bool) -> String {
    let mut parts = vec![
        format!("{}={}", SETTINGS.session_cookie_name, sid),
        "HttpOnly".into(),
        "SameSite=Lax".into(),
        "Path=/".into(),
    ];
    if SETTINGS.secure_cookie {
        parts.push("Secure".into());
    }
    if remember {
        parts.push(format!("Max-Age={}", SETTINGS.session_ttl_secs));
    }
    parts.join("; ")
}

/// Construye la cookie de expiración (Max-Age=0) para borrar la sesión del navegador.
pub fn expiry_cookie() -> String {
    format!(
        "{}=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0",
        SETTINGS.session_cookie_name
    )
}

/// Extrae el session ID de las cabeceras HTTP de la petición, si existe.
pub fn extract_sid(headers: Option<&HeaderMap>) -> Option<String> {
    let cookie_str = headers?.get(header::COOKIE)?.to_str().ok()?;

    let name = SETTINGS.session_cookie_name.as_str();
    for part in cookie_str.split(';') {
        let part = part.trim();
        if let Some(value) = part.strip_prefix(name).and_then(|s| s.strip_prefix('=')) {
            return Some(value.trim().to_owned());
        }
    }
    None
}

// **< resolve_session >****************************************************************************

/// Lee la cookie de sesión de las cabeceras y resuelve el par `(CurrentUser, Option<Account>)`.
///
/// Si no hay cookie o la sesión ha expirado, devuelve `(CurrentUser::Anonymous, None)`.
/// Se llama desde el middleware de sesión, que es async.
pub async fn resolve_session(headers: &HeaderMap) -> (CurrentUser, Option<Account>) {
    let Some(sid) = extract_sid(Some(headers)) else {
        return (CurrentUser::Anonymous, None);
    };
    load_user_from_session(&sid).await
}

// **< load_user_from_session >*********************************************************************

/// Carga el par `(CurrentUser, Option<Account>)` a partir de un session ID.
///
/// Devuelve `(CurrentUser::Anonymous, None)` si la sesión no existe o ha expirado.
pub async fn load_user_from_session(sid: &str) -> (CurrentUser, Option<Account>) {
    let now = Utc::now().naive_utc();

    // Buscar sesión activa y no expirada.
    let Ok(Some(sess)) = session::Entity::find_by_id(sid).one(dbconn()).await else {
        return (CurrentUser::Anonymous, None);
    };
    if sess.expires_at < now {
        return (CurrentUser::Anonymous, None);
    }

    // Cargar usuario con estado activo.
    let Ok(Some(user_model)) = user::Entity::find_by_id(sess.user_id).one(dbconn()).await else {
        return (CurrentUser::Anonymous, None);
    };
    if UserStatus::from_i16(user_model.status) != UserStatus::Active {
        return (CurrentUser::Anonymous, None);
    }

    // Cargar roles explícitos del usuario.
    let Ok(user_role_rows) = user_role::Entity::find()
        .filter(user_role::Column::UserId.eq(sess.user_id))
        .all(dbconn())
        .await
    else {
        return (CurrentUser::Anonymous, None);
    };

    let role_ids: Vec<i32> = user_role_rows.iter().map(|ur| ur.role_id).collect();

    let Ok(role_rows) = role::Entity::find()
        .filter(role::Column::Id.is_in(role_ids.clone()))
        .all(dbconn())
        .await
    else {
        return (CurrentUser::Anonymous, None);
    };

    let is_admin = user_model.is_admin;
    let role_names: Vec<String> = role_rows.iter().map(|r| r.machine_name.clone()).collect();

    // Cargar permisos de todos los roles (incluido "authenticated", siempre asignado).
    let mut all_role_ids = role_ids;
    if !all_role_ids.contains(&AUTHENTICATED_ROLE_ID) {
        all_role_ids.push(AUTHENTICATED_ROLE_ID);
    }

    let permissions = if is_admin {
        PermissionSet::default()
    } else {
        let Ok(perm_rows) = role_permission::Entity::find()
            .filter(role_permission::Column::RoleId.is_in(all_role_ids))
            .all(dbconn())
            .await
        else {
            return (CurrentUser::Anonymous, None);
        };
        PermissionSet::new(perm_rows.into_iter().map(|p| p.permission_key))
    };

    // Actualizar last_activity_at con throttle: sólo una vez por minuto.
    let throttle = Duration::minutes(1);
    if sess
        .last_activity_at
        .map(|t| now - t > throttle)
        .unwrap_or(true)
    {
        let mut active: session::ActiveModel = sess.into();
        active.last_activity_at = Set(Some(now));
        let _ = active.update(dbconn()).await;
    }

    let display_name = user_model.display_name.unwrap_or_default();
    let visible_name = if display_name.is_empty() {
        user_model.username.clone()
    } else {
        display_name.clone()
    };
    let account = Account {
        id: user_model.id,
        username: user_model.username,
        email: user_model.email,
        display_name,
        status: UserStatus::from_i16(user_model.status),
        roles: role_names,
        permissions,
        is_admin,
    };
    let current_user = CurrentUser::Authenticated {
        id: account.id,
        display_name: visible_name,
    };

    (current_user, Some(account))
}

// **< create_session >*****************************************************************************

/// Crea una nueva sesión en base de datos y devuelve el session ID.
pub async fn create_session(user_id: i32, remember: bool) -> Result<String, DbErr> {
    let sid = generate_sid();
    let now = Utc::now().naive_utc();
    let ttl = Duration::seconds(SETTINGS.session_ttl_secs);
    let idle = Duration::seconds(SETTINGS.session_idle_ttl_secs);
    let expires_at = if remember { now + ttl } else { now + idle };

    let new_session = session::ActiveModel {
        sid: Set(sid.clone()),
        user_id: Set(user_id),
        data: Set("{}".into()),
        last_activity_at: Set(Some(now)),
        expires_at: Set(expires_at),
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
