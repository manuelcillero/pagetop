use pagetop::prelude::*;

struct HelloWorld;

#[async_trait]
impl Extension for HelloWorld {
    fn configure_router(&self, routes: Routes) -> Routes {
        routes.public_route("/", web::get(hello_world))
    }
}

async fn hello_world(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(Html::with(|_| {
            html! {
                h1 style="text-align: center;" { "Hello World!" }
            }
        }))
        .render()
        .await
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloWorld).await.run().await
}
