use pagetop::prelude::*;

use_handle!(APP_HELLO_WORLD);

struct HelloWorld;

impl ModuleTrait for HelloWorld {
    fn handle(&self) -> Handle {
        APP_HELLO_WORLD
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        cfg.route("/", service::web::get().to(hello_world));
    }
}

async fn hello_world(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    Page::new(request)
        .with_in("content", Html::with(html! { h1 { "Hello World!" } }))
        .render()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloWorld).unwrap().run()?.await
}
