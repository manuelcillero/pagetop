use crate::{base, trace};
use crate::config::SETTINGS;
use crate::core::{Server, all, register_module, server};

use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::layer::SubscriberExt;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_actix_web::TracingLogger;

use actix_web::middleware::normalize;

pub fn run(bootstrap: Option<fn()>) -> Result<Server, std::io::Error> {
    // Inicia la traza de ejecución de la aplicación.
    let env_filter = EnvFilter::try_new(&SETTINGS.log.tracing)
        .unwrap_or(EnvFilter::new("Info"));

    let rolling = SETTINGS.log.rolling.to_lowercase();
    let (non_blocking, _guard) = match rolling.as_str() {
        "stdout" => tracing_appender::non_blocking(
            std::io::stdout()
        ),
        _ => tracing_appender::non_blocking({
            let path = &SETTINGS.log.path;
            let prefix = &SETTINGS.log.prefix;
            match rolling.as_str() {
                "daily" => tracing_appender::rolling::daily(path, prefix),
                "hourly" => tracing_appender::rolling::hourly(path, prefix),
                "minutely" => tracing_appender::rolling::minutely(path, prefix),
                "endless" => tracing_appender::rolling::never(path, prefix),
                _ => {
                    println!(
                        "Rolling value \"{}\" not valid. Using \"daily\"",
                        rolling
                    );
                    tracing_appender::rolling::daily(path, prefix)
                }
            }
        })
    };
    let formatting_layer = BunyanFormattingLayer::new(
        String::from(&SETTINGS.app.name),
        non_blocking
    );

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Unable to setup subscriber!");

    LogTracer::init().expect("Unable to setup log tracer!");

    // Imprime el rótulo (opcional) de bienvenida.
    if SETTINGS.app.startup_banner.to_lowercase() != "off" {
        let figfont = figlet_rs::FIGfont::from_content(
            match SETTINGS.app.startup_banner.to_lowercase().as_str() {
                "slant"    => include_str!("figfonts/slant.flf"),
                "small"    => include_str!("figfonts/small.flf"),
                "speed"    => include_str!("figfonts/speed.flf"),
                "starwars" => include_str!("figfonts/starwars.flf"),
                _ => {
                    trace::warn!(
                        "FIGfont \"{}\" not found for banner. Using \"{}\"",
                        SETTINGS.app.startup_banner, "Small"
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
            .wrap(normalize::NormalizePath::new(normalize::TrailingSlash::Trim))
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
