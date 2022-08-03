use pagetop::prelude::*;

pub_const_handler!(APP_PAGETOP_WEBSITE);

mod mdbook;

struct PageTopWebSite;

impl ModuleTrait for PageTopWebSite {
    fn handler(&self) -> Handler {
        APP_PAGETOP_WEBSITE
    }

    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![
            &mdbook::MdBook,
            &pagetop::base::module::homepage::DefaultHomePage,
        ]
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&PageTopWebSite).await?.run()?.await
}
