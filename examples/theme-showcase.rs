use pagetop::prelude::*;
use pagetop_bootsier::theme::*;

include_locales!(LOC from "examples/locale");

struct ThemeShowcase;

#[async_trait]
impl Extension for ThemeShowcase {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_bootsier::Bootsier]
    }

    async fn initialize(&self) -> Result<(), BoxError> {
        let brand = Brand::new()
            .with_image(
                // Recuadro en el color primario de Bootsier (petróleo, `#0e6973`), con la línea
                // clara que elige el logotipo por contraste. Se ve igual en los dos modos de color.
                Image::with(image::Source::logo(PageTopSvg::TileRGB(14, 105, 115)))
                    .with_size(image::Size::Both(UnitValue::Px(36))),
            )
            .with_title(Lc::n("PageTop"))
            .with_route(Route::from("/"));

        // Menú completo: enlace activo, desplegable con secciones, enlace externo y deshabilitado.
        // Ocupa el espacio libre entre la marca y el menú de la cuenta, con sus opciones centradas.
        let main_menu = Nav::new()
            .with_layout(nav::Layout::Center)
            .with_prop(FlexItem::new().with_grow(flex::ItemGrow::Is1))
            .with_item(nav::Item::link(Lc::t("showcase_menu_home", &LOC), "/").with_active(true))
            .with_item(nav::Item::dropdown(
                Dropdown::new()
                    .with_title(Lc::t("showcase_menu_components", &LOC))
                    .with_item(dropdown::Item::header(Lc::t("showcase_menu_forms", &LOC)))
                    .with_item(dropdown::Item::link(
                        Lc::t("showcase_menu_fields", &LOC),
                        "#",
                    ))
                    .with_item(dropdown::Item::link(
                        Lc::t("showcase_menu_buttons", &LOC),
                        "#",
                    ))
                    .with_item(dropdown::Item::divider())
                    .with_item(dropdown::Item::header(Lc::t("showcase_menu_others", &LOC)))
                    .with_item(dropdown::Item::link(
                        Lc::t("showcase_menu_messages", &LOC),
                        "#",
                    ))
                    .with_item(
                        dropdown::Item::link(Lc::t("showcase_menu_disabled", &LOC), "#")
                            .with_disabled(true),
                    ),
            ))
            .with_item(nav::Item::link_blank(
                Lc::t("showcase_menu_docs", &LOC),
                "https://docs.rs/pagetop",
            ))
            .with_item(
                nav::Item::link(Lc::t("showcase_menu_disabled", &LOC), "#").with_disabled(true),
            );

        // Menú de la cuenta al final de la barra y, con Bootsier, el selector de modo de color
        // (claro, oscuro o automático).
        let mut user_menu = Nav::new()
            .with_prop(FlexItem::push_end())
            .with_item(nav::Item::link(Lc::t("showcase_menu_sign_up", &LOC), "#"))
            .with_item(nav::Item::link(Lc::t("showcase_menu_login", &LOC), "#"));
        if is_bootsier() {
            user_menu = user_menu.with_item(bs::theme_toggle());
        }

        let header = Navbar::brand_left(brand)
            .with_expand(Breakpoint::Lg)
            .with_item(navbar::Item::nav(main_menu))
            .with_item(navbar::Item::nav(user_menu));

        InRegion::Global(&CoreRegions::Header).add(header);
        Ok(())
    }

    fn configure_router(&self, routes: Routes) -> Routes {
        routes.public_route("/", web::get(theme_showcase))
    }
}

async fn theme_showcase(request: HttpRequest) -> Result<Markup, ErrorPage> {
    let mut page = Page::new(request)
        .with_title(Lc::t("showcase_title", &LOC))
        .with_child(fields_block())
        .with_child(checks_block())
        .with_child(tables_block())
        .with_child(buttons_block())
        .with_child(misc_block());

    // Un mensaje de cada nivel, que muestra el listado del bloque de tablas entre su botón y la
    // tabla, como haría con el aviso de un borrado.
    page.push_message(MessageLevel::Info, Lc::t("showcase_msg_info", &LOC));
    page.push_message(MessageLevel::Warning, Lc::t("showcase_msg_warning", &LOC));
    page.push_message(MessageLevel::Error, Lc::t("showcase_msg_error", &LOC));

    page.render().await
}

