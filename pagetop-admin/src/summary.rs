//use super::l;
use pagetop::prelude::*;

pub async fn summary(request: server::HttpRequest) -> ResultPage<Markup, FatalError> {
    /*
    let top_menu = Menu::new()
        .with_item(MenuItem::label(l("module_name").as_str()))
        .with_item(MenuItem::link("Opción 2", "https://www.google.es"))
        .with_item(MenuItem::link_blank("Opción 3", "https://www.google.es"))
        .with_item(MenuItem::submenu(
            "Submenú 1",
            Menu::new()
                .with_item(MenuItem::label("Opción 1"))
                .with_item(MenuItem::label("Opción 2")),
        ))
        .with_item(MenuItem::separator())
        .with_item(MenuItem::submenu(
            "Submenú 2",
            Menu::new()
                .with_item(MenuItem::label("Opción 1"))
                .with_item(MenuItem::label("Opción 2")),
        ))
        .with_item(MenuItem::label("Opción 4"));

    let side_menu = Menu::new()
        .with_item(MenuItem::label("Opción 1"))
        .with_item(MenuItem::link("Opción 2", "https://www.google.es"))
        .with_item(MenuItem::link_blank("Opción 3", "https://www.google.es"))
        .with_item(MenuItem::submenu(
            "Submenú 1",
            Menu::new()
                .with_item(MenuItem::label("Opción 1"))
                .with_item(MenuItem::label("Opción 2")),
        ))
        .with_item(MenuItem::separator())
        .with_item(MenuItem::submenu(
            "Submenú 2",
            Menu::new()
                .with_item(MenuItem::label("Opción 1"))
                .with_item(MenuItem::label("Opción 2")),
        ))
        .with_item(MenuItem::label("Opción 4"));
    */

    Page::new(request)
        .with_context(ContextOp::Theme("Bootsier"))
        .with_title("Admin")
        /*      .with_this_in("top-menu", top_menu)
                .with_this_in(
                    "region-content",
                    grid::Row::new()
                        .with_column(grid::Column::new().with_component(side_menu))
                        .with_column(grid::Column::new().with_component(Html::with(html! {
                            p { "Columna 2"}
                        }))),
                )
        */
        .with_template("admin")
        .render()
}
