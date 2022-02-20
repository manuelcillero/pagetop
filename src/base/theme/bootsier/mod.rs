use crate::prelude::*;

include!(concat!(env!("OUT_DIR"), "/bootsier.rs"));

localize!("en-US", "src/base/theme/bootsier/locales");

pub struct BootsierTheme;

impl Theme for BootsierTheme {
    fn name(&self) -> &str {
        "Bootsier"
    }

    fn configure_theme(&self, cfg: &mut server::web::ServiceConfig) {
        cfg.service(actix_web_static_files::ResourceFiles::new(
            "/bootsier",
            generate()
        ));
    }

    fn before_render_page(&self, page: &mut Page) {
        page.assets()
            .with_favicon(
                Favicon::new()
                    .with_icon("/bootsier/favicon.png")
            )
            .add_stylesheet(
                StyleSheet::source(
                    "/bootsier/css/bootstrap.min.css"
                )
                .with_weight(-99)
            )
            .add_javascript(
                JavaScript::source(
                    "/bootsier/js/bootstrap.bundle.min.js"
                )
                .with_weight(-98)
            )
            .add_jquery();
    }

    fn render_error_page(&self, mut s: server::http::StatusCode) -> server::Result<Markup> {
        let mut description = "e500-description";
        let mut message = "e500-description";
        match s {
            server::http::StatusCode::NOT_FOUND => {
                description = "e404-description";
                message = "e404-message";
            },
            _ => {
                s = server::http::StatusCode::INTERNAL_SERVER_ERROR;
            }
        }
        Page::prepare()
            .with_title(format!("Error {}", s.as_str()).as_str())
            .add_to("content", Chunck::markup(html! {
                div class="jumbotron" {
                    div class="media" {
                        img
                            src="/bootsier/images/caution.png"
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
            }))
            .render()
    }
}
