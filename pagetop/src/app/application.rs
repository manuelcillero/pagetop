use crate::{Lazy, app, base, trace};
use crate::config::SETTINGS;
use crate::api::{module, theme};

use std::io::Error;
use actix_web::middleware::normalize::{NormalizePath, TrailingSlash};

pub struct Application {
    server: app::Server,
}

pub enum UsingBootstrap {Fn(fn()), No}

impl Application {
    pub async fn prepare(bootstrap: UsingBootstrap) -> Result<Self, Error> {
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
        theme::register_theme(&base::theme::aliner::Aliner);
        theme::register_theme(&base::theme::minimal::Minimal);
        theme::register_theme(&base::theme::bootsier::Bootsier);
        theme::register_theme(&base::theme::bulmix::Bulmix);

        // Ejecuta la función de inicio de la aplicación.
        trace::info!("Calling application bootstrap");
        if let UsingBootstrap::Fn(bootstrap) = bootstrap {
            let _ = &bootstrap();
        }

        // Registra el módulo de presentación de PageTop.
        // Normalmente se sobrecargará en la función de inicio.
        module::include_module(&base::module::demopage::Demopage);

        // Registra las acciones de todos los módulos.
        module::all::register_actions();

        // Actualizaciones pendientes de la base de datos (opcional).
        #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
        module::all::run_migrations();

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