// Bootsier aporta piezas propias que otros temas no tienen: el selector de modo de color, las
// etiquetas flotantes y los botones de los colores claro y oscuro de su paleta.
fn is_bootsier() -> bool {
    global::SETTINGS.app.theme.eq_ignore_ascii_case("bootsier")
}

// Rejilla de una columna en pantallas pequeñas, dos a partir de `Md` y tres a partir de `Lg`. Sin
// hueco entre filas: ya separa los campos su propio margen.
fn columns() -> Grid {
    let tracks = |n| grid::Tracks::repeat(n, grid::AxisTrack::Fraction(1.0));
    Grid::new()
        .with_columns(tracks(1))
        .with_columns_at(Breakpoint::Md, tracks(2))
        .with_columns_at(Breakpoint::Lg, tracks(3))
        .with_gap(align::Gap::Distinct {
            row: UnitValue::None,
            column: UnitValue::RelRem(1.5),
        })
}

// Campos de texto y listas en sus distintos estados.
fn fields_block() -> Block {
    let mut grid = columns()
        .with_child(
            form::input::Field::text()
                .with_name("name")
                .with_label(Lc::t("showcase_name", &LOC))
                .with_value("Ana García")
                .with_help_text(Lc::t("showcase_name_help", &LOC))
                .with_required(true),
        )
        .with_child(
            form::input::Field::email()
                .with_name("email")
                .with_label(Lc::t("showcase_email", &LOC))
                .with_placeholder(Lc::t("showcase_email_placeholder", &LOC)),
        )
        .with_child(
            form::input::Field::text()
                .with_name("focus")
                .with_label(Lc::t("showcase_focus", &LOC))
                .with_value("1200")
                .with_autofocus(true),
        )
        .with_child(
            form::select::Field::new()
                .with_name("font")
                .with_label(Lc::t("showcase_font", &LOC))
                .with_item(form::select::Item::new(
                    "playfair",
                    Lc::n("Playfair Display"),
                ))
                .with_item(form::select::Item::new("source", Lc::n("Source Sans 3")))
                .with_item(form::select::Item::new("roboto", Lc::n("Roboto"))),
        )
        .with_child(
            form::input::Field::text()
                .with_name("disabled")
                .with_label(Lc::t("showcase_disabled", &LOC))
                .with_value("Ana García")
                .with_disabled(true),
        )
        .with_child(
            form::input::Field::email()
                .with_name("readonly")
                .with_label(Lc::t("showcase_readonly", &LOC))
                .with_value("ana@example.com")
                .with_readonly(true),
        )
        .with_child(
            form::select::Field::new()
                .with_name("disabled_font")
                .with_label(Lc::t("showcase_disabled_select", &LOC))
                .with_item(form::select::Item::new(
                    "playfair",
                    Lc::n("Playfair Display"),
                ))
                .with_disabled(true),
        )
        .with_child(
            form::Textarea::new()
                .with_name("comment")
                .with_label(Lc::t("showcase_comment", &LOC))
                .with_placeholder(Lc::t("showcase_comment_placeholder", &LOC)),
        );
    if is_bootsier() {
        grid = grid.with_child(
            form::input::Field::text()
                .with_name("floating")
                .with_label(Lc::t("showcase_floating", &LOC))
                .with_value("Ana García")
                .with_floating_label(true),
        );
    }

    // El último campo no tiene margen inferior y, si es el más alto de su fila, la rejilla acaba en
    // su borde. El margen de la rejilla repone ese espacio, como el que dejan debajo los demás
    // bloques.
    Block::new()
        .with_title(Lc::t("showcase_block_fields", &LOC))
        .with_child(grid.with_prop(Margin::new().with_bottom(UnitValue::RelRem(1.0))))
}

