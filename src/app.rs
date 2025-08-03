//! Prepara y ejecuta una aplicación creada con `Pagetop`.

mod figfont;

use crate::core::{extension, extension::ExtensionRef};
use crate::{global, locale, service, trace};

use actix_session::config::{BrowserSession, PersistentSession, SessionLifecycle};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;

use substring::Substring;

use std::io::Error;
use std::sync::LazyLock;

/// Punto de entrada de una aplicación `PageTop`.
///
/// No almacena datos, **encapsula** el inicio completo de configuración y puesta en marcha. Para
/// instanciarla se puede usar [`new()`](Application::new) o [`prepare()`](Application::prepare).
/// Después sólo hay que llamar a [`run()`](Application::run) para ejecutar la aplicación (o a
/// [`test()`](Application::test) si se está preparando un entorno de pruebas).
pub struct Application;

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

impl Application {
    /// Crea una instancia de la aplicación.
    pub fn new() -> Self {
        Self::internal_prepare(None)
    }

    /// Prepara una instancia de la aplicación a partir de una extensión raíz.
    ///
    /// Esa extensión suele declarar:
    ///
    /// - Sus propias dependencias (que se habilitarán automáticamente).
    /// - Una lista de extensiones que deben deshabilitarse si estuvieran activadas.
    ///
    /// Esto simplifica el arranque en escenarios complejos.
    pub fn prepare(root_extension: ExtensionRef) -> Self {
        Self::internal_prepare(Some(root_extension))
    }

    // Método interno para preparar la aplicación, opcionalmente con una extensión.
    fn internal_prepare(root_extension: Option<ExtensionRef>) -> Self {
        // Al arrancar muestra una cabecera para la aplicación.
        Self::show_banner();

        // Inicia gestión de trazas y registro de eventos (logging).
        LazyLock::force(&trace::TRACING);

        // Valida el identificador de idioma por defecto.
        LazyLock::force(&locale::DEFAULT_LANGID);

        // Registra las extensiones de la aplicación.
        extension::all::register_extensions(root_extension);

        // Registra las acciones de las extensiones.
        extension::all::register_actions();

        // Inicializa las extensiones.
        extension::all::initialize_extensions();

        Self
    }

    // Muestra una cabecera para la aplicación basada en la configuración.
    fn show_banner() {
        use colored::Colorize;
        use terminal_size::{terminal_size, Width};

        if global::SETTINGS.app.startup_banner.to_lowercase() != "off" {
            // Nombre de la aplicación, ajustado al ancho del terminal si es necesario.
            let mut app_ff = String::new();
            let app_name = &global::SETTINGS.app.name;
            if let Some((Width(term_width), _)) = terminal_size() {
                if term_width >= 80 {
                    let maxlen: usize = ((term_width / 10) - 2).into();
                    let mut app = app_name.substring(0, maxlen).to_owned();
                    if app_name.len() > maxlen {
                        app = format!("{app}...");
                    }
                    if let Some(ff) = figfont::FIGFONT.convert(&app) {
                        app_ff = ff.to_string();
                    }
                }
            }
            if app_ff.is_empty() {
                println!("\n{app_name}");
            } else {
                print!("\n{app_ff}");
            }

            // Descripción de la aplicación.
            if !global::SETTINGS.app.description.is_empty() {
                println!("{}", global::SETTINGS.app.description.cyan());
            };

            // Versión de PageTop.
            println!(
                "{} {}\n",
                "Powered by PageTop".yellow(),
                env!("CARGO_PKG_VERSION").yellow()
            );
        }
    }

    /// Arranca el servidor web de la aplicación.
    ///
    /// Devuelve [`std::io::Error`] si el *socket* no puede enlazarse (por puerto en uso, permisos,
    /// etc.).
    pub fn run(self) -> Result<service::Server, Error> {
        // Genera clave secreta para firmar y verificar cookies.
        let secret_key = service::cookie::Key::generate();

        // Prepara el servidor web.
        Ok(service::HttpServer::new(move || {
            Self::service_app()
                .wrap(tracing_actix_web::TracingLogger::default())
                .wrap(
                    SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                        .session_lifecycle(match global::SETTINGS.server.session_lifetime {
                            0 => SessionLifecycle::BrowserSession(BrowserSession::default()),
                            _ => SessionLifecycle::PersistentSession(
                                PersistentSession::default().session_ttl(
                                    service::cookie::time::Duration::seconds(
                                        global::SETTINGS.server.session_lifetime,
                                    ),
                                ),
                            ),
                        })
                        .build(),
                )
        })
        .bind(format!(
            "{}:{}",
            &global::SETTINGS.server.bind_address,
            &global::SETTINGS.server.bind_port
        ))?
        .run())
    }

    /// Prepara el servidor web de la aplicación para pruebas.
    pub fn test(
        self,
    ) -> service::App<
        impl service::Factory<
            service::Request,
            Config = (),
            Response = service::Response<service::BoxBody>,
            Error = service::Error,
            InitError = (),
        >,
    > {
        Self::service_app()
    }

    // Configura el servicio web de la aplicación.
    fn service_app() -> service::App<
        impl service::Factory<
            service::Request,
            Config = (),
            Response = service::Response<service::BoxBody>,
            Error = service::Error,
            InitError = (),
        >,
    > {
        service::App::new().configure(extension::all::configure_services)
    }
}
