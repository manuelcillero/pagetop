use super::l;
use pagetop::prelude::*;
use pagetop_megamenu::component::{MegaMenu, MegaMenuItem};

pub async fn summary(request: server::HttpRequest) -> ResultPage<Markup, FatalError> {
    let top_menu = MegaMenu::new()
        .with_item(MegaMenuItem::label(l("module_name").as_str()))
        .with_item(MegaMenuItem::link("Opción 2", "https://www.google.es"))
        .with_item(MegaMenuItem::link_blank(
            "Opción 3",
            "https://www.google.es",
        ))
        .with_item(MegaMenuItem::submenu(
            "Submenú 1",
            MegaMenu::new()
                .with_item(MegaMenuItem::label("Opción 1"))
                .with_item(MegaMenuItem::label("Opción 2")),
        ))
        .with_item(MegaMenuItem::separator())
        .with_item(MegaMenuItem::submenu(
            "Submenú 2",
            MegaMenu::new()
                .with_item(MegaMenuItem::label("Opción 1"))
                .with_item(MegaMenuItem::label("Opción 2")),
        ))
        .with_item(MegaMenuItem::label("Opción 4"));

    let side_menu = MegaMenu::new()
        .with_item(MegaMenuItem::label("Opción 1"))
        .with_item(MegaMenuItem::link("Opción 2", "https://www.google.es"))
        .with_item(MegaMenuItem::link_blank(
            "Opción 3",
            "https://www.google.es",
        ))
        .with_item(MegaMenuItem::submenu(
            "Submenú 1",
            MegaMenu::new()
                .with_item(MegaMenuItem::label("Opción 1"))
                .with_item(MegaMenuItem::label("Opción 2")),
        ))
        .with_item(MegaMenuItem::separator())
        .with_item(MegaMenuItem::submenu(
            "Submenú 2",
            MegaMenu::new()
                .with_item(MegaMenuItem::label("Opción 1"))
                .with_item(MegaMenuItem::label("Opción 2")),
        ))
        .with_item(MegaMenuItem::label("Opción 4"));

    Page::new(request)
        .with_context(ContextOp::Theme("Bootsier"))
        .with_title("Admin")
        .with_this_in("top-menu", top_menu)
        .with_this_in(
            "region-content",
            grid::Row::new()
                .with_column(grid::Column::new().with_component(side_menu))
                .with_column(grid::Column::new().with_component(Html::with(html! {
                    p { "Columna 2"}
                }))),
        )
        .with_template("admin")
        .render()
}
