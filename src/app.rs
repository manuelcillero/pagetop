//! Prepara y ejecuta una aplicación creada con `Pagetop`.

mod figfont;

use crate::{global, service};

use substring::Substring;

use std::io::Error;

pub struct Application;

impl Application {
    /// Crea una instancia de la aplicación.
    pub fn new() -> Self {
        // Al arrancar muestra una cabecera para la aplicación.
        Self::show_banner();
        Self
    }

    // Muestra una cabecera para la aplicación basada en la configuración.
    fn show_banner() {
        use colored::Colorize;
        use terminal_size::{terminal_size, Width};

        if global::SETTINGS.app.startup_banner.to_lowercase() != "off" {
            // Nombre de la aplicación, ajustado al ancho del terminal si es necesario.
            let mut app_ff = String::new();
            let app_name = &global::SETTINGS.app.name;
            if let Some((Width(term_width), _)) = terminal_size() {
                if term_width >= 80 {
                    let maxlen: usize = ((term_width / 10) - 2).into();
                    let mut app = app_name.substring(0, maxlen).to_owned();
                    if app_name.len() > maxlen {
                        app = format!("{app}...");
                    }
                    if let Some(ff) = figfont::FIGFONT.convert(&app) {
                        app_ff = ff.to_string();
                    }
                }
            }
            if app_ff.is_empty() {
                println!("\n{app_name}");
            } else {
                print!("\n{app_ff}");
            }

            // Descripción de la aplicación.
            if !global::SETTINGS.app.description.is_empty() {
                println!("{}", global::SETTINGS.app.description.cyan());
            };

            // Versión de PageTop.
            println!(
                "{} {}\n",
                "Powered by PageTop".yellow(),
                env!("CARGO_PKG_VERSION").yellow()
            );
        }
    }

    /// Ejecuta el servidor web de la aplicación.
    pub fn run(self) -> Result<service::Server, Error> {
        // Prepara el servidor web.
        Ok(service::HttpServer::new(move || Self::service_app())
            .bind(format!(
                "{}:{}",
                &global::SETTINGS.server.bind_address,
                &global::SETTINGS.server.bind_port
            ))?
            .run())
    }

    /// Prepara el servidor web de la aplicación para pruebas.
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

    // Configura el servicio web de la aplicación.
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
    }
}
