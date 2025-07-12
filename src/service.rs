//! Gestión del servidor y servicios web (con [Actix Web](https://docs.rs/actix-web)).

pub use actix_web::body::BoxBody;
pub use actix_web::dev::Server;
pub use actix_web::dev::ServiceFactory as Factory;
pub use actix_web::dev::ServiceRequest as Request;
pub use actix_web::dev::ServiceResponse as Response;
pub use actix_web::{http, rt, web};
pub use actix_web::{App, Error, HttpServer};

#[doc(hidden)]
pub use actix_web::test;

/// Incluye en código un conjunto de recursos previamente preparado con `build.rs`.
///
/// # Formas de uso
///
/// * `include_files!(media)` - Incluye el conjunto de recursos llamado `media`. Normalmente se
///   usará esta forma.
///
/// * `include_files!(BLOG_HM => blog)` - Asigna a la variable estática `BLOG_HM` un conjunto de
///   recursos llamado `blog`.
///
/// # Argumentos
///
/// * `$bundle` – Nombre del conjunto de recursos generado por `build.rs` (consultar
///   [`pagetop_build`](https://docs.rs/pagetop-build)).
/// * `$STATIC` – Identificador para la variable estática de tipo
///   [`StaticResources`](`crate::StaticResources`).
///
/// # Ejemplos
///
/// ```rust,ignore
/// include_files!(assets); // Uso habitual.
///
/// include_files!(STATIC_ASSETS => assets);
/// ```
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
            pub static $STATIC: std::sync::LazyLock<StaticResources> = std::sync::LazyLock::new(
                [<static_files_ $bundle>]::$bundle
            );
        }
    };
}

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
///   [`absolute_dir`](crate::util::absolute_dir)). Si no existe, se usarán los recursos embebidos.
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
/// impl ExtensionTrait for MyExtension {
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
