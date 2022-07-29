use crate::prelude::*;

pub_const_handler!(THEME_BOOTSIER);

include!(concat!(env!("OUT_DIR"), "/bootsier.rs"));

localize!("src/base/theme/bootsier/locales");

pub struct Bootsier;

impl ThemeTrait for Bootsier {
    fn handler(&self) -> Handler {
        THEME_BOOTSIER
    }

    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {
        theme_static_files!(cfg, "/bootsier");
    }

    fn before_render_page(&self, page: &mut Page) {
        page
            .alter_context(InContextOp::AddFavicon(
                Favicon::new().with_icon("/theme/favicon.png"),
            ))
            .alter_context(InContextOp::AddStyleSheet(
                StyleSheet::located("/bootsier/css/bootstrap.min.css")
                    .with_version("5.1.3")
                    .with_weight(-99),
            ))
            .alter_context(InContextOp::AddJavaScript(
                JavaScript::located("/bootsier/js/bootstrap.bundle.min.js")
                    .with_version("5.1.3")
                    .with_weight(-99),
            ))
            .alter_context(InContextOp::AddJQuery);
    }

    fn error_404_not_found(&self) -> Container {
        Container::new()
            .with_component(
                Html::with(html! {
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
                })
            )
    }
}
