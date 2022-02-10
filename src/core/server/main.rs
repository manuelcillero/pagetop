use crate::core::{Server, all, server};

pub fn run(bootstrap: Option<fn()>) -> Result<Server, std::io::Error> {
    // Call application bootstrap.
    if bootstrap != None {
        let _ = &(bootstrap.unwrap())();
    }

    let server = server::HttpServer::new(|| {
        server::App::new()
            .configure(&all::modules)
        })
        .bind("127.0.0.1:8000")?
        .run();
    Ok(server)
}
