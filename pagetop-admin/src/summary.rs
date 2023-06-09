use crate::LOCALE_ADMIN;

use pagetop::prelude::*;
use pagetop_megamenu::component::{MegaMenu, MegaMenuItem};
use pagetop_minimal::component::*;

pub async fn summary(request: server::HttpRequest) -> ResultPage<Markup, FatalError> {
    let top_menu = MegaMenu::new()
        .with_item(MegaMenuItem::label(L10n::t("module_name", &LOCALE_ADMIN)))
        .with_item(MegaMenuItem::link(
            L10n::n("Opción 2"),
            "https://www.google.es",
        ))
        .with_item(MegaMenuItem::link_blank(
            L10n::n("Opción 3"),
            "https://www.google.es",
        ))
        .with_item(MegaMenuItem::submenu(
            L10n::n("Submenú 1"),
            MegaMenu::new()
                .with_item(MegaMenuItem::label(L10n::n("Opción 1")))
                .with_item(MegaMenuItem::label(L10n::n("Opción 2"))),
        ))
        .with_item(MegaMenuItem::separator())
        .with_item(MegaMenuItem::submenu(
            L10n::n("Submenú 2"),
            MegaMenu::new()
                .with_item(MegaMenuItem::label(L10n::n("Opción 1")))
                .with_item(MegaMenuItem::label(L10n::n("Opción 2"))),
        ))
        .with_item(MegaMenuItem::label(L10n::n("Opción 4")));

    let side_menu = MegaMenu::new()
        .with_item(MegaMenuItem::label(L10n::n("Opción 1")))
        .with_item(MegaMenuItem::link(
            L10n::n("Opción 2"),
            "https://www.google.es",
        ))
        .with_item(MegaMenuItem::link_blank(
            L10n::n("Opción 3"),
            "https://www.google.es",
        ))
        .with_item(MegaMenuItem::submenu(
            L10n::n("Submenú 1"),
            MegaMenu::new()
                .with_item(MegaMenuItem::label(L10n::n("Opción 1")))
                .with_item(MegaMenuItem::label(L10n::n("Opción 2"))),
        ))
        .with_item(MegaMenuItem::separator())
        .with_item(MegaMenuItem::submenu(
            L10n::n("Submenú 2"),
            MegaMenu::new()
                .with_item(MegaMenuItem::label(L10n::n("Opción 1")))
                .with_item(MegaMenuItem::label(L10n::n("Opción 2"))),
        ))
        .with_item(MegaMenuItem::label(L10n::n("Opción 4")));

    Page::new(request)
        .with_context(ContextOp::Theme("Bootsier"))
        .with_title(L10n::n("Admin"))
        .with_in("top-menu", top_menu)
        .with_in(
            "content",
            grid::Row::new()
                .with_column(grid::Column::new().with_component(side_menu))
                .with_column(grid::Column::new().with_component(Html::with(html! {
                    p { "Columna 2"}
                }))),
        )
        .with_template("admin")
        .render()
}
