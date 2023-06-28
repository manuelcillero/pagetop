use crate::base::components::L10n;
use crate::core::component::{ComponentTrait, Context};
use crate::core::module::ModuleTrait;
use crate::html::{html, Favicon, Markup};
use crate::response::page::Page;
use crate::{config, LOCALE_PAGETOP};

pub type ThemeStaticRef = &'static dyn ThemeTrait;

/// Los temas deben implementar este "trait".
pub trait ThemeTrait: ModuleTrait + Send + Sync {
    #[rustfmt::skip]
    fn regions(&self) -> Vec<(&'static str, L10n)> {
        vec![
            ("header",  L10n::t("header",  &LOCALE_PAGETOP)),
            ("pagetop", L10n::t("pagetop", &LOCALE_PAGETOP)),
            ("content", L10n::t("content", &LOCALE_PAGETOP)),
            ("sidebar", L10n::t("sidebar", &LOCALE_PAGETOP)),
            ("footer",  L10n::t("footer",  &LOCALE_PAGETOP)),
        ]
    }

    #[allow(unused_variables)]
    fn before_prepare_page(&self, page: &mut Page) {}

    fn prepare_page_head(&self, page: &mut Page) -> Markup {
        let title = page.title();
        let description = page.description();
        let viewport = "width=device-width, initial-scale=1, shrink-to-fit=no";
        html! {
            head {
                meta charset="utf-8";

                @if !title.is_empty() {
                    title { (config::SETTINGS.app.name) (" | ") (title) }
                } @else {
                    title { (config::SETTINGS.app.name) }
                }

                @if !description.is_empty() {
                    meta name="description" content=(description);
                }

                meta name="viewport" content=(viewport);
                @for (name, content) in page.metadata() {
                    meta name=(name) content=(content) {}
                }

                meta http-equiv="X-UA-Compatible" content="IE=edge";
                @for (property, content) in page.properties() {
                    meta property=(property) content=(content) {}
                }

                @if let Some(favicon) = page.favicon() {
                    (favicon.prepare())
                }

                (page.context().prepare())
            }
        }
    }

    fn prepare_page_body(&self, page: &mut Page) -> Markup {
        html! {
            body class=[page.body_classes().get()] {
                @for (region, _) in self.regions().iter() {
                    @if let Some(content) = page.prepare_region(region) {
                        #(region) { (content) }
                    }
                }
            }
        }
    }

    fn before_render_page(&self, page: &mut Page) {
        if page.favicon().is_none() {
            page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")));
        }
    }

    #[rustfmt::skip]
    #[allow(unused_variables)]
    fn before_prepare_component(
        &self,
        component: &mut dyn ComponentTrait,
        cx: &mut Context,
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
    fn after_prepare_component(
        &self,
        component: &mut dyn ComponentTrait,
        cx: &mut Context,
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
        cx: &mut Context,
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
}
