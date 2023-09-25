use crate::LOCALES_ADMIN;

use pagetop::prelude::*;
use pagetop_megamenu::component::{MegaItem, MegaMenu};

pub async fn summary(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    let top_menu = MegaMenu::new()
        .with_item(MegaItem::label(L10n::t("module_name", &LOCALES_ADMIN)))
        .with_item(MegaItem::link(L10n::n("Opción 2"), |_| {
            "https://www.google.es"
        }))
        .with_item(MegaItem::link_blank(L10n::n("Opción 3"), |_| {
            "https://www.google.es"
        }))
        .with_item(MegaItem::submenu(
            L10n::n("Submenú 1"),
            MegaMenu::new()
                .with_item(MegaItem::label(L10n::n("Opción 1")))
                .with_item(MegaItem::label(L10n::n("Opción 2"))),
        ))
        .with_item(MegaItem::separator())
        .with_item(MegaItem::submenu(
            L10n::n("Submenú 2"),
            MegaMenu::new()
                .with_item(MegaItem::label(L10n::n("Opción 1")))
                .with_item(MegaItem::label(L10n::n("Opción 2"))),
        ))
        .with_item(MegaItem::label(L10n::n("Opción 4")));

    let side_menu = MegaMenu::new()
        .with_item(MegaItem::label(L10n::n("Opción 1")))
        .with_item(MegaItem::link(L10n::n("Opción 2"), |_| {
            "https://www.google.es"
        }))
        .with_item(MegaItem::link_blank(L10n::n("Opción 3"), |_| {
            "https://www.google.es"
        }))
        .with_item(MegaItem::submenu(
            L10n::n("Submenú 1"),
            MegaMenu::new()
                .with_item(MegaItem::label(L10n::n("Opción 1")))
                .with_item(MegaItem::label(L10n::n("Opción 2"))),
        ))
        .with_item(MegaItem::separator())
        .with_item(MegaItem::submenu(
            L10n::n("Submenú 2"),
            MegaMenu::new()
                .with_item(MegaItem::label(L10n::n("Opción 1")))
                .with_item(MegaItem::label(L10n::n("Opción 2"))),
        ))
        .with_item(MegaItem::label(L10n::n("Opción 4")));

    Page::new(request)
        .with_context(ContextOp::Theme("Bootsier"))
        .with_title(L10n::n("Admin"))
        .with_in("top-menu", top_menu)
        .with_in(
            "content",
            flex::Container::new()
                .with_item(flex::Item::new().with_component(side_menu))
                .with_item(flex::Item::new().with_component(Html::with(html! {
                    p { "Columna 2"}
                }))),
        )
        .with_template("admin")
        .render()
}
