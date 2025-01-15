//! Prepara y ejecuta una aplicación creada con `Pagetop`.

mod figfont;

use crate::core::{extension, extension::ExtensionRef};
use crate::html::Markup;
use crate::response::page::{ErrorPage, ResultPage};
use crate::service::HttpRequest;
use crate::{global, locale, service, trace};

use actix_session::config::{BrowserSession, PersistentSession, SessionLifecycle};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;

use substring::Substring;

use std::io::Error;
use std::sync::LazyLock;

pub struct Application;

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

impl Application {
    /// Creates a new application instance without any extension.
    pub fn new() -> Self {
        Self::internal_prepare(None)
    }

    /// Prepares an application instance with a specific extension.
    pub fn prepare(root_extension: ExtensionRef) -> Self {
        Self::internal_prepare(Some(root_extension))
    }

    // Internal method to prepare the application, optionally with a extension.
    fn internal_prepare(root_extension: Option<ExtensionRef>) -> Self {
        // On startup, show the application banner.
        Self::show_banner();

        // Starts logging and event tracing.
        LazyLock::force(&trace::TRACING);

        // Validates the default language identifier.
        LazyLock::force(&locale::DEFAULT_LANGID);

        // Registers the application's extensions.
        extension::all::register_extensions(root_extension);

        // Registers extension actions.
        extension::all::register_actions();

        // Initializes the extensions.
        extension::all::init_extensions();

        Self
    }

    // Displays the application banner based on the configuration.
    fn show_banner() {
        use colored::Colorize;
        use terminal_size::{terminal_size, Width};

        if global::SETTINGS.app.startup_banner.to_lowercase() != "off" {
            // Application name, formatted for the terminal width if necessary.
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

            // Application description.
            if !global::SETTINGS.app.description.is_empty() {
                println!("{}", global::SETTINGS.app.description.cyan());
            };

            // PageTop version.
            println!(
                "{} {}\n",
                "Powered by PageTop".yellow(),
                env!("CARGO_PKG_VERSION").yellow()
            );
        }
    }

    /// Starts the web server.
    pub fn run(self) -> Result<service::Server, Error> {
        // Generate the cookie key.
        let secret_key = service::cookie::Key::generate();

        // Prepares the web server.
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

    /// Method for testing, returns a service application instance.
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

    // Configures the service application.
    fn service_app() -> service::App<
        impl service::Factory<
            service::Request,
            Config = (),
            Response = service::Response<service::BoxBody>,
            Error = service::Error,
            InitError = (),
        >,
    > {
        service::App::new()
            .configure(extension::all::configure_services)
            .default_service(service::web::route().to(service_not_found))
    }
}

async fn service_not_found(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Err(ErrorPage::NotFound(request))
}
