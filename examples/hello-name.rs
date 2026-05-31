use pagetop::prelude::*;

struct HelloName;

impl Extension for HelloName {
    fn configure_router(&self, router: Router) -> Router {
        router.route("/hello/{name}", web::get(hello_name))
    }
}

async fn hello_name(
    request: HttpRequest,
    web::Path(name): web::Path<String>,
) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(Html::with(move |_| {
            html! {
                h1 style="text-align: center;" { "Hello " (name) "!" }
            }
        }))
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloName).run()?.await
}
