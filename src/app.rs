//! Prepare and run an application created with **Pagetop**.

mod figfont;

use crate::core::{package, package::PackageRef};
use crate::html::Markup;
use crate::response::page::{ErrorPage, ResultPage};
use crate::service::HttpRequest;
use crate::{config, locale, service, trace};

#[cfg(feature = "database")]
use crate::db;

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
    /// Creates a new application instance without any package.
    pub fn new() -> Self {
        Self::internal_prepare(None)
    }

    /// Prepares an application instance with a specific package.
    pub fn prepare(root_package: PackageRef) -> Self {
        Self::internal_prepare(Some(root_package))
    }

    // Internal method to prepare the application, optionally with a package.
    fn internal_prepare(root_package: Option<PackageRef>) -> Self {
        // On startup, show the application banner.
        Self::show_banner();

        // Starts logging and event tracing.
        LazyLock::force(&trace::TRACING);

        // Validates the default language identifier.
        LazyLock::force(&locale::LANGID_DEFAULT);

        #[cfg(feature = "database")]
        // Connects to the database.
        LazyLock::force(&db::DBCONN);

        // Registers the application's packages.
        package::all::register_packages(root_package);

        // Registers package actions.
        package::all::register_actions();

        #[cfg(feature = "database")]
        // Runs pending database migrations.
        package::all::run_migrations();

        // Initializes the packages.
        package::all::init_packages();

        Self
    }

    // Displays the application banner based on the configuration.
    fn show_banner() {
        if config::SETTINGS.app.startup_banner.to_lowercase() != "off" {
            // Application name, formatted for the terminal width if necessary.
            let mut app_name = config::SETTINGS.app.name.to_string();
            if let Some((term_width, _)) = term_size::dimensions() {
                if term_width >= 80 {
                    let maxlen = (term_width / 10) - 2;
                    let mut app = app_name.substring(0, maxlen).to_owned();
                    if app_name.len() > maxlen {
                        app = format!("{app}...");
                    }
                    if let Some(ff) = figfont::FIGFONT.convert(&app) {
                        app_name = ff.to_string();
                    }
                }
            }
            println!("\n{app_name}");

            // Application description.
            if !config::SETTINGS.app.description.is_empty() {
                println!("{}\n", config::SETTINGS.app.description);
            };

            // PageTop version.
            println!("Powered by PageTop {}\n", env!("CARGO_PKG_VERSION"));
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
                        .session_lifecycle(match config::SETTINGS.server.session_lifetime {
                            0 => SessionLifecycle::BrowserSession(BrowserSession::default()),
                            _ => SessionLifecycle::PersistentSession(
                                PersistentSession::default().session_ttl(
                                    service::cookie::time::Duration::seconds(
                                        config::SETTINGS.server.session_lifetime,
                                    ),
                                ),
                            ),
                        })
                        .build(),
                )
        })
        .bind(format!(
            "{}:{}",
            &config::SETTINGS.server.bind_address,
            &config::SETTINGS.server.bind_port
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
            .configure(package::all::configure_services)
            .default_service(service::web::route().to(service_not_found))
    }
}

async fn service_not_found(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Err(ErrorPage::NotFound(request))
}
