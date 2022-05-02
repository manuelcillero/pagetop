use pagetop::prelude::*;
use super::l;

pub async fn summary() -> app::Result<Markup> {
    let top_menu = Menu::new()
        .add(MenuItem::label(l("module_name").as_str()))
        .add(MenuItem::link("Opción 2", "https://www.google.es"))
        .add(MenuItem::link_blank("Opción 3", "https://www.google.es"))
        .add(MenuItem::submenu("Submenú 1", Menu::new()
            .add(MenuItem::label("Opción 1"))
            .add(MenuItem::label("Opción 2"))
        ))
        .add(MenuItem::separator())
        .add(MenuItem::submenu("Submenú 2", Menu::new()
            .add(MenuItem::label("Opción 1"))
            .add(MenuItem::label("Opción 2"))
        ))
        .add(MenuItem::label("Opción 4"));

    let side_menu = Menu::new()
        .add(MenuItem::label("Opción 1"))
        .add(MenuItem::link("Opción 2", "https://www.google.es"))
        .add(MenuItem::link_blank("Opción 3", "https://www.google.es"))
        .add(MenuItem::submenu("Submenú 1", Menu::new()
            .add(MenuItem::label("Opción 1"))
            .add(MenuItem::label("Opción 2"))
        ))
        .add(MenuItem::separator())
        .add(MenuItem::submenu("Submenú 2", Menu::new()
            .add(MenuItem::label("Opción 1"))
            .add(MenuItem::label("Opción 2"))
        ))
        .add(MenuItem::label("Opción 4"));

    Page::new()

        .using_theme("Bootsier")

        .with_title("Admin")

        .add_to("top-menu", top_menu)

        .add_to("content", grid::Row::new()
            .add_column(grid::Column::new()
                .add(side_menu)
            )
            .add_column(grid::Column::new()
                .add(Chunck::with(html! {
                    p { "Columna 2"}
                }))
            )
        )


        .using_template("admin")

        .render()
}
