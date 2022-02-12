use pagetop::config_get;
use pagetop::core::module::Module;
use pagetop::core::{register_module, server};

struct Greet;
impl Module for Greet {
    fn name(&self) -> String {
        "Hello".to_string()
    }

    fn configure_module(&self, cfg: &mut server::web::ServiceConfig) {
        cfg.service(
            server::web::resource("/")
                .route(server::web::get().to(greet))
        );
    }
}

async fn greet() -> impl server::Responder {
    format!("Hello from {}!", config_get!("app.name"))
}

struct GreetWithParam;
impl Module for GreetWithParam {
    fn name(&self) -> String {
        "Hello World!".to_string()
    }

    fn configure_module(&self, cfg: &mut server::web::ServiceConfig) {
        cfg.service(
            server::web::resource("/{name}")
                .route(server::web::get().to(greet_with_param))
        );
    }
}

async fn greet_with_param(req: server::HttpRequest) -> server::HttpResponse {
    let name: String = req.match_info().get("name").unwrap_or("World").into();
    server::HttpResponse::Ok()
        .body(sycamore::render_to_string(|ctx| sycamore::view! { ctx,
            p { "Hello " (name) "!" }
        }))
}

fn bootstrap() {
    register_module(&Greet);
    register_module(&GreetWithParam);
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    server::run(Some(bootstrap))?.await
}
