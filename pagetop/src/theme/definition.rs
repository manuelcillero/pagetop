use crate::config::SETTINGS;
use crate::html::{Markup, html};
use crate::response::page::{Page, PageAssets, PageComponent};
use crate::app;
use crate::component::Chunck;

/// Los temas deben implementar este "trait".
pub trait ThemeTrait: Send + Sync {
    fn name(&self) -> &'static str;

    fn fullname(&self) -> String;

    fn description(&self) -> Option<String> {
        None
    }

    #[allow(unused_variables)]
    fn configure_theme(&self, cfg: &mut app::web::ServiceConfig) {
    }

    #[allow(unused_variables)]
    fn before_render_page(&self, page: &mut Page) {
    }

    fn render_page_head(&self, page: &mut Page) -> Markup {
        let title = page.title();
        let title = if title.is_empty() {
            SETTINGS.app.name.to_owned()
        } else {
            [SETTINGS.app.name.to_string(), title.to_string()].join(" | ")
        };
        let description = page.description();
        let viewport = "width=device-width, initial-scale=1, shrink-to-fit=no";
        html! {
            head {
                meta charset="utf-8";

                title { (title) }

                @if !description.is_empty() {
                    meta name="description" content=(description);
                }

                meta http-equiv="X-UA-Compatible" content="IE=edge";
                meta name="viewport" content=(viewport);

                (page.assets().render())
            }
        }
    }

    fn render_page_body(&self, page: &mut Page) -> Markup {
        html! {
            body class=(page.body_classes()) {
                @match page.template() {
                    "admin" => {
                        @for region in &["top-menu", "side-menu", "content"] {
                            #(region) {
                                (page.render_region(region))
                            }
                        }
                    },
                    _ => {
                        #content {
                            (page.render_region("content"))
                        }
                    }
                }
            }
        }
    }

    #[allow(unused_variables)]
    fn render_component(
        &self,
        component: &dyn PageComponent,
        assets: &mut PageAssets
    ) -> Option<Markup> {
        None
    /*
        Cómo usarlo:

        match component.name() {
            "block" => {
                let block = component.downcast_mut::<Block>().unwrap();
                match block.template() {
                    "default" => Some(block_default(block)),
                    _ => None
                }
            },
            _ => None
        }
    */
    }

    fn render_error_page(&self, s: app::http::StatusCode) -> app::Result<Markup> {
        Page::prepare()
            .with_title(format!("Error {}", s.as_str()).as_str())
            .add_to("content", Chunck::markup(html! {
                div {
                    h1 { (s) }
                }
            }))
            .render()
    }
}
