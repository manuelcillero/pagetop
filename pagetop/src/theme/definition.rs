use crate::{app, concat_string};
use crate::config::SETTINGS;
use crate::html::{Markup, html};
use crate::response::page::{Favicon, Page, PageAssets, PageComponent};
use crate::base::component::Chunck;

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
        page.assets()
            .with_favicon(
                Favicon::new()
                    .with_icon("/theme/favicon.png")
            );
    }

    fn render_page_head(&self, page: &mut Page) -> Markup {
        let viewport = "width=device-width, initial-scale=1, shrink-to-fit=no";
        html! {
            head {
                meta charset="utf-8";

                @match page.title() {
                    Some(t) => title {
                        (concat_string!(SETTINGS.app.name, " | ", t))
                    },
                    None => title { (SETTINGS.app.name) }
                }

                @match page.description() {
                    Some(d) => meta name="description" content=(d);,
                    None => {}
                }

                meta http-equiv="X-UA-Compatible" content="IE=edge";
                meta name="viewport" content=(viewport);

                (page.assets().render())
            }
        }
    }

    fn render_page_body(&self, page: &mut Page) -> Markup {
        html! {
            body class=[page.body_classes()] {
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
    fn before_render_component(
        &self,
        component: &mut dyn PageComponent,
        assets: &mut PageAssets
    ) {
    /*
        Cómo usarlo:

        match component.name() {
            "Block" => {
                let block = component.as_mut_any().downcast_mut::<Block>().unwrap();
                block.alter_title("New title");
            },
            _ => {},
        }
    */
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
            "Block" => {
                let block = component.as_any().downcast_ref::<Block>().unwrap();
                match block.template() {
                    "default" => Some(block_default(block)),
                    _ => None,
                }
            },
            _ => None,
        }
    */
    }

    fn render_error_page(&self, s: app::http::StatusCode) -> app::Result<Markup> {
        Page::new()
            .with_title(format!("Error {}", s.as_str()).as_str())
            .add_to("content", Chunck::with(html! {
                div {
                    h1 { (s) }
                }
            }))
            .render()
    }
}
