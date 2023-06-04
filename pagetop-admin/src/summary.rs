use super::LOCALE_ADMIN;
use pagetop::prelude::*;
use pagetop_megamenu::component::{MegaMenu, MegaMenuItem};
use pagetop_minimal::component::*;

pub async fn summary(request: server::HttpRequest) -> ResultPage<Markup, FatalError> {
    let top_menu = MegaMenu::new()
        .with_item(MegaMenuItem::label(L10n::t("module_name", &LOCALE_ADMIN)))
        .with_item(MegaMenuItem::link(
            L10n::text("Opción 2"),
            "https://www.google.es",
        ))
        .with_item(MegaMenuItem::link_blank(
            L10n::text("Opción 3"),
            "https://www.google.es",
        ))
        .with_item(MegaMenuItem::submenu(
            L10n::text("Submenú 1"),
            MegaMenu::new()
                .with_item(MegaMenuItem::label(L10n::text("Opción 1")))
                .with_item(MegaMenuItem::label(L10n::text("Opción 2"))),
        ))
        .with_item(MegaMenuItem::separator())
        .with_item(MegaMenuItem::submenu(
            L10n::text("Submenú 2"),
            MegaMenu::new()
                .with_item(MegaMenuItem::label(L10n::text("Opción 1")))
                .with_item(MegaMenuItem::label(L10n::text("Opción 2"))),
        ))
        .with_item(MegaMenuItem::label(L10n::text("Opción 4")));

    let side_menu = MegaMenu::new()
        .with_item(MegaMenuItem::label(L10n::text("Opción 1")))
        .with_item(MegaMenuItem::link(
            L10n::text("Opción 2"),
            "https://www.google.es",
        ))
        .with_item(MegaMenuItem::link_blank(
            L10n::text("Opción 3"),
            "https://www.google.es",
        ))
        .with_item(MegaMenuItem::submenu(
            L10n::text("Submenú 1"),
            MegaMenu::new()
                .with_item(MegaMenuItem::label(L10n::text("Opción 1")))
                .with_item(MegaMenuItem::label(L10n::text("Opción 2"))),
        ))
        .with_item(MegaMenuItem::separator())
        .with_item(MegaMenuItem::submenu(
            L10n::text("Submenú 2"),
            MegaMenu::new()
                .with_item(MegaMenuItem::label(L10n::text("Opción 1")))
                .with_item(MegaMenuItem::label(L10n::text("Opción 2"))),
        ))
        .with_item(MegaMenuItem::label(L10n::text("Opción 4")));

    Page::new(request)
        .with_context(ContextOp::Theme("Bootsier"))
        .with_title(L10n::text("Admin"))
        .with_this_in("top-menu", top_menu)
        .with_this_in(
            "content",
            grid::Row::new()
                .with_column(grid::Column::new().with_component(side_menu))
                .with_column(grid::Column::new().with_component(L10n::html(html! {
                    p { "Columna 2"}
                }))),
        )
        .with_template("admin")
        .render()
}
