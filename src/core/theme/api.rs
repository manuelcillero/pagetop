use crate::core::server;
use crate::core::theme::{Markup, html};
use crate::core::response::page::{Page, PageAssets, PageComponent};

/// Los temas deben implementar este "trait".
pub trait Theme: Send + Sync {
    fn name(&self) -> &str;

    fn description(&self) -> &str {
        ""
    }

    #[allow(unused_variables)]
    fn configure_theme(&self, cfg: &mut server::web::ServiceConfig) {
    }

    #[allow(unused_variables)]
    fn before_render_page(&self, page: &mut Page) {
    }

    fn render_page_head(&self, page: &mut Page) -> Markup {
        let viewport = "width=device-width, initial-scale=1, shrink-to-fit=no";
        let description = page.description();
        html! {
            head {
                meta charset="utf-8";

                meta http-equiv="X-UA-Compatible" content="IE=edge";
                meta name="viewport" content=(viewport);
                @if !description.is_empty() {
                    meta name="description" content=(description);
                }

                title { (page.title()) }

                (page.assets().render())
            }
        }
    }

    fn render_page_body(&self, page: &mut Page) -> Markup {
        html! {
            body id="body" class=(page.body_classes()) {
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

        match component.type_name() {
            "Block" => {
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
}
