use crate::{Lazy, base, trace};
use crate::config::SETTINGS;
use crate::core::{module, theme};
use super::AppTrait;

use std::io::Error;
use actix_web::dev::Server;

pub struct Application {
    server: Server,
}

impl Application {
    pub async fn prepare(app: impl AppTrait) -> Result<Self, Error> {
        // Rótulo de presentación.
        super::banner::print_on_startup();

        // Inicia registro de trazas y eventos.
        Lazy::force(&super::tracing::TRACING);

        // Valida el identificador de idioma.
        Lazy::force(&super::locale::LANGID);

        // Conecta con la base de datos (opcional).
        #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
        Lazy::force(&super::db::DBCONN);

        // Habilita los módulos de la aplicación.
        module::all::enable_modules(app.enable_modules());

        // Registra los temas predeterminados.
        theme::all::register_themes(vec![
            &base::theme::aliner::Aliner,
            &base::theme::minimal::Minimal,
            &base::theme::bootsier::Bootsier,
        ]);
        // Registra los temas de la aplicación.
        theme::all::register_themes(app.themes());

        // Registra las acciones de todos los módulos.
        module::all::register_hooks();

        // Ejecuta actualizaciones pendientes de la base de datos (opcional).
        #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
        module::all::run_migrations();

        // Ejecuta la función de inicio de la aplicación.
        trace::info!("Calling application bootstrap");
        app.bootstrap();

        // Prepara el servidor web.
        let server = super::HttpServer::new(move || {
            super::App::new()
                .wrap(tracing_actix_web::TracingLogger::default())
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

    pub fn run(self) -> Result<Server, Error> {
        Ok(self.server)
    }
}
