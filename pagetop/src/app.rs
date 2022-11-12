use crate::base::banner;
use crate::core::module::ModuleStaticRef;
use crate::core::{module, theme};
use crate::html::Markup;
use crate::response::page::ResultPage;
use crate::response::FatalError;
use crate::{config, db, locale, server, trace, LazyStatic};

use actix_web::dev::Server;

use std::io::Error;

pub struct Application {
    server: Server,
}

impl Application {
    pub fn prepare(app: ModuleStaticRef) -> Result<Self, Error> {
        // Rótulo de presentación.
        banner::print_on_startup();

        // Inicia registro de trazas y eventos.
        LazyStatic::force(&trace::TRACING);

        // Valida el identificador de idioma.
        LazyStatic::force(&locale::LANGID);

        #[cfg(feature = "database")]
        // Conecta con la base de datos.
        LazyStatic::force(&db::DBCONN);

        // Registra los módulos de la aplicación.
        module::all::register_modules(app);

        // Registra los temas de los módulos.
        module::all::register_themes();

        // Registra acciones de los módulos.
        module::all::register_actions();

        // Inicializa los módulos.
        module::all::init_modules();

        #[cfg(feature = "database")]
        // Ejecuta actualizaciones pendientes de la base de datos.
        module::all::run_migrations();

        // Prepara el servidor web.
        let server = server::HttpServer::new(move || {
            server::App::new()
                .wrap(tracing_actix_web::TracingLogger::default())
                .configure(module::all::configure_services)
                .configure(theme::all::configure_services)
                .default_service(server::web::route().to(service_not_found))
        })
        .bind(format!(
            "{}:{}",
            &config::SETTINGS.server.bind_address,
            &config::SETTINGS.server.bind_port
        ))?
        .run();

        Ok(Self { server })
    }

    pub fn run(self) -> Result<Server, Error> {
        Ok(self.server)
    }
}

async fn service_not_found() -> ResultPage<Markup, FatalError> {
    Err(FatalError::NotFound)
}
