use crate::config::SETTINGS;
use crate::core::{Server, all, server};

pub fn run(bootstrap: Option<fn()>) -> Result<Server, std::io::Error> {
    // Ejecuta la función de inicio específica para la aplicación.
    if bootstrap != None {
        let _ = &(bootstrap.unwrap())();
    }

    // Inicializa el servidor web.
    let server = server::HttpServer::new(|| {
        server::App::new()
            .configure(&all::modules)
        })
        .bind(format!("{}:{}",
            &SETTINGS.webserver.bind_address,
            &SETTINGS.webserver.bind_port
        ))?
        .run();
    Ok(server)
}
