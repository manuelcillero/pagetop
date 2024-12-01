use crate::core::package::PackageTrait;
use crate::global;
use crate::html::{html, PrepareMarkup};
use crate::locale::L10n;
use crate::response::page::Page;

pub type ThemeRef = &'static dyn ThemeTrait;

/// Los temas deben implementar este "trait".
pub trait ThemeTrait: PackageTrait + Send + Sync {
    fn regions(&self) -> Vec<(&'static str, L10n)> {
        vec![("content", L10n::l("content"))]
    }

    fn prepare_head(&self, page: &mut Page) -> PrepareMarkup {
        let viewport = "width=device-width, initial-scale=1, shrink-to-fit=no";
        PrepareMarkup::With(html! {
            head {
                meta charset="utf-8";

                @if let Some(title) = page.title() {
                    title { (global::SETTINGS.app.name) (" | ") (title) }
                } @else {
                    title { (global::SETTINGS.app.name) }
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

                (page.context().prepare_assets())
            }
        })
    }

    #[allow(unused_variables)]
    fn before_prepare_body(&self, page: &mut Page) {}

    fn prepare_body(&self, page: &mut Page) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            body id=[page.body_id().get()] class=[page.body_classes().get()] {
                (page.context().prepare_region("content"))
            }
        })
    }

    #[allow(unused_variables)]
    fn after_prepare_body(&self, page: &mut Page) {}
}
