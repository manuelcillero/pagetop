use crate::core::package::PackageTrait;
use crate::html::{html, PrepareMarkup};
use crate::locale::L10n;
use crate::response::page::Page;
use crate::{global, service};

pub type ThemeRef = &'static dyn ThemeTrait;

/// Los temas deben implementar este "trait".
pub trait ThemeTrait: PackageTrait + Send + Sync {
    /*
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
    } */

    fn prepare_page_head(&self, page: &mut Page) -> PrepareMarkup {
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

    fn prepare_page_body(&self, page: &mut Page) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            body id=[page.body_id().get()] class=[page.body_classes().get()] {
                (page.body_content().render())
            }
        })
    }

    fn error_403(&self, request: service::HttpRequest) -> Page {
        Page::new(request)
            .with_title(L10n::n("Error FORBIDDEN"))
            .with_body(PrepareMarkup::With(html! {
                div {
                    h1 { ("FORBIDDEN ACCESS") }
                }
            }))
    }

    fn error_404(&self, request: service::HttpRequest) -> Page {
        Page::new(request)
            .with_title(L10n::n("Error RESOURCE NOT FOUND"))
            .with_body(PrepareMarkup::With(html! {
                div {
                    h1 { ("RESOURCE NOT FOUND") }
                }
            }))
    }
}
