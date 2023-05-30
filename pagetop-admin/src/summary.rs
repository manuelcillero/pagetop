use super::LOCALE_ADMIN;
use pagetop::prelude::*;
use pagetop_megamenu::component::{MegaMenu, MegaMenuItem};
use pagetop_minimal::component::*;

pub async fn summary(request: server::HttpRequest) -> ResultPage<Markup, FatalError> {
    let top_menu = MegaMenu::new()
        .with_item(MegaMenuItem::label(Text::t("module_name", &LOCALE_ADMIN)))
        .with_item(MegaMenuItem::link(
            Text::n("Opción 2"),
            "https://www.google.es",
        ))
        .with_item(MegaMenuItem::link_blank(
            Text::n("Opción 3"),
            "https://www.google.es",
        ))
        .with_item(MegaMenuItem::submenu(
            Text::n("Submenú 1"),
            MegaMenu::new()
                .with_item(MegaMenuItem::label(Text::n("Opción 1")))
                .with_item(MegaMenuItem::label(Text::n("Opción 2"))),
        ))
        .with_item(MegaMenuItem::separator())
        .with_item(MegaMenuItem::submenu(
            Text::n("Submenú 2"),
            MegaMenu::new()
                .with_item(MegaMenuItem::label(Text::n("Opción 1")))
                .with_item(MegaMenuItem::label(Text::n("Opción 2"))),
        ))
        .with_item(MegaMenuItem::label(Text::n("Opción 4")));

    let side_menu = MegaMenu::new()
        .with_item(MegaMenuItem::label(Text::n("Opción 1")))
        .with_item(MegaMenuItem::link(
            Text::n("Opción 2"),
            "https://www.google.es",
        ))
        .with_item(MegaMenuItem::link_blank(
            Text::n("Opción 3"),
            "https://www.google.es",
        ))
        .with_item(MegaMenuItem::submenu(
            Text::n("Submenú 1"),
            MegaMenu::new()
                .with_item(MegaMenuItem::label(Text::n("Opción 1")))
                .with_item(MegaMenuItem::label(Text::n("Opción 2"))),
        ))
        .with_item(MegaMenuItem::separator())
        .with_item(MegaMenuItem::submenu(
            Text::n("Submenú 2"),
            MegaMenu::new()
                .with_item(MegaMenuItem::label(Text::n("Opción 1")))
                .with_item(MegaMenuItem::label(Text::n("Opción 2"))),
        ))
        .with_item(MegaMenuItem::label(Text::n("Opción 4")));

    Page::new(request)
        .with_context(ContextOp::Theme("Bootsier"))
        .with_title(Text::n("Admin"))
        .with_this_in("top-menu", top_menu)
        .with_this_in(
            "region-content",
            grid::Row::new()
                .with_column(grid::Column::new().with_component(side_menu))
                .with_column(grid::Column::new().with_component(Html::n(html! {
                    p { "Columna 2"}
                }))),
        )
        .with_template("admin")
        .render()
}
