use pagetop::prelude::*;

struct HelloName;

impl ExtensionTrait for HelloName {
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/hello/{name}", service::web::get().to(hello_name));
    }
}

async fn hello_name(
    request: HttpRequest,
    path: service::web::Path<String>,
) -> ResultPage<Markup, ErrorPage> {
    let name = path.into_inner();
    Page::new(request)
        .with_component(Html::with(html! { h1 { "Hello " (name) "!" } }))
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloName).run()?.await
}
