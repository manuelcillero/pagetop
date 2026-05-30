//! Servidor web y rutas de la aplicación (basado en [Axum](https://docs.rs/axum)).
//!
//! Define rutas y manejadores: el [`Router`], las operaciones HTTP ([`get`], [`post`], [`put`],
//! [`delete`], [`patch`]), los extractores ([`Path`], [`Query`]), [`Json`] e [`IntoResponse`], y
//! re-exporta el módulo `http` para tipos de bajo nivel como `StatusCode`, `HeaderName` o `Method`.
//! También incluye servicios para gestionar archivos estáticos como [`ServeDir`] y
//! [`ServeEmbedded`].

use std::collections::HashMap;
use std::convert::Infallible;
use std::task::{Context, Poll};

use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{HeaderMap, Request, Response, StatusCode, Uri};

// Infraestructura del router.
pub use axum::Router;
pub use axum::http;

// Extractores de petición.
pub use axum::extract::{Path, Query};

// Tipos de respuesta.
pub use axum::Json;
pub use axum::response::IntoResponse;

// Verbos HTTP para registrar rutas.
pub use axum::routing::{delete, get, patch, post, put};

// Servicios para archivos estáticos (disco y embebidos).
pub use pagetop_statics::StaticResource;
pub use tower_http::services::ServeDir;

// **< HttpRequest >********************************************************************************

/// Representa una petición HTTP.
///
/// Almacena los datos necesarios para negociar el idioma y renderizar las páginas de error,
/// incluyendo la URI completa y las cabeceras de la petición original.
///
/// Puede declararse directamente como parámetro en un *handler* para pasarlo al
/// [`Context`](crate::core::component::Context) de renderizado y a las variantes de
/// [`ErrorPage`](crate::response::page::ErrorPage):
///
/// ```rust,ignore
/// async fn my_handler(request: HttpRequest) -> ResultPage<Markup, ErrorPage> { ... }
/// ```
#[derive(Clone, Debug)]
pub struct HttpRequest {
    uri: Uri,
    headers: HeaderMap,
}

impl HttpRequest {
    /// Devuelve la URI completa de la petición, incluyendo la *query string* si la hay.
    pub fn uri(&self) -> &str {
        self.uri
            .path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or("/")
    }

    /// Devuelve la ruta (*path*) de la petición, sin la *query string*.
    pub fn path(&self) -> &str {
        self.uri.path()
    }

    /// Devuelve la cadena de consulta (*query string*) de la petición, sin el carácter `?`.
    ///
    /// Devuelve una cadena vacía si la petición no tiene *query string*.
    pub fn query_string(&self) -> &str {
        self.uri.query().unwrap_or("")
    }

    /// Devuelve las cabeceras HTTP de la petición.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }
}

impl<S: Send + Sync> FromRequestParts<S> for HttpRequest {
    type Rejection = Infallible;

    // Implementa el extractor de Axum para poder declarar `HttpRequest` como parámetro.
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(HttpRequest {
            uri: parts.uri.clone(),
            headers: parts.headers.clone(),
        })
    }
}

// **< ServeEmbedded >******************************************************************************

/// Permite servir archivos estáticos embebidos en el binario.
///
/// Creado por la macro [`crate::static_files_service!`] cuando se pide servir recursos embebidos.
/// Los recursos se indexan por ruta relativa sin la barra inicial (p. ej. `"css/style.css"`). Si se
/// solicita la raíz o un directorio, devuelve `index.html` si existe.
///
/// Es [`Clone`] para clonar el servicio por petición, pero internamente comparte el mapa de
/// recursos con un [`Arc`](std::sync::Arc) para evitar copias innecesarias.
#[derive(Clone)]
pub struct ServeEmbedded {
    files: std::sync::Arc<HashMap<&'static str, StaticResource>>,
}

impl ServeEmbedded {
    /// Crea un nuevo servicio a partir del mapa de recursos embebidos generado por `build.rs`.
    pub fn new(files: HashMap<&'static str, StaticResource>) -> Self {
        Self {
            files: std::sync::Arc::new(files),
        }
    }
}

impl tower::Service<Request<Body>> for ServeEmbedded {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        use axum::http::header;

        // Axum elimina el prefijo de montaje: la ruta restante puede o no comenzar con '/'.
        let path = req.uri().path().trim_start_matches('/');

        // Busca la ruta exacta; si es raíz o directorio, intenta index.html.
        let resource = self.files.get(path).or_else(|| {
            if path.is_empty() || path.ends_with('/') {
                self.files.get("index.html")
            } else {
                None
            }
        });

        let response = match resource {
            Some(r) => Response::builder()
                .header(header::CONTENT_TYPE, r.mime_type)
                .body(Body::from(r.data))
                .unwrap(),
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap(),
        };

        std::future::ready(Ok(response))
    }
}

// **< static_files_service! >**********************************************************************

