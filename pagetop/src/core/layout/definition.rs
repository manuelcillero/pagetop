use crate::core::package::PackageTrait;
use crate::global;
use crate::html::{html, Markup};
use crate::locale::L10n;
use crate::response::page::Page;

pub type LayoutRef = &'static dyn LayoutTrait;

/// Los diseños deben implementar este "trait".
pub trait LayoutTrait: PackageTrait + Send + Sync {
    fn regions(&self) -> Vec<(&'static str, L10n)> {
        vec![("content", L10n::l("content"))]
    }

    fn render_head(&self, page: &mut Page) -> Markup {
        let viewport = "width=device-width, initial-scale=1, shrink-to-fit=no";
        html! {
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

                (page.context().render_assets())
            }
        }
    }

    fn render_body(&self, page: &mut Page) -> Markup {
        html! {
            body id=[page.body_id().get()] class=[page.body_classes().get()] {
                (page.context().render_region("content"))
            }
        }
    }
}
