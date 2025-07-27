use crate::core::extension::ExtensionTrait;
use crate::core::theme::CONTENT_REGION_NAME;
use crate::global;
use crate::html::{html, Markup};
use crate::locale::L10n;
use crate::response::page::Page;

/// Representa una referencia a un tema.
///
/// Los temas son también extensiones. Por tanto se deben definir igual, es decir, como instancias
/// estáticas globales que implementan [`ThemeTrait`], pero también [`ExtensionTrait`].
pub type ThemeRef = &'static dyn ThemeTrait;

/// Interfaz común que debe implementar cualquier tema de `PageTop`.
///
/// Un tema implementará [`ThemeTrait`] y los métodos que sean necesarios de [`ExtensionTrait`],
/// aunque el único obligatorio es [`theme()`](ExtensionTrait::theme).
///
/// ```rust
/// use pagetop::prelude::*;
///
/// pub struct MyTheme;
///
/// impl ExtensionTrait for MyTheme {
///     fn name(&self) -> L10n { L10n::n("My theme") }
///     fn description(&self) -> L10n { L10n::n("Un tema personal") }
///
///     fn theme(&self) -> Option<ThemeRef> {
///         Some(&Self)
///     }
/// }
///
/// impl ThemeTrait for MyTheme {}
/// ```
pub trait ThemeTrait: ExtensionTrait + Send + Sync {
    fn regions(&self) -> Vec<(&'static str, L10n)> {
        vec![(CONTENT_REGION_NAME, L10n::l("content"))]
    }

    #[allow(unused_variables)]
    fn before_render_page_body(&self, page: &mut Page) {}

    fn render_page_body(&self, page: &mut Page) -> Markup {
        html! {
            body id=[page.body_id().get()] class=[page.body_classes().get()] {
                @for (region_name, _) in self.regions() {
                    @let output = page.render_region(region_name);
                    @if !output.is_empty() {
                        div id=(region_name) class={ "region-container region-" (region_name) } {
                            (output)
                        }
                    }
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

                (page.render_assets())
            }
        }
    }

    fn error403(&self, _page: &mut Page) -> Markup {
        html! { div { h1 { ("FORBIDDEN ACCESS") } } }
    }

    fn error404(&self, _page: &mut Page) -> Markup {
        html! { div { h1 { ("RESOURCE NOT FOUND") } } }
    }
}
