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

/// **Obsoleto desde la versión 0.3.0**: usar [`static_files_service!`](crate::static_files_service)
/// en su lugar.
///
/// Incluye en código un conjunto de recursos previamente preparado con `build.rs`.
///
/// # Formas de uso
///
/// * `include_files!(media)` - Para incluir un conjunto de recursos llamado `media`. Normalmente se
///   usará esta forma.
///
/// * `include_files!(BLOG => media)` - También se puede asignar el conjunto de recursos a una
///   variable global; p.ej. `BLOG`.
///
/// # Argumentos
///
/// * `$bundle` – Nombre del conjunto de recursos generado por `build.rs` (consultar
///   [`pagetop_build`](https://docs.rs/pagetop-build)).
/// * `$STATIC` – Asigna el conjunto de recursos a una variable global de tipo
///   [`StaticResources`](crate::StaticResources).
///
/// # Ejemplos
///
/// ```rust,ignore
/// include_files!(assets); // Uso habitual.
///
/// include_files!(STATIC_ASSETS => assets);
/// ```
#[deprecated(since = "0.3.0", note = "Use `static_files_service!` instead")]
#[macro_export]
macro_rules! include_files {
    // Forma 1: incluye un conjunto de recursos por nombre.
    ( $bundle:ident ) => {
        $crate::util::paste! {
            mod [<static_files_ $bundle>] {
                include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
            }
        }
    };
    // Forma 2: asigna a una variable estática $STATIC un conjunto de recursos.
    ( $STATIC:ident => $bundle:ident ) => {
        $crate::util::paste! {
            mod [<static_files_ $bundle>] {
                include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
            }
            pub static $STATIC: std::sync::LazyLock<$crate::StaticResources> =
                std::sync::LazyLock::new(
                    $crate::StaticResources::new([<static_files_ $bundle>]::$bundle)
                );
        }
    };
}

/// **Obsoleto desde la versión 0.3.0**: usar [`static_files_service!`](crate::static_files_service)
/// en su lugar.
///
/// Configura un servicio web para publicar los recursos embebidos con [`include_files!`].
///
/// El código expandido de la macro decide durante el arranque de la aplicación si debe servir los
/// archivos de los recursos embebidos o directamente desde el sistema de ficheros, si se ha
/// indicado una ruta válida a un directorio de recursos.
///
/// # Argumentos
///
/// * `$scfg` – Instancia de [`ServiceConfig`](crate::service::web::ServiceConfig) donde aplicar la
///   configuración del servicio web.
/// * `$bundle` – Nombre del conjunto de recursos incluido con [`include_files!`].
/// * `$route` – Ruta URL de origen desde la que se servirán los archivos.
/// * `[ $root, $relative ]` *(opcional)* – Directorio raíz y ruta relativa para construir la ruta
///   absoluta donde buscar los archivos en el sistema de ficheros (ver
///   [`absolute_dir()`](crate::util::absolute_dir)). Si no existe, se usarán los recursos
///   embebidos.
///
/// # Ejemplos
///
/// ```rust,ignore
/// use pagetop::prelude::*;
///
/// include_files!(assets);
///
/// pub struct MyExtension;
///
/// impl Extension for MyExtension {
///     fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
///         include_files_service!(scfg, assets => "/public");
///     }
/// }
/// ```
///
/// Y para buscar los recursos en el sistema de ficheros (si existe la ruta absoluta):
///
/// ```rust,ignore
/// include_files_service!(cfg, assets => "/public", ["/var/www", "assets"]);
///
/// // También desde el directorio actual de ejecución.
/// include_files_service!(cfg, assets => "/public", ["", "static"]);
/// ```
#[deprecated(since = "0.3.0", note = "Use `static_files_service!` instead")]
#[macro_export]
macro_rules! include_files_service {
    ( $scfg:ident, $bundle:ident => $route:expr $(, [$root:expr, $relative:expr])? ) => {{
        $crate::util::paste! {
            let span = $crate::trace::debug_span!("Configuring static files ", path = $route);
            let _ = span.in_scope(|| {
                // Determina si se sirven recursos embebidos (`true`) o desde disco (`false`).
                #[allow(unused_mut)]
                let mut serve_embedded:bool = true;
                $(
                    // Si `$root` y `$relative` no están vacíos, se comprueba la ruta absoluta.
                    if !$root.is_empty() && !$relative.is_empty() {
                        if let Ok(absolute) = $crate::util::absolute_dir($root, $relative) {
                            // Servimos directamente desde el sistema de ficheros.
                            $scfg.service($crate::service::ActixFiles::new(
                                $route,
                                absolute,
                            ).show_files_listing());
                            serve_embedded = false
                        }
                    }
                )?
                // Si no se localiza el directorio, se exponen entonces los recursos embebidos.
                if serve_embedded {
                    $scfg.service($crate::service::ResourceFiles::new(
                        $route,
                        [<static_files_ $bundle>]::$bundle(),
                    ));
                }
            });
        }
    }};
}

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
/// * `$scfg` – Instancia de [`ServiceConfig`](crate::service::web::ServiceConfig) donde aplicar la
///   configuración.
/// * `$path` – Ruta al directorio local con los archivos estáticos.
/// * `$bundle` – Nombre del conjunto de recursos que esta macro integra en el binario.
/// * `$route` – Ruta URL base desde la que se servirán los archivos.
///
/// # Ejemplos
///
/// ```rust,ignore
/// use pagetop::prelude::*;
///
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
                $crate::util::paste! {
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
            $crate::util::paste! {
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
