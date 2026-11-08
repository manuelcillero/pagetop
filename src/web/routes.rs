use crate::admin::{AdminRegistry, admin_path, strip_admin_path, undeclared_permission};
use crate::auth::{Access, PermissionRegistry, deny_access};
use crate::web::{Endpoint, HttpRequest, IntoResponse, Request, Response, Router, middleware};
use crate::{global, trace};

use axum::extract::State;
use axum::routing::{MethodRouter, Route};
use tower::{Layer, Service};

use std::collections::BTreeMap;
use std::convert::Infallible;

// **< Routes >*************************************************************************************

/// Rutas de una extensión, cada una con el acceso que la habilita.
///
/// Es lo que recibe [`Extension::configure_router()`]. A diferencia del [`Router`] de Axum, no hay
/// forma de montar una ruta sin indicar quién puede acceder a ella ([`Access`]): PageTop lo
/// comprueba antes de llegar al *handler* y responde con [`deny_access()`] a quien no lo cumple.
/// Una ruta [`Access::Public`] no añade ninguna comprobación.
///
/// Las rutas que se enlazan desde otros sitios se declaran mejor con [`Endpoint`] y se montan con
/// [`endpoint()`](Self::endpoint): así, sus enlaces conocen el acceso sin repetirlo.
///
/// Las rutas del panel de administración (bajo [`admin_path`]) se montan con [`admin_route()`], que
/// exige el permiso de la página declarada ([`AdminPage`]) de la que cuelgan: el permiso se declara
/// sólo en la página. Montar una con cualquier otro método, o una que no cuelgue de ninguna página,
/// impide arrancar la aplicación.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # #[derive(Clone, Copy, Debug)]
/// # enum BlogPermission { Publish }
/// # impl Permission for BlogPermission {
/// #     fn key(&self) -> CowStr { "blog:publish".into() }
/// # }
/// # async fn list_posts() -> &'static str { "" }
/// # async fn new_post_form() -> &'static str { "" }
/// # async fn new_post() -> &'static str { "" }
/// pub struct Blog;
///
/// #[async_trait]
/// impl Extension for Blog {
///     fn configure_router(&self, routes: Routes) -> Routes {
///         routes
///             .public_route("/posts", web::get(list_posts))
///             .route(
///                 "/posts/new",
///                 Access::Permission(&BlogPermission::Publish),
///                 web::get(new_post_form).post(new_post),
///             )
///     }
/// }
/// ```
///
/// [`Extension::configure_router()`]: crate::core::extension::Extension::configure_router
/// [`admin_path`]: crate::global::AppSettings::admin_path
/// [`admin_route()`]: Self::admin_route
/// [`AdminPage`]: crate::admin::AdminPage
pub struct Routes {
    router: Router,
    // Rutas del panel de administración mal montadas, para detener el arranque.
    invalid_admin: Vec<String>,
    // Rutas que exigen un permiso que no está en el catálogo, por permiso, para avisar.
    undeclared: BTreeMap<String, Vec<String>>,
}

impl Routes {
    pub(crate) fn new(router: Router) -> Self {
        Routes {
            router,
            invalid_admin: Vec::new(),
            undeclared: BTreeMap::new(),
        }
    }

    // Devuelve el router, o las rutas del panel de administración mal montadas. Antes avisa de los
    // permisos que exigen las rutas sin estar en el catálogo: ninguna extensión de autenticación
    // podría ofrecerlos para concederlos. Los de las rutas del panel los avisa su registro.
    pub(crate) fn into_router(self) -> Result<Router, Vec<String>> {
        for (key, paths) in &self.undeclared {
            trace::warn!("{}", undeclared_permission(key, paths));
        }
        if self.invalid_admin.is_empty() {
            Ok(self.router)
        } else {
            Err(self.invalid_admin)
        }
    }

    /// Añade una ruta con su acceso y sus *handlers* por método (`web::get(...)`,
    /// `web::get(...).post(...)`...).
    ///
    /// Se puede llamar varias veces con la misma ruta y otros métodos, cada vez con su acceso: p.
    /// ej. ver con un permiso y guardar con otro. Las rutas del panel de administración se montan
    /// con [`admin_route()`](Self::admin_route).
    pub fn route(mut self, path: &str, access: Access, method_router: MethodRouter) -> Self {
        self.reject_admin(path, false);
        self.check_declared(path, access);
        self.router = self
            .router
            .route(path, protect_method(method_router, access));
        self
    }

