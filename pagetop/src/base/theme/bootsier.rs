use crate::prelude::*;

pub_handle!(THEME_BOOTSIER);

pub_locale!("src/base/theme/bootsier/locales");

include!(concat!(env!("OUT_DIR"), "/bootsier.rs"));

pub struct Bootsier;

impl ThemeTrait for Bootsier {
    fn handle(&self) -> Handle {
        THEME_BOOTSIER
    }

    fn configure_service(&self, cfg: &mut server::web::ServiceConfig) {
        configure_service_for_static_files!(cfg, "/bootsier", bundle_bootsier);
    }

    fn before_render_page(&self, page: &mut Page) {
        page.alter_context(PageOp::AddFavicon(
            Favicon::new().with_icon("/theme/favicon.ico"),
        ))
        .alter_context(PageOp::AddStyleSheet(
            StyleSheet::located("/bootsier/css/bootstrap.min.css")
                .with_version("5.1.3")
                .with_weight(-99),
        ))
        .alter_context(PageOp::AddJavaScript(
            JavaScript::located("/bootsier/js/bootstrap.bundle.min.js")
                .with_version("5.1.3")
                .with_weight(-99),
        ))
        .alter_context(PageOp::AddJQuery);
    }

    fn error_404_not_found(&self) -> Container {
        Container::new().with_component(Html::with(html! {
            div class="jumbotron" {
                div class="media" {
                    img
                        src="/bootsier/images/caution.png"
                        class="mr-4"
                        style="width: 20%; max-width: 188px"
                        alt="Caution!";
                    div class="media-body" {
                        h1 class="display-4" { ("RESOURCE NOT FOUND") }
                        p class="lead" { (l("e404-description")) }
                        hr class="my-4";
                        p { (l("e404-description")) }
                        a
                            class="btn btn-primary btn-lg"
                            href="/"
                            role="button"
                        {
                            (l("back-homepage"))
                        }
                    }
                }
            }
        }))
    }
}
