use crate::prelude::*;

pub async fn summary() -> server::Result<Markup> {
    let top_menu = Menu::prepare()
        .add(MenuItem::label("Opción 1"))
        .add(MenuItem::link("Opción 2", "https://www.google.es"))
        .add(MenuItem::link_blank("Opción 3", "https://www.google.es"))
        .add(MenuItem::submenu("Submenú 1", Menu::prepare()
            .add(MenuItem::label("Opción 1"))
            .add(MenuItem::label("Opción 2"))
        ))
        .add(MenuItem::separator())
        .add(MenuItem::submenu("Submenú 2", Menu::prepare()
            .add(MenuItem::label("Opción 1"))
            .add(MenuItem::label("Opción 2"))
        ))
        .add(MenuItem::label("Opción 4"));

    let side_menu = Menu::prepare()
        .add(MenuItem::label("Opción 1"))
        .add(MenuItem::link("Opción 2", "https://www.google.es"))
        .add(MenuItem::link_blank("Opción 3", "https://www.google.es"))
        .add(MenuItem::submenu("Submenú 1", Menu::prepare()
            .add(MenuItem::label("Opción 1"))
            .add(MenuItem::label("Opción 2"))
        ))
        .add(MenuItem::separator())
        .add(MenuItem::submenu("Submenú 2", Menu::prepare()
            .add(MenuItem::label("Opción 1"))
            .add(MenuItem::label("Opción 2"))
        ))
        .add(MenuItem::label("Opción 4"));

    Page::prepare()
        .with_title("Admin")

        .add_to("top-menu", top_menu)

        .add_to("content", Container::row()
            .add(Container::column()
                .add(side_menu)
            )
            .add(Container::column()
                .add(Chunck::markup(html! {
                    p { "Columna 2"}
                }))
            )
        )


        .using_template("admin")

        .render()
}
