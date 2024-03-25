use crate::base::component::Layout;
use crate::core::component::{ComponentBase, ComponentTrait, Context};
use crate::core::package::PackageTrait;
use crate::html::{html, Favicon, Markup};
use crate::locale::L10n;
use crate::response::page::Page;
use crate::{concat_string, config};

pub type ThemeRef = &'static dyn ThemeTrait;

/// Los temas deben implementar este "trait".
pub trait ThemeTrait: PackageTrait + Send + Sync {
    #[rustfmt::skip]
    fn regions(&self) -> Vec<(&'static str, L10n)> {
        vec![
            ("header",        L10n::l("header")),
            ("pagetop",       L10n::l("pagetop")),
            ("sidebar_left",  L10n::l("sidebar_left")),
            ("content",       L10n::l("content")),
            ("sidebar_right", L10n::l("sidebar_right")),
            ("footer",        L10n::l("footer")),
        ]
    }

    #[allow(unused_variables)]
    fn before_prepare_body(&self, page: &mut Page) {}

    fn prepare_body(&self, page: &mut Page) -> Markup {
        let skip_to = concat_string!("#", page.skip_to().get().unwrap_or("content".to_owned()));

        html! {
            body id=[page.body_id().get()] class=[page.body_classes().get()] {
                @if let Some(skip) = L10n::l("skip_to_content").using(page.context().langid()) {
                    div class="skip__to_content" {
                        a href=(skip_to) { (skip) }
                    }
                }
                (Layout::new().render(page.context()))
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

                (page.context().prepare_assets())
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
                    if let Some(b) = component.downcast_mut::<Block>() {
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
                    if let Some(b) = component.downcast_mut::<Block>() {
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
