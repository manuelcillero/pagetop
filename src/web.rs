//! Servidor web y rutas de la aplicación (basado en [Axum](https://docs.rs/axum)).
//!
//! Define rutas y manejadores: el [`Router`], las operaciones HTTP ([`get`], [`post`], [`put`],
//! [`delete`], [`patch`]), los extractores ([`Path`], [`Query`]) e [`IntoResponse`], y re-exporta
//! el módulo `http` para tipos de bajo nivel como `StatusCode`, `HeaderName` o `Method`. También
//! ofrece utilidades para servir archivos estáticos, [`ServeDir`] y [`ServeEmbedded`].

use std::collections::HashMap;
use std::convert::Infallible;
use std::task::{Context, Poll};

use axum::body::Body;
use axum::extract::FromRequestParts;

// Infraestructura del router.
pub use axum::Router;
pub use axum::http;

// Extractores de petición.
pub use axum::extract::{Path, Query};

// Para implementar respuestas.
pub use axum::response::{IntoResponse, Response};

// Operaciones HTTP para registrar rutas.
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
/// async fn my_handler(request: HttpRequest) -> Result<Markup, ErrorPage> { ... }
/// ```
#[derive(Clone, Debug)]
pub struct HttpRequest {
    uri: http::Uri,
    headers: http::HeaderMap,
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
    pub fn headers(&self) -> &http::HeaderMap {
        &self.headers
    }
}

impl<S: Send + Sync> FromRequestParts<S> for HttpRequest {
    type Rejection = Infallible;

    // Implementa el extractor de Axum para poder declarar `HttpRequest` como parámetro.
    async fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(HttpRequest {
            uri: parts.uri.clone(),
            headers: parts.headers.clone(),
        })
    }
}

// **< ServeEmbedded >******************************************************************************

/// Servicio para archivos estáticos embebidos en el binario.
///
/// Creado por la macro [`serve_static_files!`](crate::serve_static_files) en los modos que incluyen
/// recursos embebidos. Estos recursos se identifican por su ruta relativa sin la barra inicial
/// (p. ej. `"css/style.css"`). Si se solicita la raíz o una ruta que termina en `/`, el servicio
/// devuelve el `index.html` raíz si existe; no busca por subdirectorio.
///
/// Implementa [`Clone`] para clonar el servicio por petición, pero internamente comparte el mapa de
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

impl tower::Service<http::Request<Body>> for ServeEmbedded {
    type Response = http::Response<Body>;
    type Error = Infallible;
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request<Body>) -> Self::Future {
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
            Some(r) => http::Response::builder()
                .header(header::CONTENT_TYPE, r.mime_type)
                .body(Body::from(r.data))
                .unwrap(),
            None => http::Response::builder()
                .status(http::StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap(),
        };

        std::future::ready(Ok(response))
    }
}

// **< serve_static_files! >************************************************************************

