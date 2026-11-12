//! Middleware para la resolución de sesión de usuario.
//!
//! Se registra globalmente en [`crate::User::configure_middleware`] y se ejecuta en todas las
//! peticiones a rutas de la aplicación antes de que lleguen al handler, excepto en las de archivos
//! estáticos (ver [`pagetop::core::extension::Extension::configure_static_files`]). Inserta
//! [`pagetop::auth::CurrentUser`] y, si el usuario está autenticado, el [`crate::account::Account`]
//! con sus datos ricos en las extensiones de la petición HTTP; si no, los permisos del rol
//! "anonymous".

use std::sync::Arc;

use pagetop::prelude::*;

use crate::account::{Account, PermissionSet};
use crate::actions::AfterSessionResolved;
use crate::config::SETTINGS;
use crate::session;

// Permisos del rol "anonymous", insertados en las peticiones sin sesión. Un tipo propio para que no
// se confundan con los de un `Account`; `Arc` para compartirlos sin copiarlos en cada petición.
#[derive(Clone)]
struct AnonymousPermissions(Arc<PermissionSet>);

/// Resuelve la sesión del usuario e inyecta los tipos de identidad en las extensiones de la
/// petición HTTP.
///
/// - Siempre inserta [`pagetop::auth::CurrentUser`] (anónimo o autenticado).
/// - Si hay sesión activa, inserta también el [`Account`] con sus permisos; si no, los permisos del
///   rol "anonymous".
/// - Con sesión, despacha [`AfterSessionResolved`]: si una acción responde, la petición no llega al
///   *handler*.
/// - Si se ha prorrogado una sesión iniciada con "recordarme", renueva su cookie en la respuesta.
pub(crate) async fn session_middleware(
    mut req: web::Request,
    next: web::middleware::Next,
) -> Response {
    let (current_user, maybe_account, renewed_cookie) =
        session::resolve_session(req.headers()).await;

    req.extensions_mut().insert(current_user);
    let early = match maybe_account {
        Some(account) => {
            // Antes de insertar la cuenta, para no clonarla: la acción la recibe aparte.
            let early = AfterSessionResolved::dispatch(&mut req, &account);
            req.extensions_mut().insert(account);
            early
        }
        None => {
            let permissions = session::anonymous_permissions().await;
            req.extensions_mut()
                .insert(AnonymousPermissions(permissions));
            None
        }
    };

    let mut response = match early {
        Some(response) => response,
        None => next.run(req).await,
    };

    // No se pisa la cookie de sesión que ya fije el handler (p. ej. la de expiración al cerrar
    // sesión). Las demás (mensajes, estado de un listado) conviven con la renovación: si la
    // impidieran, la cookie caducaría en el navegador antes que la sesión.
    if let Some(cookie) = renewed_cookie
        && !sets_cookie(response.headers(), &SETTINGS.session.cookie_name)
    {
        match web::http::HeaderValue::try_from(cookie) {
            Ok(value) => {
                response
                    .headers_mut()
                    .append(web::http::header::SET_COOKIE, value);
            }
            Err(_) => trace::warn!("session: invalid renewed cookie, header discarded"),
        }
    }
    response
}

// Si la respuesta ya fija (o borra) la cookie `name`.
fn sets_cookie(headers: &web::http::HeaderMap, name: &str) -> bool {
    headers
        .get_all(web::http::header::SET_COOKIE)
        .iter()
        .any(|value| {
            value
                .as_bytes()
                .strip_prefix(name.as_bytes())
                .is_some_and(|rest| rest.starts_with(b"="))
        })
}

// **< check_rbac_permission >**********************************************************************

/// Handler de la acción [`CheckPermission`](pagetop::base::action::auth::CheckPermission) para el
/// modelo RBAC de `pagetop-user`.
///
/// Lee el [`Account`] inyectado por el middleware de sesión desde las extensiones de la petición
/// HTTP y concede el permiso si el account lo tiene (un administrador lo tiene concedido siempre,
/// ver [`Account::has_permission`]). Sin sesión, lo concede si lo tiene el rol "anonymous".
pub(crate) fn check_rbac_permission(
    request: &pagetop::web::HttpRequest,
    perm: PermissionRef,
    granted: &mut bool,
) {
    let has_permission = match request.extension::<Account>() {
        Some(account) => account.has_permission(perm),
        None => request
            .extension::<AnonymousPermissions>()
            .is_some_and(|anonymous| anonymous.0.contains(perm.key().as_ref())),
    };
    if has_permission {
        *granted = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers(cookies: &[&'static str]) -> web::http::HeaderMap {
        let mut headers = web::http::HeaderMap::new();
        for cookie in cookies {
            headers.append(
                web::http::header::SET_COOKIE,
                web::http::HeaderValue::from_static(cookie),
            );
        }
        headers
    }

    #[pagetop::test]
    async fn only_the_session_cookie_itself_blocks_its_renewal() {
        // Other cookies (flash messages, the state of a list) must not stop the renewal.
        let others = headers(&[
            "pagetop_flash=iSaved; Path=/",
            "pagetop_list_0=q=a; Path=/x",
        ]);
        assert!(!sets_cookie(&others, "session"));
        let logout = headers(&["pagetop_flash=iBye; Path=/", "session=; Path=/; Max-Age=0"]);
        assert!(sets_cookie(&logout, "session"));
        // A cookie whose name only starts like the session one is a different cookie.
        assert!(!sets_cookie(&headers(&["session_old=x"]), "session"));
    }
}