// Casillas, interruptores y botones de opción, marcados, sin marcar y deshabilitados.
fn checks_block() -> Block {
    Block::new()
        .with_title(Lc::t("showcase_block_checks", &LOC))
        .with_child(
            columns()
                .with_child(
                    form::Fieldset::new()
                        .with_legend(Lc::t("showcase_checks", &LOC))
                        .with_child(
                            form::Checkbox::new()
                                .with_name("check_off")
                                .with_label(Lc::t("showcase_check_off", &LOC)),
                        )
                        .with_child(
                            form::Checkbox::new()
                                .with_name("check_on")
                                .with_label(Lc::t("showcase_check_on", &LOC))
                                .with_checked(true),
                        )
                        .with_child(
                            form::Checkbox::new()
                                .with_name("check_disabled")
                                .with_label(Lc::t("showcase_check_disabled", &LOC))
                                .with_disabled(true),
                        ),
                )
                .with_child(
                    form::Fieldset::new()
                        .with_legend(Lc::t("showcase_switches", &LOC))
                        .with_child(
                            form::Checkbox::switch()
                                .with_name("switch_off")
                                .with_label(Lc::t("showcase_switch_off", &LOC)),
                        )
                        .with_child(
                            form::Checkbox::switch()
                                .with_name("switch_on")
                                .with_label(Lc::t("showcase_switch_on", &LOC))
                                .with_checked(true),
                        )
                        .with_child(
                            form::Checkbox::switch()
                                .with_name("switch_disabled")
                                .with_label(Lc::t("showcase_switch_disabled", &LOC))
                                .with_disabled(true),
                        ),
                )
                .with_child(
                    form::Fieldset::new()
                        .with_legend(Lc::t("showcase_options", &LOC))
                        .with_child(
                            form::radio::Field::new()
                                .with_name("size")
                                .with_item(form::radio::Item::new(
                                    "small",
                                    Lc::t("showcase_option_small", &LOC),
                                ))
                                .with_item(
                                    form::radio::Item::new(
                                        "medium",
                                        Lc::t("showcase_option_medium", &LOC),
                                    )
                                    .with_checked(true),
                                )
                                .with_item(
                                    form::radio::Item::new(
                                        "large",
                                        Lc::t("showcase_option_large", &LOC),
                                    )
                                    .with_disabled(true),
                                )
                                .with_help_text(Lc::t("showcase_options_help", &LOC)),
                        ),
                ),
        )
}

// Filtro de listado, listado completo (botón de acción, tabla con cabeceras ordenables y
// paginador) y tabla vacía.
fn tables_block() -> Block {
    // Columna ordenable; `dir` marca la que determina el orden vigente.
    let sortable = |key: &'static str, sort: &str, dir: Option<SortDir>| {
        table::Column::new(Lc::t(key, &LOC))
            .with_sort(table::SortLink::new(util::join!("/?sort=", sort)).with_dir(dir))
    };

    let mut users = Table::new()
        .with_column(sortable("showcase_col_user", "user", Some(SortDir::Asc)))
        .with_column(sortable("showcase_col_email", "email", None))
        .with_column(Lc::t("showcase_col_status", &LOC))
        .with_column(sortable("showcase_col_last_access", "access", None))
        .with_column(Lc::t("showcase_col_actions", &LOC));
    for (user, email, status, access) in [
        ("ana", "ana@example.com", "active", "2026-11-14 09:12"),
        ("bruno", "bruno@example.com", "pending", "-"),
        ("carmen", "carmen@example.com", "active", "2026-11-13 18:40"),
        ("david", "david@example.com", "blocked", "2026-10-02 11:05"),
    ] {
        let badge = match status {
            "active" => Badge::success(Lc::t("showcase_status_active", &LOC)),
            "pending" => Badge::warning(Lc::t("showcase_status_pending", &LOC)),
            _ => Badge::severe(Lc::t("showcase_status_blocked", &LOC)),
        };
        users.alter_row(
            table::Row::new()
                .with_cell(user)
                .with_cell(email)
                .with_cell(table::Cell::new(badge))
                .with_cell(access)
                .with_cell(table::Cell::new(
                    Button::anchor(Lc::t("showcase_edit", &LOC), "#")
                        .with_style(button::Style::Link),
                )),
        );
    }

    let list = ListView::new()
        .with_id("showcase-users")
        .with_action(
            Button::anchor(Lc::t("showcase_new_user", &LOC), "#")
                .with_style(button::Style::Solid(Intent::Primary)),
        )
        .with_table(users)
        .with_pager(
            Pager::new()
                .with_base_path("/")
                .with_current_page(1)
                .with_items_per_page(4)
                .with_total_items(14),
        );

    // Sin filas, la tabla muestra su mensaje predeterminado.
    let empty = Table::new()
        .with_column(Lc::t("showcase_col_user", &LOC))
        .with_column(Lc::t("showcase_col_email", &LOC))
        .with_column(Lc::t("showcase_col_status", &LOC));

    Block::new()
        .with_title(Lc::t("showcase_block_tables", &LOC))
        .with_child(Html::with(|cx| {
            html! {
                p { (Lc::t("showcase_filter_hint", &LOC).using(cx)) }
            }
        }))
        // El filtro va fuera del listado: si se reemplazara con él, el campo de búsqueda perdería
        // el foco al actualizar sólo el listado.
        .with_child(
            ListFilter::new()
                .with_filter_action("/")
                .with_field(
                    form::input::Field::search()
                        .with_name("q")
                        .with_label(Lc::t("showcase_search", &LOC))
                        .with_placeholder(Lc::t("showcase_search_placeholder", &LOC)),
                )
                .with_field(
                    form::select::Field::new()
                        .with_name("status")
                        .with_label(Lc::t("showcase_status", &LOC))
                        .with_item(form::select::Item::new(
                            "",
                            Lc::t("showcase_status_any", &LOC),
                        ))
                        .with_item(form::select::Item::new(
                            "active",
                            Lc::t("showcase_status_active", &LOC),
                        ))
                        .with_item(form::select::Item::new(
                            "blocked",
                            Lc::t("showcase_status_blocked", &LOC),
                        )),
                )
                .with_field(
                    form::Checkbox::new()
                        .with_name("admins")
                        .with_label(Lc::t("showcase_only_admins", &LOC)),
                ),
        )
        .with_child(list)
        .with_child(Html::with(|cx| {
            html! {
                p { (Lc::t("showcase_empty_table", &LOC).using(cx)) }
            }
        }))
        .with_child(empty)
}