    /// Añade una ruta pública ([`Access::Public`]), a la que puede acceder cualquiera, también un
    /// usuario anónimo.
    ///
    /// Equivale a [`route()`](Self::route) con [`Access::Public`], que sigue sirviendo cuando el
    /// acceso llega en una variable.
    pub fn public_route(self, path: &str, method_router: MethodRouter) -> Self {
        self.route(path, Access::Public, method_router)
    }

    /// Añade una ruta declarada con [`Endpoint`], con sus *handlers* por método.
    ///
    /// Equivale a [`route()`](Self::route) con la ruta y el acceso que declara.
    pub fn endpoint(self, endpoint: impl Endpoint, method_router: MethodRouter) -> Self {
        self.route(&endpoint.path(), endpoint.access(), method_router)
    }

    /// Añade una ruta del panel de administración, relativa a su raíz, con sus *handlers* por
    /// método.
    ///
    /// Exige el permiso de la página declarada ([`AdminPage`]) de la que cuelga: la más cercana que
    /// se encuentra al quitarle tramos por el final, tal como se monta, con sus parámetros
    /// (`"user/users/{id}/edit"` cuelga de `"user/users"`). Si no cuelga de ninguna, la aplicación
    /// no arranca; como las páginas se declaran con rutas fijas, tampoco cuelga una ruta con un
    /// parámetro antes de cualquier página (`"{section}/list"`). Un permiso más concreto lo
    /// comprueba el *handler* (ver [`require_permission()`](crate::auth::require_permission)).
    ///
    /// [`AdminPage`]: crate::admin::AdminPage
    pub fn admin_route(mut self, path: &str, method_router: MethodRouter) -> Self {
        let full_path = admin_path(path);
        let relative = strip_admin_path(&full_path).unwrap_or_default();
        match AdminRegistry::global().nearest_page(relative) {
            Some(page) => {
                let access = Access::Permission(page.permission());
                self.router = self
                    .router
                    .route(&full_path, protect_method(method_router, access));
            }
            None => self.invalid_admin.push(full_path),
        }
        self
    }

    /// Añade un [`Router`] de Axum bajo un prefijo, con el mismo acceso para todas sus rutas.
    ///
    /// Sirve para agrupar rutas con sus propias capas de *middleware* (p. ej. una API con su
    /// autenticación). El prefijo no puede estar dentro del panel de administración ni contener su
    /// raíz.
    pub fn nest(mut self, path: &str, access: Access, router: Router) -> Self {
        self.reject_admin(path, true);
        self.check_declared(path, access);
        self.router = self.router.nest(path, protect(router, access));
        self
    }

    /// Añade un servicio de Tower bajo un prefijo, con el mismo acceso para todo lo que sirva (p.
    /// ej. los recursos de un libro, o una interfaz web de otro *crate*).
    ///
    /// El prefijo no puede estar dentro del panel de administración ni contener su raíz.
    pub fn nest_service<S>(mut self, path: &str, access: Access, service: S) -> Self
    where
        S: Service<Request, Error = Infallible> + Clone + Send + Sync + 'static,
        S::Response: IntoResponse,
        S::Future: Send + 'static,
    {
        self.reject_admin(path, true);
        self.check_declared(path, access);
        let nested = Router::new().nest_service(path, service);
        self.router = self.router.merge(protect(nested, access));
        self
    }

    fn check_declared(&mut self, path: &str, access: Access) {
        if let Access::Permission(perm) = access
            && !PermissionRegistry::global().has(perm)
        {
            self.undeclared
                .entry(perm.key().into_owned())
                .or_default()
                .push(path.to_owned());
        }
    }

    // Las rutas del panel sólo se montan con `admin_route()`. Un prefijo (`nested`) tampoco puede
    // contener la raíz del panel, porque agruparía rutas del panel con otras.
    fn reject_admin(&mut self, path: &str, nested: bool) {
        let inside = strip_admin_path(path).is_some();
        let contains_root = nested
            && global::SETTINGS
                .app
                .admin_path
                .strip_prefix(path.trim_end_matches('/'))
                .is_some_and(|rest| rest.starts_with('/'));
        if inside || contains_root {
            self.invalid_admin.push(path.to_owned());
        }
    }
}

