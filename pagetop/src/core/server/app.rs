use crate::{Lazy, base, trace};
use crate::config::SETTINGS;
use crate::core::{Server, global, server};
use crate::core::theme::register_theme;
use crate::core::module::register_module;

use std::io::Error;
use actix_web::middleware::normalize::{NormalizePath, TrailingSlash};

pub struct Application {
    server: Server,
}

impl Application {
    pub async fn build(bootstrap: Option<fn()>) -> Result<Self, Error> {
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
        Lazy::force(&server::tracing::TRACING);

        // Valida el identificador de idioma.
        Lazy::force(&server::locale::LANGID);

        // Conecta con la base de datos (opcional).
        #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
        Lazy::force(&server::db::DBCONN);

        // Registra los temas predefinidos.
        register_theme(&base::theme::aliner::AlinerTheme);
        register_theme(&base::theme::minimal::MinimalTheme);
        register_theme(&base::theme::bootsier::BootsierTheme);

        // Registra los módulos predeterminados.
        register_module(&base::module::admin::AdminModule);
        // Registra los módulos que requieren base de datos.
        #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
        register_module(&base::module::user::UserModule);

        // Ejecuta la función de inicio de la aplicación.
        if bootstrap != None {
            trace::info!("Calling application bootstrap.");
            let _ = &(bootstrap.unwrap())();
        }

        // Registra el módulo para la página de inicio de PageTop.
        // Al ser el último, puede sobrecargarse con la función de inicio.
        register_module(&base::module::homepage::HomepageModule);

        // Comprueba actualizaciones pendientes de la base de datos (opcional).
        #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
        global::check_migrations();

        // Prepara el servidor web.
        let server = server::HttpServer::new(move || {
            server::App::new()
                .wrap(tracing_actix_web::TracingLogger)
                .wrap(NormalizePath::new(TrailingSlash::Trim))
                .configure(&global::themes)
                .configure(&global::modules)
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
