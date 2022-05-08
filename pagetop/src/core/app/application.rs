use crate::{Lazy, base, trace};
use crate::config::SETTINGS;
use crate::core::{module, theme};
use super::AppTrait;

use std::io::Error;
use actix_web::middleware::normalize::{NormalizePath, TrailingSlash};

pub struct Application {
    server: super::Server,
}

impl Application {
    pub async fn prepare(brrrz: impl AppTrait) -> Result<Self, Error> {
        // Rótulo de presentación.
        super::banner::print_on_startup();

        // Inicia registro de trazas y eventos.
        Lazy::force(&super::tracing::TRACING);

        // Valida el identificador de idioma.
        Lazy::force(&super::locale::LANGID);

        // Conecta con la base de datos (opcional).
        #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
        Lazy::force(&super::db::DBCONN);

        // Registra los temas predeterminados.
        theme::register_themes(vec![
            &base::theme::aliner::Aliner,
            &base::theme::minimal::Minimal,
            &base::theme::bootsier::Bootsier,
            &base::theme::bulmix::Bulmix,
        ]);
        theme::register_themes(brrrz.register_themes());

        // Habilita los módulos predeterminados.
        module::enable_modules(brrrz.enabled_modules());
        // Habilita el módulo de presentación de PageTop.
        // Normalmente se sobrecargará en la función de inicio.
        module::enable_module(&base::module::demopage::Demopage);

        // Registra las acciones de todos los módulos.
        module::all::register_hooks();

        // Ejecuta la función de inicio de la aplicación.
        trace::info!("Calling application bootstrap");
        brrrz.bootstrap();
        /*
        if let UsingBootstrap::Fn(bootstrap) = bootstrap {
            let _ = &bootstrap();
        }*/

        // Actualizaciones pendientes de la base de datos (opcional).
        #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
        module::all::run_migrations();

        // Prepara el servidor web.
        let server = super::HttpServer::new(move || {
            super::App::new()
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

    pub fn run(self) -> Result<super::Server, Error> {
        Ok(self.server)
    }
}
