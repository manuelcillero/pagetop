//! Gestión del servidor y servicios web (con [Actix Web](https://docs.rs/actix-web)).

pub use actix_session::Session;
pub use actix_web::body::BoxBody;
pub use actix_web::dev::Server;
pub use actix_web::dev::ServiceFactory as Factory;
pub use actix_web::dev::ServiceRequest as Request;
pub use actix_web::dev::ServiceResponse as Response;
pub use actix_web::{cookie, http, rt, web};
pub use actix_web::{App, Error, HttpMessage, HttpRequest, HttpResponse, HttpServer};
pub use actix_web_files::Files as ActixFiles;

pub use pagetop_statics::ResourceFiles;

#[doc(hidden)]
pub use actix_web::test;

#[doc(hidden)]
pub use paste::paste;

/// Configura un servicio web para publicar archivos estáticos.
///
/// La macro ofrece tres modos para configurar el servicio:
///
/// - **Sistema de ficheros o embebido** (`[$path, $bundle]`): trata de servir los archivos desde
///   `$path`; y si es una cadena vacía, no existe o no es un directorio, entonces usará el conjunto
///   de recursos `$bundle` integrado en el binario.
/// - **Sólo embebido** (`[$bundle]`): sirve siempre desde el conjunto de recursos `$bundle`
///   integrado en el binario.
/// - **Sólo sistema de ficheros** (`$path`): sin usar corchetes, sirve únicamente desde el sistema
///   de ficheros si existe; en otro caso no registra el servicio.
///
/// # Argumentos
///
/// * `$scfg` - Instancia de [`ServiceConfig`](crate::service::web::ServiceConfig) donde aplicar la
///   configuración.
/// * `$path` - Ruta al directorio local con los archivos estáticos.
/// * `$bundle` - Nombre del conjunto de recursos que esta macro integra en el binario.
/// * `$route` - Ruta URL base desde la que se servirán los archivos.
///
/// # Ejemplos
///
/// ```rust,ignore
/// # use pagetop::prelude::*;
/// pub struct MyExtension;
///
/// impl Extension for MyExtension {
///     fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
///         // Forma 1) Sistema de ficheros o embebido.
///         static_files_service!(scfg, ["/var/www/static", assets] => "/public");
///
///         // Forma 2) Siempre embebido.
///         static_files_service!(scfg, [assets] => "/public");
///
///         // Forma 3) Sólo sistema de ficheros (no requiere `assets`).
///         static_files_service!(scfg, "/var/www/static" => "/public");
///     }
/// }
/// ```
#[macro_export]
macro_rules! static_files_service {
    // Forma 1: primero intenta servir desde el sistema de ficheros; si falla, sirve embebido.
    ( $scfg:ident, [$path:expr, $bundle:ident] => $route:expr $(,)? ) => {{
        let span = $crate::trace::debug_span!(
            "Configuring static files (file system or embedded)",
            mode = "fs_or_embedded",
            route = $route,
        );
        let _ = span.in_scope(|| {
            let mut serve_embedded: bool = true;
            if !::std::path::Path::new(&$path).as_os_str().is_empty() {
                if let Ok(absolute) = $crate::util::resolve_absolute_dir($path) {
                    $scfg.service($crate::service::ActixFiles::new($route, absolute));
                    serve_embedded = false;
                }
            }
            if serve_embedded {
                $crate::service::paste! {
                    mod [<static_files_ $bundle>] {
                        include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
                    }
                    $scfg.service($crate::service::ResourceFiles::new(
                        $route,
                        [<static_files_ $bundle>]::$bundle(),
                    ));
                }
            }
        });
    }};
    // Forma 2: sirve siempre embebido.
    ( $scfg:ident, [$bundle:ident] => $route:expr $(,)? ) => {{
        let span = $crate::trace::debug_span!(
            "Configuring static files (using embedded only)",
            mode = "embedded",
            route = $route,
        );
        let _ = span.in_scope(|| {
            $crate::service::paste! {
                mod [<static_files_ $bundle>] {
                    include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
                }
                $scfg.service($crate::service::ResourceFiles::new(
                    $route,
                    [<static_files_ $bundle>]::$bundle(),
                ));
            }
        });
    }};
    // Forma 3: intenta servir desde el sistema de ficheros.
    ( $scfg:ident, $path:expr => $route:expr $(,)? ) => {{
        let span = $crate::trace::debug_span!(
            "Configuring static files (file system only)",
            mode = "fs",
            route = $route,
        );
        let _ = span.in_scope(|| match $crate::util::resolve_absolute_dir($path) {
            Ok(absolute) => {
                $scfg.service($crate::service::ActixFiles::new($route, absolute));
            }
            Err(e) => {
                $crate::trace::warn!(
                    "Static dir not found or invalid for route `{}`: {:?} ({e})",
                    $route,
                    $path,
                );
            }
        });
    }};
}
