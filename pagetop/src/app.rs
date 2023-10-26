//! Instancia y ejecuta una aplicación creada con **PageTop**.

mod figfont;

use crate::core::{module, module::ModuleRef};
use crate::html::Markup;
use crate::response::fatal_error::FatalError;
use crate::response::page::ResultPage;
use crate::{concat_string, config, locale, service, trace, LazyStatic};

#[cfg(feature = "database")]
use crate::db;

use actix_session::config::{BrowserSession, PersistentSession, SessionLifecycle};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;

use substring::Substring;

use std::io::Error;

pub struct Application;

impl Application {
    pub fn prepare(app: ModuleRef) -> Result<Self, Error> {
        // On startup.
        show_banner();

        // Inicia registro de trazas y eventos.
        LazyStatic::force(&trace::TRACING);

        // Valida el identificador global de idioma.
        LazyStatic::force(&locale::LANGID);

        #[cfg(feature = "database")]
        // Conecta con la base de datos.
        LazyStatic::force(&db::DBCONN);

        // Registra los módulos de la aplicación.
        module::all::register_modules(app);

        // Registra acciones de los módulos.
        module::all::register_actions();

        // Inicializa los módulos.
        module::all::init_modules();

        #[cfg(feature = "database")]
        // Ejecuta actualizaciones pendientes de la base de datos.
        module::all::run_migrations();

        Ok(Self)
    }

    pub fn run(self) -> Result<service::Server, Error> {
        // Generate cookie key.
        let secret_key = service::cookie::Key::generate();

        // Prepara el servidor web.
        Ok(service::HttpServer::new(move || {
            service_app()
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
        service_app()
    }
}

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
        .configure(module::all::configure_services)
        .default_service(service::web::route().to(service_not_found))
}

async fn service_not_found(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    Err(FatalError::NotFound(request))
}

fn show_banner() {
    if config::SETTINGS.app.startup_banner.to_lowercase() != "off" {
        // Application name.
        let mut app_name = config::SETTINGS.app.name.to_string();
        if let Some((term_width, _)) = term_size::dimensions() {
            if term_width >= 80 {
                let maxlen = (term_width / 10) - 2;
                let mut app = app_name.substring(0, maxlen).to_owned();
                if app_name.len() > maxlen {
                    app = format!("{}...", app);
                }
                app_name = figfont::FIGFONT.convert(&app).unwrap().to_string();
            }
        }
        // Application description.
        let app_description = if !config::SETTINGS.app.description.is_empty() {
            concat_string!("\n", config::SETTINGS.app.description)
        } else {
            "".to_string()
        };
        // Print banner.
        println!(
            "\n{}{}\n\nPowered by PageTop {}\n",
            app_name,
            app_description,
            env!("CARGO_PKG_VERSION")
        );
    }
}