// Botones macizos y de contorno de todas las intenciones.
fn buttons_block() -> Block {
    Block::new()
        .with_title(Lc::t("showcase_block_buttons", &LOC))
        .with_child(
            Flex::new()
                .with_direction(flex::Direction::Column)
                .with_gap(align::Gap::Both(UnitValue::RelRem(0.5)))
                .with_prop(Margin::new().with_bottom(UnitValue::RelRem(1.5)))
                .with_child(buttons(false))
                .with_child(buttons(true)),
        )
}

// Una fila con un botón por intención. Con Bootsier, también los colores claro y oscuro de su
// paleta, que no corresponden a ninguna intención. El estilo enlace sólo va en la fila maciza.
fn buttons(outline: bool) -> Flex {
    let style = |intent| match outline {
        false => button::Style::Solid(intent),
        true => button::Style::Outline(intent),
    };

    let mut row = Flex::new()
        .with_wrap(flex::Behavior::Wrap)
        .with_gap(align::Gap::Both(UnitValue::RelRem(0.5)));
    for (label, intent) in [
        ("Primary", Intent::Primary),
        ("Neutral", Intent::Neutral),
        ("Info", Intent::Info),
        ("Success", Intent::Success),
        ("Warning", Intent::Warning),
        ("Severe", Intent::Severe),
    ] {
        row = row.with_child(Button::plain(Lc::n(label)).with_style(style(intent)));
    }
    if is_bootsier() {
        for (label, color) in [
            ("Light", BootsierColors::Light),
            ("Dark", BootsierColors::Dark),
        ] {
            row = row.with_child(
                Button::plain(Lc::n(label))
                    .with_style(style(Intent::Neutral))
                    .with_color(color),
            );
        }
    }
    if !outline {
        row = row.with_child(Button::plain(Lc::n("Link")).with_style(button::Style::Link));
    }
    row
}

// Enlaces, insignias, paginación y mensajes, que también toman los colores del tema.
fn misc_block() -> Block {
    let mut badges = Flex::new()
        .with_wrap(flex::Behavior::Wrap)
        .with_gap(align::Gap::Both(UnitValue::RelRem(0.5)));
    for badge in [
        Badge::primary(Lc::n("Primary")),
        Badge::neutral(Lc::n("Neutral")),
        Badge::info(Lc::n("Info")),
        Badge::success(Lc::n("Success")),
        Badge::warning(Lc::n("Warning")),
        Badge::severe(Lc::n("Severe")),
    ] {
        badges = badges.with_child(badge);
    }

    Block::new()
        .with_title(Lc::t("showcase_block_misc", &LOC))
        .with_child(Html::with(|cx| {
            html! {
                p { (Lc::t("showcase_link_text", &LOC).using(cx)) }
            }
        }))
        .with_child(badges)
        // Con 50 páginas y la actual en medio aparecen todas sus piezas: anterior y siguiente, las
        // elipsis de los tramos ocultos a cada lado, el resumen y el formulario para saltar a una
        // página.
        .with_child(
            Pager::new()
                .with_base_path("/")
                .with_current_page(24)
                .with_items_per_page(10)
                .with_total_items(500),
        )
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&ThemeShowcase).await.run().await
}
