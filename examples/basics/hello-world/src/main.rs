use pagetop::prelude::*;

#[derive(BindHandle)]
struct HelloWorld;

impl ModuleTrait for HelloWorld {
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/", service::web::get().to(hello_world));
    }
}

async fn hello_world(request: service::HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Page::new(request)
        .with_component_in("content", Html::with(html! { h1 { "Hello World!" } }))
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloWorld).unwrap().run()?.await
}