// Con `layer()` y no `route_layer()`, que dejaría sin comprobar el *fallback* propio de las rutas
// (`web::get(...).fallback(...)`) o de un `Router` anidado. Un método no admitido también pasa
// antes por el acceso: quien no lo cumple no llega a saber qué métodos tiene la ruta.
fn protect_method(method_router: MethodRouter, access: Access) -> MethodRouter {
    match access {
        Access::Public => method_router,
        _ => method_router.layer(middleware::from_fn_with_state(access, check_access)),
    }
}

fn protect(router: Router, access: Access) -> Router {
    match access {
        Access::Public => router,
        _ => router.layer(middleware::from_fn_with_state(access, check_access)),
    }
}

// Comprueba el acceso de la ruta antes de llegar a su *handler*.
async fn check_access(
    State(access): State<Access>,
    request: Request,
    next: middleware::Next,
) -> Response {
    let (parts, body) = request.into_parts();
    let http_request = HttpRequest::from_parts(&parts);
    if access.allows(&http_request) {
        next.run(Request::from_parts(parts, body)).await
    } else {
        deny_access(http_request)
    }
}

// **< Layers >*************************************************************************************

/// Capas de *middleware* globales, que envuelven todas las rutas de la aplicación.
///
/// Es lo que recibe [`Extension::configure_middleware()`]. Sólo admite capas: las rutas se montan
/// en [`Extension::configure_router()`], con su acceso ([`Routes`]).
///
/// [`Extension::configure_middleware()`]: crate::core::extension::Extension::configure_middleware
/// [`Extension::configure_router()`]: crate::core::extension::Extension::configure_router
pub struct Layers(Router);

impl Layers {
    pub(crate) fn new(router: Router) -> Self {
        Layers(router)
    }

    pub(crate) fn into_router(self) -> Router {
        self.0
    }

    /// Añade una capa de *middleware* (p. ej. `web::middleware::from_fn(...)`), igual que
    /// [`Router::layer()`].
    pub fn layer<L>(self, layer: L) -> Self
    where
        L: Layer<Route> + Clone + Send + Sync + 'static,
        L::Service: Service<Request> + Clone + Send + Sync + 'static,
        <L::Service as Service<Request>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request>>::Future: Send + 'static,
    {
        Layers(self.0.layer(layer))
    }
}

// **< StaticFiles >********************************************************************************

/// Archivos estáticos de una extensión, siempre públicos.
///
/// Es lo que recibe [`Extension::configure_static_files()`], y sólo se usa con la macro
/// [`serve_static_files!`](crate::serve_static_files): las demás rutas se montan en
/// [`Extension::configure_router()`], con su acceso ([`Routes`]).
///
/// [`Extension::configure_static_files()`]: crate::core::extension::Extension::configure_static_files
/// [`Extension::configure_router()`]: crate::core::extension::Extension::configure_router
pub struct StaticFiles(Router);

impl StaticFiles {
    pub(crate) fn new(router: Router) -> Self {
        StaticFiles(router)
    }

    pub(crate) fn into_router(self) -> Router {
        self.0
    }

    // La usa `serve_static_files!` con los servicios de archivos de PageTop.
    #[doc(hidden)]
    pub fn nest_service<S>(self, path: &str, service: S) -> Self
    where
        S: Service<Request, Error = Infallible> + Clone + Send + Sync + 'static,
        S::Response: IntoResponse,
        S::Future: Send + 'static,
    {
        StaticFiles(self.0.nest_service(path, service))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CowStr;
    use crate::auth::Permission;
    use crate::web::get;

    struct UndeclaredPermission;

    impl Permission for UndeclaredPermission {
        fn key(&self) -> CowStr {
            "test:undeclared".into()
        }
    }

    #[test]
    fn routes_with_permissions_missing_from_the_catalog_are_grouped_by_permission() {
        let undeclared = Access::Permission(&UndeclaredPermission);
        let routes = Routes::new(Router::new())
            .route("/a", undeclared, get(|| async { "" }))
            .route("/b", Access::Public, get(|| async { "" }))
            .route("/c", undeclared, get(|| async { "" }));
        assert_eq!(routes.undeclared.len(), 1);
        assert_eq!(routes.undeclared["test:undeclared"], ["/a", "/c"]);
    }
}
