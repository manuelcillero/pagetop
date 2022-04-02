use crate::prelude::*;

include!(concat!(env!("OUT_DIR"), "/bootsier.rs"));

localize!("src/base/theme/bootsier/locales");

pub struct BootsierTheme;

impl ThemeTrait for BootsierTheme {
    fn name(&self) -> &'static str {
        "Bootsier"
    }

    fn fullname(&self) -> String {
        "Bootsier".to_owned()
    }

    fn configure_theme(&self, cfg: &mut app::web::ServiceConfig) {
        theme_static_files!(cfg, "/bootsier");
    }

    fn before_render_page(&self, page: &mut Page) {
        page.assets()
            .with_favicon(
                Favicon::new()
                    .with_icon("/theme/favicon.png")
            )
            .add_stylesheet(
                StyleSheet::source(
                    "/bootsier/css/bootstrap.min.css?ver=5.1.3"
                )
                .with_weight(-99)
            )
            .add_javascript(
                JavaScript::source(
                    "/bootsier/js/bootstrap.bundle.min.js?ver=5.1.3"
                )
                .with_weight(-99)
            )
            .add_jquery();
    }

    fn render_error_page(&self, mut s: app::http::StatusCode) -> app::Result<Markup> {
        let mut description = "e500-description";
        let mut message = "e500-description";
        match s {
            app::http::StatusCode::NOT_FOUND => {
                description = "e404-description";
                message = "e404-message";
            },
            _ => {
                s = app::http::StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
        Page::new()
            .with_title(format!("Error {}", s.as_str()).as_str())
            .add_to("content", Chunck::with(html! {
                div class="jumbotron" {
                    div class="media" {
                        img
                            src="/static/bootsier/images/caution.png"
                            class="mr-4"
                            style="width: 20%; max-width: 188px"
                            alt="Caution!";
                        div class="media-body" {
                            h1 class="display-4" { (s) }
                            p class="lead" { (l(description)) }
                            hr class="my-4";
                            p { (l(message)) }
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
            }).arc())
            .render()
    }
}
