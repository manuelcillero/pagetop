use crate::base;
use crate::config::SETTINGS;
use crate::core::{Server, all, register_module, server};

pub fn run(bootstrap: Option<fn()>) -> Result<Server, std::io::Error> {
    // Ejecuta la función de arranque de la aplicación.
    if bootstrap != None {
        let _ = &(bootstrap.unwrap())();
    }

    // Registra el módulo para la página de inicio de PageTop.
    // Al ser el último, puede sobrecargarse en la función de arranque.
    register_module(&base::module::homepage::HomepageModule);

    // Si el arranque ha ido bien imprime un rótulo opcional de bienvenida.
    if SETTINGS.app.startup_banner.to_lowercase() != "off" {
        let figfont = figlet_rs::FIGfont::from_content(
            match SETTINGS.app.startup_banner.to_lowercase().as_str() {
                "slant"    => include_str!("../../../resources/slant.flf"),
                "small"    => include_str!("../../../resources/small.flf"),
                "speed"    => include_str!("../../../resources/speed.flf"),
                "starwars" => include_str!("../../../resources/starwars.flf"),
                _          => include_str!("../../../resources/small.flf")
            }
        ).unwrap();
        println!("\n{} {}\n\n Powered by PageTop {}\n",
            figfont.convert(&SETTINGS.app.name).unwrap(),
            &SETTINGS.app.description,
            env!("CARGO_PKG_VERSION")
        );
    }

    // Inicializa el servidor web.
    let server = server::HttpServer::new(|| {
        server::App::new()
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
