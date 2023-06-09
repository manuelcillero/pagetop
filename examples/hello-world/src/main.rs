use pagetop::prelude::*;

define_handle!(APP_HELLO_WORLD);

struct HelloWorld;

impl ModuleTrait for HelloWorld {
    fn handle(&self) -> Handle {
        APP_HELLO_WORLD
    }

    fn configure_service(&self, cfg: &mut server::web::ServiceConfig) {
        cfg.route("/", server::web::get().to(hello_world));
    }
}

async fn hello_world(request: server::HttpRequest) -> ResultPage<Markup, FatalError> {
    Page::new(request)
        .with_in("content", L10n::html(html! { h1 { "Hello World!" } }))
        .render()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloWorld).unwrap().run()?.await
}
