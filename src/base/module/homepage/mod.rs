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
        cfg.service(
            server::web::resource("/")
                .route(server::web::get().to(greet))
        );
        cfg.service(
            server::web::resource("/{name}")
                .route(server::web::get().to(greet_with_param))
        );
    }
}

async fn greet() -> impl server::Responder {
    t("greeting", &args!["name" => config_get!("app.name")])
}

async fn greet_with_param(req: server::HttpRequest) -> server::HttpResponse {
    let name: String = req.match_info().get("name").unwrap_or("World").into();
    let args = args!["name" => name];
    server::HttpResponse::Ok()
        .body(sycamore::render_to_string(|ctx| sycamore::view! { ctx,
            p { (t("greeting", &args)) }
        }))
}
