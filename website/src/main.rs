use pagetop::prelude::*;

mod mdbook;

struct PageTopWebSite;

impl AppTrait for PageTopWebSite {
    fn enable_modules(&self) -> Vec<&'static dyn ModuleTrait> {
        vec![&mdbook::MdBook]
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(PageTopWebSite).await?.run()?.await
}
