//! Essential web framework ([actix-web](https://docs.rs/actix-web)).

pub use actix_session::Session;
pub use actix_web::body::BoxBody;
pub use actix_web::dev::Server;
pub use actix_web::dev::ServiceFactory as Factory;
pub use actix_web::dev::ServiceRequest as Request;
pub use actix_web::dev::ServiceResponse as Response;
pub use actix_web::{cookie, get, http, rt, test, web};
pub use actix_web::{App, Error, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder};

pub use actix_web_files::Files as ActixFiles;
pub use actix_web_static_files::ResourceFiles;

#[macro_export]
macro_rules! static_files {
    ( $bundle:ident ) => {
        $crate::paste! {
            mod [<static_files_ $bundle>] {
                include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
            }
        }
    };
    ( $bundle:ident => $STATIC:ident ) => {
        $crate::paste! {
            mod [<static_files_ $bundle>] {
                include!(concat!(env!("OUT_DIR"), "/", stringify!($bundle), ".rs"));
            }
            static $STATIC: std::sync::LazyLock<HashMapResources> = std::sync::LazyLock::new(
                [<static_files_ $bundle>]::$bundle
            );
        }
    };
}

#[macro_export]
macro_rules! service_for_static_files {
    ( $scfg:ident, $bundle:ident => $path:expr $(, [$root:expr, $relative:expr])? ) => {{
        $crate::paste! {
            let span = $crate::trace::debug_span!("Configuring static files ", path = $path);
            let _ = span.in_scope(|| {
                let mut serve_embedded:bool = true;
                $(
                    if !$root.is_empty() && !$relative.is_empty() {
                        if let Ok(absolute) = $crate::util::absolute_dir($root, $relative) {
                            $scfg.service($crate::service::ActixFiles::new(
                                $path,
                                absolute,
                            ).show_files_listing());
                            serve_embedded = false
                        }
                    }
                )?
                if serve_embedded {
                    $scfg.service($crate::service::ResourceFiles::new(
                        $path,
                        [<static_files_ $bundle>]::$bundle(),
                    ));
                }
            });
        }
    }};
}
