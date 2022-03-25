use crate::{Lazy, app, base, module, theme, trace};
use crate::config::SETTINGS;

use std::io::Error;
use actix_web::middleware::normalize::{NormalizePath, TrailingSlash};

pub struct Application {
    server: app::Server,
}

pub fn essence() {
    trace::info!("No bootstrap configured");
}

impl Application {
    pub async fn prepare(bootstrap: fn()) -> Result<Self, Error> {
        // Rótulo de presentación.
        app::banner::print_on_startup();

        // Inicia registro de trazas y eventos.
        Lazy::force(&app::tracing::TRACING);

        // Valida el identificador de idioma.
        Lazy::force(&app::locale::LANGID);

        // Conecta con la base de datos (opcional).
        #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
        Lazy::force(&app::db::DBCONN);

        // Registra los temas predeterminados.
        theme::register_theme(&base::theme::aliner::AlinerTheme);
        theme::register_theme(&base::theme::minimal::MinimalTheme);
        theme::register_theme(&base::theme::bootsier::BootsierTheme);

        // Ejecuta la función de inicio de la aplicación.
        trace::info!("Calling application bootstrap");
        let _ = &bootstrap();

        // Registra el módulo de presentación de PageTop.
        // Normalmente se sobrecargará en la función de inicio.
        module::register_module(&base::module::demopage::DemopageModule);

        // Actualizaciones pendientes de la base de datos (opcional).
        #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
        module::all::migrations();

        // Prepara el servidor web.
        let server = app::HttpServer::new(move || {
            app::App::new()
                .wrap(tracing_actix_web::TracingLogger)
                .wrap(NormalizePath::new(TrailingSlash::Trim))
                .configure(&module::all::modules)
                .configure(&theme::all::themes)
            })
            .bind(format!("{}:{}",
                &SETTINGS.webserver.bind_address,
                &SETTINGS.webserver.bind_port
            ))?
            .run();

        Ok(Self { server })
    }

    pub fn run(self) -> Result<app::Server, Error> {
        Ok(self.server)
    }
}
