use crate::base::component::*;
use crate::core::component::{ComponentBase, ComponentClassesOp, ComponentTrait};
use crate::core::package::PackageTrait;
use crate::html::{html, ClassesOp, Favicon, Markup};
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
        let skip_to_id = concat_string!("#", page.skip_to().get().unwrap_or("content".to_owned()));

        Container::body()
            .with_id(page.body_id().get().unwrap_or_default())
            .with_classes(ClassesOp::Add, page.body_classes().get().unwrap_or_default())
            .add_item(Flex::bundle()
                .add_component(Html::with(html! {
                    @if let Some(skip) = L10n::l("skip_to_content").using(page.context().langid()) {
                        div class="skip__to_content" {
                            a href=(skip_to_id) { (skip) }
                        }
                    }
                }))
                .add_component(Container::new()
                    .with_id("body__wrapper")
                    .with_direction(FlexDirection::Column(BreakPoint::None))
                    .with_align(FlexAlign::Center)
                    .add_item(Flex::with(Region::named("header")).with_id("header"))
                    .add_item(Flex::with(Region::named("pagetop")).with_id("pagetop"))
                    .add_item(
                        Flex::with(
                            Container::new()
                                .with_direction(FlexDirection::Row(BreakPoint::None))
                                .add_item(
                                    Flex::with(Region::named("sidebar_left"))
                                        .with_id("sidebar_left")
                                        .with_grow(FlexGrow::Is1),
                                )
                                .add_item(
                                    Flex::with(Region::named("content"))
                                        .with_id("content")
                                        .with_grow(FlexGrow::Is3),
                                )
                                .add_item(
                                    Flex::with(Region::named("sidebar_right"))
                                        .with_id("sidebar_right")
                                        .with_grow(FlexGrow::Is1),
                                ),
                        )
                        .with_id("flex__wrapper"),
                    )
                    .add_item(Flex::with(Region::named("footer")).with_id("footer")),
                )
            )
            .render(page.context())
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
}
