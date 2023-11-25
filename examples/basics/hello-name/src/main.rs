use pagetop::prelude::*;

#[derive(BindHandle)]
struct HelloName;

impl ModuleTrait for HelloName {
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.service(hello_name);
    }
}

#[service::get("/hello/{name}")]
async fn hello_name(
    request: service::HttpRequest,
    path: service::web::Path<String>,
) -> ResultPage<Markup, ErrorPage> {
    let name = path.into_inner();
    Page::new(request)
        .with_component_in("content", Html::with(html! { h1 { "Hello " (name) "!" } }))
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloName).unwrap().run()?.await
}
