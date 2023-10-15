use crate::LOCALES_ADMIN;

use pagetop::prelude::*;

pub async fn summary(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    let top_menu = Menu::new()
        .with_item(menu::Item::label(L10n::t("module_name", &LOCALES_ADMIN)))
        .with_item(menu::Item::label(L10n::n("Ejemplo \"Label\"")))
        .with_item(menu::Item::link(L10n::n("Ejemplo \"Link\""), |_| {
            "https://www.google.es"
        }))
        .with_item(menu::Item::link_blank(
            L10n::n("Ejemplo \"LinkBlank\""),
            |_| "https://www.google.es",
        ))
        .with_item(menu::Item::submenu(
            L10n::n("Ejemplo Submenú"),
            menu::Submenu::new()
                .with_title(L10n::n("Título submenú"))
                .with_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                    "https://www.google.es"
                }))
                .with_item(menu::Item::link_blank(
                    L10n::n("Opción \"LinkBlank\""),
                    |_| "https://www.google.es",
                ))
                .with_item(menu::Item::submenu(
                    L10n::n("Otro submenú con un texto muy, pero que muy largo"),
                    menu::Submenu::new()
                        .with_item(menu::Item::label(L10n::n("Opción \"Label\"")))
                        .with_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                            "https://www.google.es"
                        }))
                        .with_item(menu::Item::link_blank(
                            L10n::n("Opción \"LinkBlank\""),
                            |_| "https://www.google.es",
                        ))
                        .with_item(menu::Item::label(L10n::n("Opción \"Label\""))),
                ))
                .with_item(menu::Item::label(L10n::n("Opción \"Label\""))),
        ))
        .with_item(menu::Item::megamenu(
            L10n::n("Ejemplo Megamenú 1"),
            menu::Megamenu::new()
                .with_group(
                    menu::Group::new()
                        .with_element(menu::Element::submenu(
                            menu::Submenu::new()
                                .with_title(L10n::n("Título submenú"))
                                .with_item(menu::Item::label(L10n::n("Opción \"Label\"")))
                                .with_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                                    "https://www.google.es"
                                }))
                                .with_item(menu::Item::link_blank(
                                    L10n::n("Opción \"LinkBlank\""),
                                    |_| "https://www.google.es",
                                )),
                        ))
                        .with_element(menu::Element::submenu(
                            menu::Submenu::new()
                                .with_title(L10n::n("Título submenú"))
                                .with_item(menu::Item::label(L10n::n("Opción \"Label\"")))
                                .with_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                                    "https://www.google.es"
                                }))
                                .with_item(menu::Item::link_blank(
                                    L10n::n("Opción \"LinkBlank\""),
                                    |_| "https://www.google.es",
                                )),
                        )),
                )
                .with_group(
                    menu::Group::new().with_element(menu::Element::submenu(
                        menu::Submenu::new()
                            .with_item(menu::Item::label(L10n::n("Opción \"Label\"")))
                            .with_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                                "https://www.google.es"
                            }))
                            .with_item(menu::Item::link_blank(
                                L10n::n("Opción \"LinkBlank\""),
                                |_| "https://www.google.es",
                            ))
                            .with_item(menu::Item::label(L10n::n("Opción \"Label\""))),
                    )),
                )
                .with_group(
                    menu::Group::new()
                        .with_element(menu::Element::submenu(
                            menu::Submenu::new()
                                .with_title(L10n::n("Título submenú"))
                                .with_item(menu::Item::label(L10n::n("Opción \"Label\"")))
                                .with_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                                    "https://www.google.es"
                                }))
                                .with_item(menu::Item::link_blank(
                                    L10n::n("Opción \"LinkBlank\""),
                                    |_| "https://www.google.es",
                                )),
                        ))
                        .with_element(menu::Element::submenu(
                            menu::Submenu::new()
                                .with_title(L10n::n("Título submenú"))
                                .with_item(menu::Item::label(L10n::n("Opción \"Label\"")))
                                .with_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                                    "https://www.google.es"
                                }))
                                .with_item(menu::Item::link_blank(
                                    L10n::n("Opción \"LinkBlank\""),
                                    |_| "https://www.google.es",
                                )),
                        )),
                )
                .with_group(
                    menu::Group::new().with_element(menu::Element::submenu(
                        menu::Submenu::new()
                            .with_item(menu::Item::label(L10n::n("Opción \"Label\"")))
                            .with_item(menu::Item::link(L10n::n("Opción \"Link\""), |_| {
                                "https://www.google.es"
                            }))
                            .with_item(menu::Item::link_blank(
                                L10n::n("Opción \"LinkBlank\""),
                                |_| "https://www.google.es",
                            ))
                            .with_item(menu::Item::label(L10n::n("Opción \"Label\""))),
                    )),
                ),
        ));

    let side_menu = Menu::new()
        .with_item(menu::Item::label(L10n::n("Opción 1")))
        .with_item(menu::Item::link(L10n::n("Opción 2"), |_| {
            "https://www.google.es"
        }))
        .with_item(menu::Item::link_blank(L10n::n("Opción 3"), |_| {
            "https://www.google.es"
        }))
        .with_item(menu::Item::submenu(
            L10n::n("Submenú 1"),
            menu::Submenu::new()
                .with_item(menu::Item::label(L10n::n("Opción 1")))
                .with_item(menu::Item::label(L10n::n("Opción 2"))),
        )) /*
        .with_item(menu::Item::separator()) */
        .with_item(menu::Item::submenu(
            L10n::n("Submenú 2"),
            menu::Submenu::new()
                .with_item(menu::Item::label(L10n::n("Opción 1")))
                .with_item(menu::Item::label(L10n::n("Opción 2"))),
        ))
        .with_item(menu::Item::label(L10n::n("Opción 4")));

    Page::new(request)
        //.with_context(ContextOp::Theme("Bootsier"))
        .with_title(L10n::n("Admin"))
        .with_in("top-menu", side_menu)
        .with_in(
            "content",
            flex::Container::new()
                .with_item(flex::Item::new().with_component(Html::with(html! {
                    p { "Columna 1"}
                })))
                .with_item(flex::Item::new().with_component(top_menu))
                .with_item(flex::Item::new().with_component(Html::with(html! {
                    p { "Columna 3"}
                }))),
        )
        .with_template("admin")
        .render()
}
