use pagetop::prelude::*;
use pagetop_mdbook::MdBook;

pub_const_handler!(APP_PAGETOP_WEBSITE);

include!(concat!(env!("OUT_DIR"), "/guides_en.rs"));
static GUIDES_EN: LazyStatic<HashMapResources> = LazyStatic::new(bundle_guides_en);

include!(concat!(env!("OUT_DIR"), "/guias_es.rs"));
static GUIAS_ES: LazyStatic<HashMapResources> = LazyStatic::new(bundle_guias_es);

struct PageTopWebSite;

impl ModuleTrait for PageTopWebSite {
    fn handler(&self) -> Handler {
        APP_PAGETOP_WEBSITE
    }

    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![&MdBook, &pagetop::base::module::homepage::DefaultHomePage]
    }

    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {
        MdBook::configure_service_for_common_resources(cfg);
        MdBook::configure_service_for_mdbook(cfg, "/doc/en", &GUIDES_EN);
        MdBook::configure_service_for_mdbook(cfg, "/doc/es", &GUIAS_ES);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&PageTopWebSite).await?.run()?.await
}
