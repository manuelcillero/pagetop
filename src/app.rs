//! Prepare and run an application created with **Pagetop**.

mod figfont;

use crate::core::{package, package::PackageRef};
use crate::html::Markup;
use crate::response::page::{ErrorPage, ResultPage};
use crate::{config, locale, service, trace, LazyStatic};

#[cfg(feature = "database")]
use crate::db;

use actix_session::config::{BrowserSession, PersistentSession, SessionLifecycle};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;

use substring::Substring;

use std::io::Error;

pub struct Application;

impl Application {
    /// Creates a new application instance without any package.
    pub fn new() -> Self {
        Self::internal_prepare(None)
    }

    /// Prepares an application instance with a specific package.
    pub fn prepare(app: PackageRef) -> Self {
        Self::internal_prepare(Some(app))
    }

    // Internal method to prepare the application, optionally with a package.
    fn internal_prepare(app: Option<PackageRef>) -> Self {
        // On startup, show the application banner.
        Self::show_banner();

        // Starts logging and event tracing.
        LazyStatic::force(&trace::TRACING);

        // Validates the global language identifier.
        LazyStatic::force(&locale::LANGID);

        #[cfg(feature = "database")]
        // Connects to the database.
        LazyStatic::force(&db::DBCONN);

        // Registers the application's packages.
        if let Some(app) = app {
            package::all::register_packages(app);
        }

        // Registers package actions.
        package::all::register_actions();

        // Initializes the packages.
        package::all::init_packages();

        #[cfg(feature = "database")]
        // Runs pending database migrations.
        package::all::run_migrations();

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
                        app = format!("{}...", app);
                    }
                    if let Some(ff) = figfont::FIGFONT.convert(&app) {
                        app_name = ff.to_string();
                    }
                }
            }
            println!("\n{}", app_name);

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

async fn service_not_found(request: service::HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Err(ErrorPage::NotFound(request))
}