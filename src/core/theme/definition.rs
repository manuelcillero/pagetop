use crate::core::component::{ComponentTrait, Context};
use crate::core::package::PackageTrait;
use crate::html::{html, Favicon, Markup, OptionId};
use crate::locale::L10n;
use crate::response::page::Page;
use crate::{concat_string, config};

pub type ThemeRef = &'static dyn ThemeTrait;

/// Los temas deben implementar este "trait".
pub trait ThemeTrait: PackageTrait + Send + Sync {
    #[rustfmt::skip]
    fn regions(&self) -> Vec<(&'static str, L10n)> {
        vec![
            ("header",  L10n::l("header")),
            ("pagetop", L10n::l("pagetop")),
            ("content", L10n::l("content")),
            ("sidebar", L10n::l("sidebar")),
            ("footer",  L10n::l("footer")),
        ]
    }

    fn prepare_region(&self, page: &mut Page, region_name: &str) -> Markup {
        let render_region = page.components_in(region_name).render(page.context());
        if render_region.is_empty() {
            html! {}
        } else {
            html! {
                div id=[OptionId::new(region_name).get()] class="pt-region" {
                    div class="pt-region__inner" {
                        (render_region)
                    }
                }
            }
        }
    }

    #[allow(unused_variables)]
    fn before_prepare_body(&self, page: &mut Page) {}

    fn prepare_body(&self, page: &mut Page) -> Markup {
        let skip_to = concat_string!("#", page.skip_to().get().unwrap_or("content".to_owned()));

        html! {
            body id=[page.body_id().get()] class=[page.body_classes().get()] {
                @if let Some(skip) = L10n::l("skip_to_content").using(page.context().langid()) {
                    div class="pt-body__skip" {
                        a href=(skip_to) { (skip) }
                    }
                }
                div class="pt-body__wrapper" {
                    div class="pt-body__regions" {
                        (self.prepare_region(page, "header"))
                        (self.prepare_region(page, "pagetop"))
                        div class="pt-content" {
                            div class="pt-content__inner" {
                                (self.prepare_region(page, "content"))
                                (self.prepare_region(page, "sidebar"))
                            }
                        }
                        (self.prepare_region(page, "footer"))
                    }
                }
            }
        }
    }

    fn after_prepare_body(&self, page: &mut Page) {
        if page.favicon().is_none() {
            page.alter_favicon(Some(Favicon::new().with_icon("/base/favicon.ico")));
        }
    }

    fn prepare_head(&self, page: &mut Page) -> Markup {
        let viewport = "width=device-width, initial-scale=1, shrink-to-fit=no";
        html! {
            head {
                meta charset="utf-8";

                @if let Some(title) = page.title() {
                    title { (config::SETTINGS.app.name) (" - ") (title) }
                } @else {
                    title { (config::SETTINGS.app.name) }
                }

                @if let Some(description) = page.description() {
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

    #[rustfmt::skip]
    #[allow(unused_variables)]
    fn before_prepare_component(
        &self,
        component: &mut dyn ComponentTrait,
        cx: &mut Context,
    ) {
        /*
            Cómo usarlo:

            match component.type_id() {
                t if t == TypeId::of::<Block>() => {
                    if let Some(b) = component_as_mut::<Block>(component) {
                        b.alter_title("New title");
                    }
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

            match component.type_id() {
                t if t == TypeId::of::<Block>() => {
                    if let Some(b) = component_as_mut::<Block>(component) {
                        b.alter_title("New title");
                    }
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

            match component.type_id() {
                t if t == TypeId::of::<Block>() => {
                    Some(block_default(block))
                },
                _ => None,
            }
        */
    }
}
