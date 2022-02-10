use crate::core::{server, Server};

async fn greet() -> impl server::Responder {
    "Hello!"
}

async fn greet_with_param(req: server::HttpRequest) -> server::HttpResponse {
    let name: String = req.match_info().get("name").unwrap_or("World").into();
    server::HttpResponse::Ok()
        .body(sycamore::render_to_string(|ctx| sycamore::view! { ctx,
            p { "Hello " (name) "!" }
        }))
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = server::HttpServer::new(|| {
        server::App::new()
            .route("/", server::web::get().to(greet))
            .route("/{name}", server::web::get().to(greet_with_param))
        })
        .bind("127.0.0.1:8000")?
        .run();
    Ok(server)
}
