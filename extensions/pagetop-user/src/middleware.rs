//! Middleware para la resolución de sesión de usuario.
//!
//! Se registra globalmente en [`crate::User::configure_middleware`] y se ejecuta en todas las
//! peticiones a rutas de la aplicación antes de que lleguen al handler, excepto en las de archivos
//! estáticos (ver [`pagetop::core::extension::Extension::configure_static_files`]). Inserta
//! [`pagetop::auth::CurrentUser`] y, si el usuario está autenticado, el [`crate::account::Account`]
//! con sus datos ricos en las extensiones de la petición HTTP.

use pagetop::prelude::*;

use crate::account::Account;
use crate::session;

/// Resuelve la sesión del usuario e inyecta los tipos de identidad en las extensiones de la
/// petición HTTP.
///
/// - Siempre inserta [`pagetop::auth::CurrentUser`] (anónimo o autenticado).
/// - Si hay sesión activa, inserta también el [`Account`] con roles y permisos.
/// - Si se ha prorrogado una sesión iniciada con "recordarme", renueva su cookie en la respuesta.
pub(crate) async fn session_middleware(
    mut req: web::Request,
    next: web::middleware::Next,
) -> Response {
    let (current_user, maybe_account, renewed_cookie) =
        session::resolve_session(req.headers()).await;

    req.extensions_mut().insert(current_user);
    if let Some(account) = maybe_account {
        req.extensions_mut().insert(account);
    }

    let mut response = next.run(req).await;

    // No se pisa una cookie que ya fije el handler (p. ej. la de expiración al cerrar sesión).
    if let Some(cookie) = renewed_cookie
        && !response
            .headers()
            .contains_key(web::http::header::SET_COOKIE)
    {
        match web::http::HeaderValue::try_from(cookie) {
            Ok(value) => {
                response
                    .headers_mut()
                    .insert(web::http::header::SET_COOKIE, value);
            }
            Err(_) => trace::warn!("session: invalid renewed cookie, header discarded"),
        }
    }
    response
}

// **< check_rbac_permission >**********************************************************************

/// Handler de la acción [`pagetop::auth::CheckPermission`] para el modelo RBAC de `pagetop-user`.
///
/// Lee el [`Account`] inyectado por el middleware de sesión desde las extensiones de la petición
/// HTTP y concede el permiso si el account lo tiene (un administrador lo tiene concedido siempre,
/// ver [`Account::has_permission`]).
pub(crate) fn check_rbac_permission(
    request: &pagetop::web::HttpRequest,
    perm: PermissionRef,
    granted: &mut bool,
) {
    let Some(account) = request.extension::<Account>() else {
        return;
    };
    if account.has_permission(perm) {
        *granted = true;
    }
}
