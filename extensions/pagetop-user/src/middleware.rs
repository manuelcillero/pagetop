//! Middleware Tower para la resolución de sesión de usuario.
//!
//! Se registra globalmente en [`crate::User::configure_middleware`] y se ejecuta en todas las
//! peticiones entrantes antes de que lleguen al handler. Inserta [`pagetop::auth::CurrentUser`] y,
//! si el usuario está autenticado, el [`crate::account::Account`] con sus datos ricos en las
//! extensiones de la petición HTTP.

use pagetop::auth::PermissionRef;
use pagetop::web::middleware::Next;
use pagetop::web::{Request, Response};

use crate::account::Account;
use crate::session;

/// Resuelve la sesión del usuario e inyecta los tipos de identidad en las extensiones de la
/// petición HTTP.
///
/// - Siempre inserta [`pagetop::auth::CurrentUser`] (anónimo o autenticado).
/// - Si hay sesión activa, inserta también el [`Account`] con roles y permisos.
pub(crate) async fn session_middleware(mut req: Request, next: Next) -> Response {
    let (current_user, maybe_account) = session::resolve_session(req.headers()).await;

    req.extensions_mut().insert(current_user);
    if let Some(account) = maybe_account {
        req.extensions_mut().insert(account);
    }

    next.run(req).await
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
