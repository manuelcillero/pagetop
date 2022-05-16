use crate::{app, concat_string, util};
use crate::config::SETTINGS;
use crate::html::{Favicon, Markup, html};
use crate::core::component::{ComponentTrait, InContext};
use crate::response::page::Page;
use crate::base::component::Chunck;

pub trait BaseTheme {
    fn single_name(&self) -> &'static str;
}

/// Los temas deben implementar este "trait".
pub trait ThemeTrait: BaseTheme + Send + Sync {
    fn handler(&self) -> &'static str;

    fn name(&self) -> String {
        self.single_name().to_owned()
    }

    fn description(&self) -> Option<String> {
        None
    }

    #[allow(unused_variables)]
    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {
    }

    #[allow(unused_variables)]
    fn before_render_page(&self, page: &mut Page) {
        page.context()
            .with_favicon(Some(Favicon::new()
                .with_icon("/theme/favicon.png")
            ));
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

                (page.context().render())
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
        component: &mut dyn ComponentTrait,
        context: &mut InContext
    ) {
    /*
        Cómo usarlo:

        match component.handler() {
            BLOCK_COMPONENT => {
                let block = component_mut::<Block>(component);
                block.alter_title("New title");
            },
            _ => {},
        }
    */
    }

    #[allow(unused_variables)]
    fn render_component(
        &self,
        component: &dyn ComponentTrait,
        context: &mut InContext
    ) -> Option<Markup> {
        None
    /*
        Cómo usarlo:

        match component.handler() {
            BLOCK_COMPONENT => {
                let block = component_ref::<Block>(component);
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

impl<T: ?Sized + ThemeTrait> BaseTheme for T {
    fn single_name(&self) -> &'static str {
        util::single_type_name::<Self>()
    }
}
