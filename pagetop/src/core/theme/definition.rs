use crate::core::package::PackageTrait;

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
    }

    #[allow(unused_variables)]
    fn before_prepare_body(&self, page: &mut Page) {}

    fn prepare_body(&self, page: &mut Page) -> PrepareMarkup {
        let skip_to_id = page.body_skip_to().get().unwrap_or("content".to_owned());

        PrepareMarkup::With(html! {
            body id=[page.body_id().get()] class=[page.body_classes().get()] {
                @if let Some(skip) = L10n::l("skip_to_content").using(page.context().langid()) {
                    div class="skip__to_content" {
                        a href=(concat_string!("#", skip_to_id)) { (skip) }
                    }
                }
                (flex::Container::new()
                    .with_id("body__wrapper")
                    .with_direction(flex::Direction::Column(BreakPoint::None))
                    .with_align(flex::Align::Center)
                    .add_item(flex::Item::region().with_id("header"))
                    .add_item(flex::Item::region().with_id("pagetop"))
                    .add_item(
                        flex::Item::with(
                            flex::Container::new()
                                .with_direction(flex::Direction::Row(BreakPoint::None))
                                .add_item(
                                    flex::Item::region()
                                        .with_id("sidebar_left")
                                        .with_grow(flex::Grow::Is1),
                                )
                                .add_item(
                                    flex::Item::region()
                                        .with_id("content")
                                        .with_grow(flex::Grow::Is3),
                                )
                                .add_item(
                                    flex::Item::region()
                                        .with_id("sidebar_right")
                                        .with_grow(flex::Grow::Is1),
                                ),
                        )
                        .with_id("flex__wrapper"),
                    )
                    .add_item(flex::Item::region().with_id("footer"))
                    .render(page.context()))
            }
        })
    }

    fn after_prepare_body(&self, page: &mut Page) {
        page.set_assets(AssetsOp::SetFaviconIfNone(
            Favicon::new().with_icon("/base/favicon.ico"),
        ));
    }

    fn prepare_head(&self, page: &mut Page) -> PrepareMarkup {
        let viewport = "width=device-width, initial-scale=1, shrink-to-fit=no";
        PrepareMarkup::With(html! {
            head {
                meta charset="utf-8";

                @if let Some(title) = page.title() {
                    title { (global::SETTINGS.app.name) (" - ") (title) }
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
    */
}
