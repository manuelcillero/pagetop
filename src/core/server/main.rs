use crate::{base, trace};
use crate::config::SETTINGS;
use crate::core::{Server, all, register_module, server};

use tracing::subscriber::set_global_default;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::layer::SubscriberExt;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_actix_web::TracingLogger;

pub fn run(bootstrap: Option<fn()>) -> Result<Server, std::io::Error> {
    // Inicia el seguimiento de la traza de ejecución de la aplicación.
    let env_filter = EnvFilter::try_new(String::from(&SETTINGS.app.tracing))
        .unwrap_or(EnvFilter::new(String::from("Info")));
    let formatting_layer = BunyanFormattingLayer::new(
        String::from(&SETTINGS.app.name),
        std::io::stdout
    );
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");

    // Imprime el rótulo (opcional) de bienvenida.
    if SETTINGS.app.startup_banner.to_lowercase() != "off" {
        let figfont = figlet_rs::FIGfont::from_content(
            match SETTINGS.app.startup_banner.to_lowercase().as_str() {
                "slant"    => include_str!("../../../resources/slant.flf"),
                "small"    => include_str!("../../../resources/small.flf"),
                "speed"    => include_str!("../../../resources/speed.flf"),
                "starwars" => include_str!("../../../resources/starwars.flf"),
                _ => {
                    trace::warn!(
                        ">>> FIGfont \"{}\" not found for banner. Using \"{}\"",
                        SETTINGS.app.startup_banner, "Small"
                    );
                    include_str!("../../../resources/small.flf")
                }
            }
        ).unwrap();
        println!("\n{} {}\n\n Powered by PageTop {}\n",
            figfont.convert(&SETTINGS.app.name).unwrap(),
            &SETTINGS.app.description,
            env!("CARGO_PKG_VERSION")
        );
    }

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
            .wrap(TracingLogger)
            .configure(&all::themes)
            .configure(&all::modules)
        })
        .bind(format!("{}:{}",
            &SETTINGS.webserver.bind_address,
            &SETTINGS.webserver.bind_port
        ))?
        .run();
    Ok(server)
}