/// Configura un servicio web para publicar archivos estáticos.
///
/// La macro añade rutas al [`Router`] de Axum pasado como primer argumento y ofrece tres modos:
///
/// - **Sistema de ficheros o embebido** (`[$path, $bundle]`): intenta servir desde `$path`; si es
///   vacío, no existe o no es un directorio, usa el conjunto de recursos `$bundle` embebido.
/// - **Sólo embebido** (`[$bundle]`): sirve siempre desde el conjunto de recursos embebido.
/// - **Sólo sistema de ficheros** (`$path`): sin corchetes, sirve únicamente desde disco si existe.
///
/// # Argumentos
///
/// * `$router` — Variable mutable de tipo [`Router`] donde registrar el servicio.
/// * `$path`   — Ruta al directorio local con los archivos estáticos.
/// * `$bundle` — Nombre del conjunto de recursos embebidos generado por `build.rs`.
/// * `$route`  — Ruta URL base desde la que se servirán los archivos.
///
/// # Ejemplos
///
/// ```rust,ignore
/// # use pagetop::prelude::*;
/// pub struct MyExtension;
///
/// impl Extension for MyExtension {
///     fn configure_router(&self, mut router: Router) -> Router {
///         // Forma 1) Sistema de ficheros o embebido.
///         static_files_service!(router, ["/var/www/static", assets] => "/public");
///
///         // Forma 2) Siempre embebido.
///         static_files_service!(router, [assets] => "/public");
///
///         // Forma 3) Sólo sistema de ficheros (no requiere `assets`).
///         static_files_service!(router, "/var/www/static" => "/public");
///
///         router
///     }
/// }
/// ```
#[macro_export]
macro_rules! static_files_service {
    // Forma 1: primero intenta servir desde el sistema de ficheros; si falla, sirve embebido.
    ( $router:ident, [$path:expr, $bundle:ident] => $route:expr $(,)? ) => {{
        let span = $crate::trace::debug_span!(
            "static_files_service",
            mode = "filesystem_or_embedded",
            route = $route,
        );
        let _guard = span.enter();
        let mut served_from_fs = false;
        if !::std::path::Path::new(&$path).as_os_str().is_empty() {
            if let Ok(absolute) = $crate::util::resolve_absolute_dir($path) {
                $router = $router.nest_service($route, $crate::web::ServeDir::new(absolute));
                served_from_fs = true;
            }
        }
        if !served_from_fs {
            $crate::util::paste! {
                mod [<static_files_ $bundle>] {
                    include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
                }
                $router = $router.nest_service(
                    $route,
                    $crate::web::ServeEmbedded::new(
                        [<static_files_ $bundle>]::$bundle(),
                    ),
                );
            }
        }
    }};
    // Forma 2: sirve siempre embebido.
    ( $router:ident, [$bundle:ident] => $route:expr $(,)? ) => {{
        let span = $crate::trace::debug_span!(
            "static_files_service",
            mode = "embedded_only",
            route = $route,
        );
        let _guard = span.enter();
        $crate::util::paste! {
            mod [<static_files_ $bundle>] {
                include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
            }
            $router = $router.nest_service(
                $route,
                $crate::web::ServeEmbedded::new(
                    [<static_files_ $bundle>]::$bundle(),
                ),
            );
        }
    }};
    // Forma 3: intenta servir desde el sistema de ficheros.
    ( $router:ident, $path:expr => $route:expr $(,)? ) => {{
        let span = $crate::trace::debug_span!(
            "static_files_service",
            mode = "filesystem_only",
            route = $route,
        );
        let _guard = span.enter();
        match $crate::util::resolve_absolute_dir($path) {
            Ok(absolute) => {
                $router = $router.nest_service($route, $crate::web::ServeDir::new(absolute));
            }
            Err(e) => {
                $crate::trace::warn!(
                    "Static dir not found or invalid for route `{}`: {:?} ({e})",
                    $route,
                    $path,
                );
            }
        }
    }};
}

// **< Utilidades de test >*************************************************************************

/// Utilidades para escribir pruebas de integración con PageTop sobre Axum.
#[doc(hidden)]
pub mod test {
    use axum::Router;
    use axum::body::Body;
    use axum::http::{Method, Request};
    use axum::response::Response;
    use tower::ServiceExt;

    /// Devuelve el router tal como se recibe, listo para usarse en pruebas de integración.
    pub fn init_router(router: Router) -> Router {
        router
    }

    /// Constructor de peticiones HTTP para pruebas.
    pub struct TestRequest {
        method: Method,
        uri: String,
    }

    impl TestRequest {
        /// Crea una petición GET.
        pub fn get() -> Self {
            Self {
                method: Method::GET,
                uri: "/".to_owned(),
            }
        }

        /// Crea una petición POST.
        pub fn post() -> Self {
            Self {
                method: Method::POST,
                uri: "/".to_owned(),
            }
        }

        /// Establece la URI de la petición.
        pub fn uri(mut self, uri: impl Into<String>) -> Self {
            self.uri = uri.into();
            self
        }

        /// Construye la petición HTTP de Axum (para enviar al router en tests de integración).
        pub fn to_request(self) -> Request<Body> {
            Request::builder()
                .method(self.method)
                .uri(self.uri)
                .body(Body::empty())
                .unwrap()
        }

        /// Construye un [`HttpRequest`](super::HttpRequest) listo para pasarlo a
        /// [`Context::new`](crate::core::component::Context::new) en tests unitarios de componentes.
        pub fn to_http_request(self) -> super::HttpRequest {
            let uri = self.uri.parse().unwrap();
            super::HttpRequest {
                uri,
                headers: axum::http::HeaderMap::new(),
            }
        }
    }

    /// Envía una petición al router y devuelve la respuesta.
    pub async fn send_request(router: &Router, req: Request<Body>) -> Response {
        router.clone().oneshot(req).await.unwrap()
    }
}
