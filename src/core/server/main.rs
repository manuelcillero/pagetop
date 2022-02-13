use crate::base;
use crate::config::SETTINGS;
use crate::core::{Server, all, register_module, server};

pub fn run(bootstrap: Option<fn()>) -> Result<Server, std::io::Error> {
    // Ejecuta la función de arranque de la aplicación.
    if bootstrap != None {
        let _ = &(bootstrap.unwrap())();
    }

    // Registra la página de inicio de PageTop como último módulo.
    // Así, la función de arranque de la aplicación podría sobrecargarlo.
    register_module(&base::module::homepage::HomepageModule);

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