/// Configura el servidor web para publicar archivos estáticos.
///
/// La macro añade rutas al [`Router`] del primer argumento usando uno de los tres modos posibles:
///
/// - **Sistema de ficheros o embebido** (`[$dir, $bundle]`): intenta servir los archivos desde el
///   directorio `$dir`; si está vacío, no existe o no es un directorio, usa el conjunto de recursos
///   `$bundle` embebido.
/// - **Sólo embebido** (`[$bundle]`): sirve siempre desde el conjunto de recursos embebido en el
///   binario.
/// - **Sólo sistema de ficheros** (`$dir`): sin corchetes, sirve únicamente desde el directorio si
///   existe.
///
/// # Argumentos
///
/// * `$router` - Variable de tipo [`Router`] donde registrar las rutas.
/// * `$dir`    - Ruta al directorio local con los archivos estáticos.
/// * `$bundle` - Nombre del conjunto de recursos embebidos generado por `build.rs`.
/// * `$path`   - Prefijo URL bajo el que se publicarán los archivos.
///
/// # Ejemplos
///
/// ```rust,ignore
/// # use pagetop::prelude::*;
/// pub struct MyExtension;
///
/// impl Extension for MyExtension {
///     fn configure_router(&self, router: Router) -> Router {
///         // Forma 1) Sistema de ficheros o embebido.
///         serve_static_files!(router, ["/var/www/static", assets] => "/public");
///
///         // Forma 2) Siempre embebido.
///         serve_static_files!(router, [assets] => "/public");
///
///         // Forma 3) Sólo sistema de ficheros (no requiere `assets`).
///         serve_static_files!(router, "/var/www/static" => "/public");
///
///         router
///     }
/// }
/// ```
#[macro_export]
macro_rules! serve_static_files {
    // Forma 1: primero intenta servir desde el sistema de ficheros; si falla, sirve embebido.
    ( $router:ident, [$dir:expr, $bundle:ident] => $path:expr $(,)? ) => {
        let $router = {
            let _span = $crate::trace::debug_span!(
                "serve_static_files",
                mode = "filesystem_or_embedded",
                route = $path,
            )
            .entered();
            let mut __r = $router;
            let mut served_from_fs = false;
            if !::std::path::Path::new(&$dir).as_os_str().is_empty() {
                if let Ok(absolute) = $crate::util::resolve_absolute_dir($dir) {
                    __r = __r.nest_service($path, $crate::web::ServeDir::new(absolute));
                    served_from_fs = true;
                }
            }
            if !served_from_fs {
                $crate::util::paste! {
                    mod [<static_files_ $bundle>] {
                        include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
                    }
                    __r = __r.nest_service(
                        $path,
                        $crate::web::ServeEmbedded::new(
                            [<static_files_ $bundle>]::$bundle(),
                        ),
                    );
                }
            }
            __r
        };
    };
    // Forma 2: sirve siempre embebido.
    ( $router:ident, [$bundle:ident] => $path:expr $(,)? ) => {
        let $router = {
            let _span = $crate::trace::debug_span!(
                "serve_static_files",
                mode = "embedded_only",
                route = $path,
            )
            .entered();
            $crate::util::paste! {
                mod [<static_files_ $bundle>] {
                    include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
                }
                $router.nest_service(
                    $path,
                    $crate::web::ServeEmbedded::new(
                        [<static_files_ $bundle>]::$bundle(),
                    ),
                )
            }
        };
    };
    // Forma 3: intenta servir desde el sistema de ficheros.
    ( $router:ident, $dir:expr => $path:expr $(,)? ) => {
        let $router = {
            let _span = $crate::trace::debug_span!(
                "serve_static_files",
                mode = "filesystem_only",
                route = $path,
            )
            .entered();
            match $crate::util::resolve_absolute_dir($dir) {
                Ok(absolute) => $router.nest_service($path, $crate::web::ServeDir::new(absolute)),
                Err(e) => {
                    $crate::trace::warn!(
                        "Static dir not found or invalid for route `{}`: {} ({e})",
                        $path,
                        $dir,
                    );
                    $router
                }
            }
        };
    };
}

// **< Utilidades de test >*************************************************************************

/// Utilidades para escribir pruebas de integración con PageTop sobre Axum.
#[doc(hidden)]
pub mod test {
    use axum::Router;
    use axum::body::Body;
    use axum::http;
    use tower::ServiceExt;

    /// Devuelve el router tal como se recibe, listo para usarse en pruebas de integración.
    pub fn init_router(router: Router) -> Router {
        router
    }

    /// Constructor de peticiones HTTP para pruebas.
    pub struct TestRequest {
        method: http::Method,
        uri: String,
    }

    impl TestRequest {
        /// Crea una petición GET.
        pub fn get() -> Self {
            Self {
                method: http::Method::GET,
                uri: "/".to_owned(),
            }
        }

        /// Crea una petición POST.
        pub fn post() -> Self {
            Self {
                method: http::Method::POST,
                uri: "/".to_owned(),
            }
        }

        /// Establece la URI de la petición.
        pub fn uri(mut self, uri: impl Into<String>) -> Self {
            self.uri = uri.into();
            self
        }

        /// Construye la petición HTTP de Axum (para enviar al router en tests de integración).
        pub fn to_request(self) -> http::Request<Body> {
            http::Request::builder()
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
    pub async fn send_request(router: &Router, req: http::Request<Body>) -> http::Response<Body> {
        router.clone().oneshot(req).await.unwrap()
    }
}
