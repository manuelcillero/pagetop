use crate::{Lazy, all, app, trace};
use crate::config::SETTINGS;
use crate::theme::*;
use crate::module::*;

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
        // Imprime un rótulo de presentación (opcional).
        if SETTINGS.app.startup_banner.to_lowercase() != "off" {
            let figfont = figlet_rs::FIGfont::from_content(
                match SETTINGS.app.startup_banner.to_lowercase().as_str() {
                    "slant"    => include_str!("figfonts/slant.flf"),
                    "small"    => include_str!("figfonts/small.flf"),
                    "speed"    => include_str!("figfonts/speed.flf"),
                    "starwars" => include_str!("figfonts/starwars.flf"),
                    _ => {
                        println!(
                            "FIGfont \"{}\" not found for banner. {}. {}.",
                            SETTINGS.app.startup_banner,
                            "Using \"Small\"",
                            "Check the settings file",
                        );
                        include_str!("figfonts/small.flf")
                    }
                }
            ).unwrap();
            println!("\n{} {}\n\n Powered by PageTop {}\n",
                figfont.convert(&SETTINGS.app.name).unwrap(),
                &SETTINGS.app.description,
                env!("CARGO_PKG_VERSION")
            );
        }

        // Inicia registro de trazas y eventos.
        Lazy::force(&app::tracing::TRACING);

        // Valida el identificador de idioma.
        Lazy::force(&app::locale::LANGID);

        // Conecta con la base de datos (opcional).
        #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
        Lazy::force(&app::db::DBCONN);

        // Registra los temas predefinidos.
        register_theme(&aliner::AlinerTheme);
        register_theme(&minimal::MinimalTheme);
        register_theme(&bootsier::BootsierTheme);

        // Ejecuta la función de inicio de la aplicación.
        trace::info!("Calling application bootstrap");
        let _ = &bootstrap();

        // Registra el módulo para la página de inicio de PageTop.
        // Al ser el último, puede sobrecargarse con la función de inicio.
        register_module(&homepage::HomepageModule);

        // Comprueba actualizaciones pendientes de la base de datos (opcional).
        #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
        all::run_migrations();

        // Prepara el servidor web.
        let server = app::HttpServer::new(move || {
            app::App::new()
                .wrap(tracing_actix_web::TracingLogger)
                .wrap(NormalizePath::new(TrailingSlash::Trim))
                .configure(&all::themes)
                .configure(&all::modules)
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
