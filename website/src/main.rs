use pagetop::prelude::*;

struct PageTopWebSite;

impl AppTrait for PageTopWebSite {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(PageTopWebSite).await?.run()?.await
}
