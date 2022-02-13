use crate::prelude::*;

localize!("en-US", "src/base/module/homepage/locales");

pub struct HomepageModule;

impl Module for HomepageModule {
    fn name(&self) -> String {
        l("module_name")
    }

    fn description(&self) -> String {
        l("module_desc")
    }

    fn configure_module(&self, cfg: &mut server::web::ServiceConfig) {
        cfg.service(server::web::resource("/").to(home));
        cfg.service(server::web::resource("/{name}").to(home));
    }
}

async fn home(req: server::HttpRequest) -> server::Result<Markup> {
    let name: String = req.match_info().get("name").unwrap_or("World").into();
    Page::prepare()
        .add_to("content", Chunck::markup(html! {
            h1 { (t("greetings", &args![ "name" => name])) }
        }))
        .render()
}
