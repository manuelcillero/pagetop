use crate::base::component::Region;
use crate::core::component::ComponentBase;
use crate::core::extension::ExtensionTrait;
use crate::global;
use crate::html::{html, Markup};
use crate::locale::L10n;
use crate::response::page::Page;

pub type ThemeRef = &'static dyn ThemeTrait;

/// Los temas deben implementar este "trait".
pub trait ThemeTrait: ExtensionTrait + Send + Sync {
    fn regions(&self) -> Vec<(&'static str, L10n)> {
        vec![("region-content", L10n::l("content"))]
    }

    #[allow(unused_variables)]
    fn before_render_page_body(&self, page: &mut Page) {}

    fn render_page_body(&self, page: &mut Page) -> Markup {
        html! {
            body id=[page.body_id().get()] class=[page.body_classes().get()] {
                @for (region_id, _) in self.regions() {
                    (Region::of(region_id).render(page.context()))
                }
            }
        }
    }

    #[allow(unused_variables)]
    fn after_render_page_body(&self, page: &mut Page) {}

    fn render_page_head(&self, page: &mut Page) -> Markup {
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
}
