use crate::{Lazy, base, locale, trace};
use crate::config::SETTINGS;
use crate::core::{Server, global, server};
use crate::core::theme::register_theme;
use crate::core::module::register_module;

use actix_web::middleware::normalize;

pub fn run(bootstrap: Option<fn()>) -> Result<Server, std::io::Error> {
    // Imprime el rótulo (opcional) de bienvenida.
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

    // Asigna identificador de idioma.
    Lazy::force(&locale::LANGID);

    // Registra los temas predefinidos.
    register_theme(&base::theme::aliner::AlinerTheme);
    register_theme(&base::theme::minimal::MinimalTheme);
    register_theme(&base::theme::bootsier::BootsierTheme);

    // Registra los módulos predeterminados.
    register_module(&base::module::admin::AdminModule);
    register_module(&base::module::user::UserModule);

    // Ejecuta la función de inicio de la aplicación.
    if bootstrap != None {
        trace::debug!("Calling application bootstrap");
        let _ = &(bootstrap.unwrap())();
    }

    // Registra el módulo para la página de inicio de PageTop.
    // Al ser el último, puede sobrecargarse en la función de arranque.
    register_module(&base::module::homepage::HomepageModule);


    // Inicializa el servidor web.
    let server = server::HttpServer::new(|| {
        server::App::new()
            .wrap(tracing_actix_web::TracingLogger)
            .wrap(normalize::NormalizePath::new(normalize::TrailingSlash::Trim))
            .configure(&global::themes)
            .configure(&global::modules)
        })
        .bind(format!("{}:{}",
            &SETTINGS.webserver.bind_address,
            &SETTINGS.webserver.bind_port
        ))?
        .run();
    Ok(server)
}
