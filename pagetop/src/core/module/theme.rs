use super::ModuleTrait;

use crate::core::component::{ComponentTrait, HtmlMarkup, RenderContext};
use crate::html::{html, Favicon, Markup};
use crate::response::page::Page;
use crate::{concat_string, config};

pub type ThemeStaticRef = &'static dyn ThemeTrait;

/// Los temas deben implementar este "trait".
pub trait ThemeTrait: ModuleTrait + Send + Sync {
    #[allow(unused_variables)]
    fn before_render_page(&self, page: &mut Page) {
        if page.favicon().is_none() {
            page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")));
        }
    }

    fn render_page_head(&self, page: &mut Page) -> Markup {
        let viewport = "width=device-width, initial-scale=1, shrink-to-fit=no";
        html! {
            head {
                meta charset="utf-8";

                @match page.title().get() {
                    Some(t) => title {
                        (concat_string!(config::SETTINGS.app.name, " | ", t))
                    },
                    None => title { (config::SETTINGS.app.name) }
                }

                @if let Some(d) = page.description().get() {
                    meta name="description" content=(d);
                }

                meta name="viewport" content=(viewport);
                @for (name, content) in page.metadata() {
                    meta name=(name) content=(content) {}
                }

                meta http-equiv="X-UA-Compatible" content="IE=edge";
                @for (property, content) in page.properties() {
                    meta property=(property) content=(content) {}
                }

                @if let Some(f) = page.favicon() {
                    (f.render())
                }

                (page.context().render())
            }
        }
    }

    fn render_page_body(&self, page: &mut Page) -> Markup {
        html! {
            body class=[page.body_classes().get()] {
                @match page.template() {
                    "admin" => {
                        @for region in &["top-menu", "side-menu", "region-content"] {
                            @if let Some(content) = page.render_region(region) {
                                #(region) { (content) }
                            }
                        }
                    },
                    _ => {
                        @for region in &["region-content"] {
                            @if let Some(content) = page.render_region(region) {
                                #(region) { (content) }
                            }
                        }
                    }
                }
            }
        }
    }

    #[rustfmt::skip]
    #[allow(unused_variables)]
    fn before_render_component(
        &self,
        component: &mut dyn ComponentTrait,
        rcx: &mut RenderContext,
    ) {
        /*
            Cómo usarlo:

            match component.handle() {
                BLOCK_COMPONENT => {
                    let block = component_mut::<Block>(component);
                    block.alter_title("New title");
                },
                _ => {},
            }
        */
    }

    #[rustfmt::skip]
    #[allow(unused_variables)]
    fn render_component(
        &self,
        component: &dyn ComponentTrait,
        rcx: &mut RenderContext,
    ) -> Option<Markup> {
        None
        /*
            Cómo usarlo:

            match component.handle() {
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

    fn error_404_not_found(&self) -> HtmlMarkup {
        HtmlMarkup::new().with(html! {
            div {
                h1 { ("RESOURCE NOT FOUND") }
            }
        })
    }

    fn error_403_access_denied(&self) -> HtmlMarkup {
        HtmlMarkup::new().with(html! {
            div {
                h1 { ("FORBIDDEN ACCESS") }
            }
        })
    }
}
