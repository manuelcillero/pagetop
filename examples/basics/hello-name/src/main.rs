use pagetop::prelude::*;

new_handle!(APP_HELLO_NAME);

struct HelloName;

impl ModuleTrait for HelloName {
    fn handle(&self) -> Handle {
        APP_HELLO_NAME
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        cfg.service(hello_name);
    }
}

#[service::get("/hello/{name}")]
async fn hello_name(
    request: service::HttpRequest,
    path: service::web::Path<String>,
) -> ResultPage<Markup, FatalError> {
    let name = path.into_inner();
    Page::new(request)
        .with_in("content", Html::with(html! { h1 { "Hello " (name) "!" } }))
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloName).unwrap().run()?.await
}
