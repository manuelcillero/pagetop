use crate::LOCALES_ADMIN;

use pagetop::prelude::*;

pub async fn summary(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    let top_menu = Menu::new()
        .with_id("admin-menu-test")
        .add_item(menu::Item::label(L10n::t("package_name", &LOCALES_ADMIN)))
        .add_item(menu::Item::label(L10n::n("Ejemplo \"Label\"")))
        .add_item(menu::Item::link(L10n::n("Ejemplo \"Link\""), |_| {
            "https://www.google.es"
        }))
        .add_item(menu::Item::link_blank(
            L10n::n("Ejemplo \"LinkBlank\""),
            |_| "https://www.google.es",
        ))
        .add_item(menu::Item::submenu(
            L10n::n("Ejemplo Submenú"),
            menu::Submenu::new()
                .with_title(L10n::n("Título submenú"))
                .add_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                    "https://www.google.es"
                }))
                .add_item(menu::Item::link_blank(
                    L10n::n("Opción \"LinkBlank\""),
                    |_| "https://www.google.es",
                ))
                .add_item(menu::Item::submenu(
                    L10n::n("Otro submenú con un texto muy, pero que muy largo"),
                    menu::Submenu::new()
                        .add_item(menu::Item::label(L10n::n("Opción \"Label\"")))
                        .add_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                            "https://www.google.es"
                        }))
                        .add_item(menu::Item::link_blank(
                            L10n::n("Opción \"LinkBlank\""),
                            |_| "https://www.google.es",
                        ))
                        .add_item(menu::Item::label(L10n::n("Opción \"Label\""))),
                ))
                .add_item(menu::Item::label(L10n::n("Opción \"Label\""))),
        ))
        .add_item(menu::Item::megamenu(
            L10n::n("Ejemplo Megamenú 1"),
            menu::Megamenu::new()
                .add_group(
                    menu::Group::new()
                        .add_element(menu::Element::submenu(
                            menu::Submenu::new()
                                .with_title(L10n::n("Título submenú"))
                                .add_item(menu::Item::label(L10n::n("Opción \"Label\"")))
                                .add_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                                    "https://www.google.es"
                                }))
                                .add_item(menu::Item::link_blank(
                                    L10n::n("Opción \"LinkBlank\""),
                                    |_| "https://www.google.es",
                                )),
                        ))
                        .add_element(menu::Element::submenu(
                            menu::Submenu::new()
                                .with_title(L10n::n("Título submenú"))
                                .add_item(menu::Item::label(L10n::n("Opción \"Label\"")))
                                .add_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                                    "https://www.google.es"
                                }))
                                .add_item(menu::Item::link_blank(
                                    L10n::n("Opción \"LinkBlank\""),
                                    |_| "https://www.google.es",
                                )),
                        )),
                )
                .add_group(
                    menu::Group::new().add_element(menu::Element::submenu(
                        menu::Submenu::new()
                            .add_item(menu::Item::label(L10n::n("Opción \"Label\"")))
                            .add_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                                "https://www.google.es"
                            }))
                            .add_item(menu::Item::link_blank(
                                L10n::n("Opción \"LinkBlank\""),
                                |_| "https://www.google.es",
                            ))
                            .add_item(menu::Item::label(L10n::n("Opción \"Label\""))),
                    )),
                )
                .add_group(
                    menu::Group::new()
                        .add_element(menu::Element::submenu(
                            menu::Submenu::new()
                                .with_title(L10n::n("Título submenú"))
                                .add_item(menu::Item::label(L10n::n("Opción \"Label\"")))
                                .add_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                                    "https://www.google.es"
                                }))
                                .add_item(menu::Item::link_blank(
                                    L10n::n("Opción \"LinkBlank\""),
                                    |_| "https://www.google.es",
                                )),
                        ))
                        .add_element(menu::Element::submenu(
                            menu::Submenu::new()
                                .with_title(L10n::n("Título submenú"))
                                .add_item(menu::Item::label(L10n::n("Opción \"Label\"")))
                                .add_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                                    "https://www.google.es"
                                }))
                                .add_item(menu::Item::link_blank(
                                    L10n::n("Opción \"LinkBlank\""),
                                    |_| "https://www.google.es",
                                )),
                        )),
                )
                .add_group(
                    menu::Group::new().add_element(menu::Element::submenu(
                        menu::Submenu::new()
                            .add_item(menu::Item::label(L10n::n("Opción \"Label\"")))
                            .add_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                                "https://www.google.es"
                            }))
                            .add_item(menu::Item::link_blank(
                                L10n::n("Opción \"LinkBlank\""),
                                |_| "https://www.google.es",
                            ))
                            .add_item(menu::Item::label(L10n::n("Opción \"Label\""))),
                    )),
                ),
        ));

    let side_menu = Menu::new()
        .add_item(menu::Item::label(L10n::n("Opción 1")))
        .add_item(menu::Item::link(L10n::n("Opción 2"), |_| {
            "https://www.google.es"
        }))
        .add_item(menu::Item::link_blank(L10n::n("Opción 3"), |_| {
            "https://www.google.es"
        }))
        .add_item(menu::Item::submenu(
            L10n::n("Submenú 1"),
            menu::Submenu::new()
                .add_item(menu::Item::label(L10n::n("Opción 1")))
                .add_item(menu::Item::label(L10n::n("Opción 2"))),
        )) /*
        .add_item(menu::Item::separator()) */
        .add_item(menu::Item::submenu(
            L10n::n("Submenú 2"),
            menu::Submenu::new()
                .add_item(menu::Item::label(L10n::n("Opción 1")))
                .add_item(menu::Item::label(L10n::n("Opción 2"))),
        ))
        .add_item(menu::Item::label(L10n::n("Opción 4")));

    Page::new(request)
        .with_title(L10n::n("Admin"))
        .with_component_in("top-menu", side_menu)
        .with_component(
            Container::new()
                .add_item(Flex::with(Html::with(html! { p { "Columna 1"} })))
                .add_item(Flex::with(top_menu))
                .add_item(Flex::with(Html::with(html! { p { "Columna 3"} }))),
        )
        .render()
}
