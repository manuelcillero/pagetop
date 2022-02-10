use crate::core::{server, Server};

async fn greet(req: server::HttpRequest) -> impl server::Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = server::HttpServer::new(|| {
        server::App::new()
            .route("/", server::web::get().to(greet))
            .route("/{name}", server::web::get().to(greet))
        })
        .bind("127.0.0.1:8000")?
        .run();
    Ok(server)
}
