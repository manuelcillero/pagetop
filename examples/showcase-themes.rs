use pagetop::prelude::*;
use pagetop_bootsier::theme::*;

include_locales!(LOC from "examples/locale");

struct ShowcaseThemes;

#[async_trait]
impl Extension for ShowcaseThemes {
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
        // Algunas opciones llevan icono; en el desplegable, las que no lo tienen dejan su hueco.
        let main_menu = Nav::new()
            .with_layout(nav::Layout::Center)
            .with_prop(FlexItem::new().with_grow(flex::ItemGrow::Is1))
            .with_item(
                nav::Item::link(Lc::t("showcase_menu_home", &LOC), "/")
                    .with_active(true)
                    .with_icon_start(Icon::with(icon::Core::Home)),
            )
            .with_item(nav::Item::dropdown(
                Dropdown::new()
                    .with_title(Lc::t("showcase_menu_components", &LOC))
                    .with_item(dropdown::Item::header(Lc::t("showcase_menu_forms", &LOC)))
                    .with_item(
                        dropdown::Item::link(Lc::t("showcase_menu_fields", &LOC), "#")
                            .with_icon_start(Icon::with(ShowcaseIcon::Pencil)),
                    )
                    .with_item(dropdown::Item::link(
                        Lc::t("showcase_menu_buttons", &LOC),
                        "#",
                    ))
                    .with_item(dropdown::Item::divider())
                    .with_item(dropdown::Item::header(Lc::t("showcase_menu_others", &LOC)))
                    .with_item(
                        dropdown::Item::link(Lc::t("showcase_menu_messages", &LOC), "#")
                            .with_icon_start(Icon::with(ShowcaseIcon::Chat))
                            .with_icon_end(Icon::with(icon::Core::ChevronRight)),
                    )
                    .with_item(
                        dropdown::Item::link(Lc::t("showcase_menu_disabled", &LOC), "#")
                            .with_disabled(true),
                    ),
            ))
            .with_item(
                nav::Item::link_blank(Lc::t("showcase_menu_docs", &LOC), "https://docs.rs/pagetop")
                    .with_icon_end(Icon::with(ShowcaseIcon::External)),
            )
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
        routes.public_route("/", web::get(showcase_themes))
    }
}

async fn showcase_themes(request: HttpRequest) -> Result<Markup, ErrorPage> {
    let mut page = Page::new(request)
        .with_title(Lc::t("showcase_title", &LOC))
        .with_child(fields_block())
        .with_child(checks_block())
        .with_child(tables_block())
        .with_child(buttons_block())
        .with_child(icons_block())
        .with_child(tiles_block())
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

    let mut row = Flex::buttons();
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

// Conjunto de iconos propio del ejemplo, para mostrar cómo se integra uno: basta implementar
// `From<ShowcaseIcon> for icon::Source` para pasarlos a `Icon::with()`. Son SVG en línea y se ven
// igual con cualquier tema, a diferencia de los de `icon::Core`, que cada tema puede sustituir por
// los de su propio conjunto (Bootsier, por los de Bootstrap Icons).
#[derive(Clone, Copy)]
enum ShowcaseIcon {
    AlertCircle,
    AlignCenter,
    AlignJustify,
    AlignLeft,
    AlignRight,
    Bell,
    Bold,
    Chat,
    CheckCircle,
    Eye,
    EyeOff,
    External,
    File,
    Filter,
    Folder,
    Image,
    Italic,
    Lightbulb,
    Pencil,
    Pin,
    Plug,
    Search,
    Tag,
    Underline,
}

impl From<ShowcaseIcon> for icon::Source {
    fn from(icon: ShowcaseIcon) -> Self {
        use ShowcaseIcon::*;

        let shapes = match icon {
            AlertCircle => html! {
                circle cx="8" cy="8" r="6.5" {}
                path d="M8 4.75v3.75M8 11.25v.01" {}
            },
            AlignCenter => html! { path d="M2 3.5h12M4 6.5h8M2 9.5h12M4 12.5h8" {} },
            AlignJustify => html! { path d="M2 3.5h12M2 6.5h12M2 9.5h12M2 12.5h12" {} },
            AlignLeft => html! { path d="M2 3.5h12M2 6.5h8M2 9.5h12M2 12.5h8" {} },
            AlignRight => html! { path d="M2 3.5h12M6 6.5h8M2 9.5h12M6 12.5h8" {} },
            Bell => html! { path d="M4 11.5V7a4 4 0 0 1 8 0v4.5l1 1H3zM6.5 14.5h3" {} },
            Bold => html! {
                path d="M4.5 2.5H9a2.75 2.75 0 0 1 0 5.5H4.5zM4.5 8h5a2.75 2.75 0 0 1 0 5.5h-5z" {}
            },
            Chat => html! { path d="M2.5 3.5h11V11H7l-3 2.5V11H2.5z" {} },
            CheckCircle => html! {
                circle cx="8" cy="8" r="6.5" {}
                path d="M5 8.25l2 2 4-4" {}
            },
            Eye => html! {
                path d="M1.5 8S4 3.5 8 3.5 14.5 8 14.5 8 12 12.5 8 12.5 1.5 8 1.5 8z" {}
                circle cx="8" cy="8" r="2" {}
            },
            EyeOff => html! {
                path d="M1.5 8S4 3.5 8 3.5 14.5 8 14.5 8 12 12.5 8 12.5 1.5 8 1.5 8z" {}
                circle cx="8" cy="8" r="2" {}
                path d="M2.5 2.5l11 11" {}
            },
            External => html! { path d="M9 2.5h4.5V7M13.5 2.5l-6 6M11.5 9.5v4h-9v-9h4" {} },
            File => html! { path d="M3.5 1.5h6l3 3v10h-9zM9.5 1.5v3h3" {} },
            Filter => html! { path d="M2 3h12l-4.5 5.5V13l-3-1.5v-3z" {} },
            Folder => html! { path d="M1.5 3.5H6L7.5 5h7v7.5h-13z" {} },
            Image => html! {
                rect x="1.5" y="2.5" width="13" height="11" rx="1.5" {}
                circle cx="10.5" cy="6" r="1.25" {}
                path d="M1.5 11.5l4-4 3 3 2-2 4 4" {}
            },
            Italic => html! { path d="M7 2.5h5M4 13.5h5M10 2.5l-4 11" {} },
            Lightbulb => html! {
                path d="M6 11c0-.9-.4-1.5-1.2-2.3a4.25 4.25 0 1 1 6.4 0C10.4 9.5 10 10.1 10 11" {}
                path d="M6 11h4M6.75 14h2.5" {}
            },
            Pencil => html! { path d="M10.5 2.5l3 3L6 13H3v-3zM9 4l3 3" {} },
            Pin => html! { path d="M6 1.5h4M7 1.5v4L4.5 8.5h7L9 5.5v-4M8 8.5v6" {} },
            Plug => {
                html! { path d="M5.5 1.5v3M10.5 1.5v3M3.5 4.5h9V7a4.5 4.5 0 0 1-9 0zM8 11.5v3" {} }
            }
            Search => html! {
                circle cx="7" cy="7" r="4.5" {}
                path d="M10.5 10.5l3.5 3.5" {}
            },
            Tag => html! {
                path d="M2.5 2.5h5l6 6-5 5-6-6z" {}
                circle cx="5.5" cy="5.5" r="1" {}
            },
            Underline => html! { path d="M4.5 2.5V7a3.5 3.5 0 0 0 7 0V2.5M3 14.5h10" {} },
        };
        icon::Source::svg(
            "0 0 16 16",
            html! {
                g
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                {
                    (shapes)
                }
            },
        )
    }
}

// Iconos en sus distintos usos: tamaños y colores, tarjetas de estado, barras de botones sólo con
// icono, botones con icono y texto, y una tabla. Mezcla el conjunto propio con el de PageTop.
fn icons_block() -> Block {
    use ShowcaseIcon::*;

    let row = || {
        Flex::new()
            .with_wrap(flex::Behavior::Wrap)
            .with_align(align::Items::Center)
            .with_gap(align::Gap::Both(UnitValue::RelRem(0.75)))
            .with_prop(Margin::new().with_bottom(UnitValue::RelRem(1.5)))
    };
    let caption = |key: &'static str| {
        Html::with(move |cx| {
            html! {
                p { (Lc::t(key, &LOC).using(cx)) }
            }
        })
    };
    // Botón sólo con icono: la etiqueta del icono es el nombre accesible del botón, y el texto
    // emergente la muestra al pasar por encima.
    let icon_button = |source: icon::Source, key: &'static str, style: button::Style| {
        Button::plain(Lc::none())
            .with_style(style)
            .with_title(Lc::t(key, &LOC))
            .with_icon_start(Icon::with(source).with_label(Lc::t(key, &LOC)))
    };

    // El tamaño sigue al de la fuente; el color, a la intención.
    let mut sizes = row();
    for size in [1.0, 1.5, 2.0, 2.5, 3.0] {
        sizes = sizes.with_child(Icon::with(Lightbulb).with_size(UnitValue::RelRem(size)));
    }
    let mut colors = row();
    for (source, intent) in [
        (icon::Source::from(Pin), Intent::Primary),
        (Bell.into(), Intent::Neutral),
        (Chat.into(), Intent::Info),
        (CheckCircle.into(), Intent::Success),
        (AlertCircle.into(), Intent::Warning),
        (icon::Core::Close.into(), Intent::Severe),
    ] {
        colors = colors.with_child(
            Icon::with(source)
                .with_size(UnitValue::RelRem(1.5))
                .with_intent(intent),
        );
    }

    // Tarjetas de estado: un icono grande de color, el nombre del aparato y su estado.
    let card = |source: ShowcaseIcon, intent: Intent, title: &'static str, badge: Badge| {
        Flex::new()
            .with_direction(flex::Direction::Column)
            .with_align(align::Items::Start)
            .with_gap(align::Gap::Both(UnitValue::RelRem(0.5)))
            .with_prop(Margin::new().with_bottom(UnitValue::RelRem(1.5)))
            .with_child(
                Icon::with(source)
                    .with_size(UnitValue::RelRem(2.5))
                    .with_intent(intent),
            )
            .with_child(Lc::t(title, &LOC))
            .with_child(badge)
    };
    let cards = columns()
        .with_child(card(
            Lightbulb,
            Intent::Warning,
            "showcase_card_switch",
            Badge::success(Lc::t("showcase_card_active", &LOC)),
        ))
        .with_child(card(
            Plug,
            Intent::Primary,
            "showcase_card_outlet",
            Badge::info(Lc::n("32 %")),
        ))
        .with_child(card(
            AlertCircle,
            Intent::Severe,
            "showcase_card_battery",
            Badge::severe(Lc::t("showcase_card_low", &LOC)),
        ));

    // Barra de acciones de una nota, con botones de estilo enlace. Su propio relleno ya los separa,
    // así que la fila no añade hueco entre ellos.
    let mut actions = row().with_gap(align::Gap::Both(UnitValue::Zero));
    for (source, key) in [
        (Pencil, "showcase_edit"),
        (Tag, "showcase_tag"),
        (Eye, "showcase_view"),
        (Chat, "showcase_comment_action"),
        (Bell, "showcase_notify"),
        (Pin, "showcase_pin"),
        (EyeOff, "showcase_hide"),
    ] {
        actions = actions.with_child(icon_button(source.into(), key, button::Style::Link));
    }

    // Grupos de formato de texto, con botones pequeños de contorno.
    let format_group = |items: &[(ShowcaseIcon, &'static str)]| {
        let mut group = Flex::new().with_gap(align::Gap::Both(UnitValue::RelRem(0.25)));
        for (source, key) in items {
            group = group.with_child(
                icon_button(
                    (*source).into(),
                    key,
                    button::Style::Outline(Intent::Neutral),
                )
                .with_size(button::Size::Small),
            );
        }
        group
    };
    let format = row()
        .with_child(format_group(&[
            (AlignLeft, "showcase_align_left"),
            (AlignCenter, "showcase_align_center"),
            (AlignRight, "showcase_align_right"),
            (AlignJustify, "showcase_align_justify"),
        ]))
        .with_child(format_group(&[
            (Bold, "showcase_bold"),
            (Italic, "showcase_italic"),
            (Underline, "showcase_underline"),
        ]));

    // Botones con icono y texto: al principio, al final o a los dos lados de la etiqueta.
    let mut buttons = row();
    for (start, key, end, style) in [
        (
            Some(icon::Source::from(icon::Core::Plus)),
            "showcase_new",
            None::<icon::Source>,
            button::Style::Solid(Intent::Primary),
        ),
        (
            Some(Search.into()),
            "showcase_search",
            None,
            button::Style::Outline(Intent::Primary),
        ),
        (
            Some(Filter.into()),
            "showcase_filter",
            None,
            button::Style::Outline(Intent::Neutral),
        ),
        (
            Some(Image.into()),
            "showcase_insert_image",
            None,
            button::Style::Outline(Intent::Neutral),
        ),
        (
            Some(icon::Core::Check.into()),
            "showcase_save",
            None,
            button::Style::Solid(Intent::Success),
        ),
        (
            Some(icon::Core::Close.into()),
            "showcase_discard",
            None,
            button::Style::Outline(Intent::Severe),
        ),
        (
            None,
            "showcase_next",
            Some(icon::Core::ChevronRight.into()),
            button::Style::Outline(Intent::Primary),
        ),
        (
            Some(Folder.into()),
            "showcase_move_to",
            Some(icon::Core::ChevronDown.into()),
            button::Style::Outline(Intent::Neutral),
        ),
    ] {
        buttons = buttons.with_child(
            Button::plain(Lc::t(key, &LOC))
                .with_style(style)
                .with_icon_start(start.map(Icon::with))
                .with_icon_end(end.map(Icon::with)),
        );
    }

    // Menú vertical, como una barra lateral, y menú desplegable de acciones. Las opciones sin icono
    // dejan su hueco para que las etiquetas queden alineadas; el icono final va junto al borde.
    let sidebar = Nav::new()
        .with_layout(nav::Layout::Vertical)
        .with_prop(FlexItem::new().with_size(flex::ItemSize::Custom(UnitValue::RelRem(14.0))))
        .with_item(
            nav::Item::link(Lc::t("showcase_nav_start", &LOC), "#")
                .with_icon_start(Icon::with(Lightbulb)),
        )
        .with_item(
            nav::Item::link(Lc::t("showcase_nav_styles", &LOC), "#")
                .with_icon_start(Icon::with(Pencil)),
        )
        .with_item(nav::Item::link(Lc::t("showcase_nav_grid", &LOC), "#"))
        .with_item(
            nav::Item::link(Lc::t("showcase_nav_folders", &LOC), "#")
                .with_icon_start(Icon::with(Folder))
                .with_icon_end(Icon::with(icon::Core::ChevronRight)),
        )
        .with_item(
            nav::Item::link(Lc::t("showcase_nav_templates", &LOC), "#")
                .with_disabled(true)
                .with_icon_start(Icon::with(File))
                .with_icon_end(Icon::with(Pin)),
        );
    let actions_menu = Dropdown::new()
        .with_title(Lc::t("showcase_actions", &LOC))
        .with_button_style(button::Style::Outline(Intent::Neutral))
        .with_item(
            dropdown::Item::button(Lc::t("showcase_edit", &LOC))
                .with_icon_start(Icon::with(Pencil)),
        )
        .with_item(
            dropdown::Item::button(Lc::t("showcase_tag", &LOC)).with_icon_start(Icon::with(Tag)),
        )
        .with_item(dropdown::Item::button(Lc::t("showcase_duplicate", &LOC)))
        .with_item(
            dropdown::Item::button(Lc::t("showcase_pin", &LOC))
                .with_icon_start(Icon::with(Pin))
                .with_icon_end(Icon::with(icon::Core::Check)),
        )
        .with_item(dropdown::Item::divider())
        .with_item(
            dropdown::Item::button(Lc::t("showcase_delete", &LOC))
                .with_icon_start(Icon::with(icon::Core::Close).with_intent(Intent::Severe)),
        );
    let menus = Flex::new()
        .with_wrap(flex::Behavior::Wrap)
        .with_align(align::Items::Start)
        .with_gap(align::Gap::Both(UnitValue::RelRem(2.0)))
        .with_prop(Margin::new().with_bottom(UnitValue::RelRem(1.5)))
        .with_child(sidebar)
        .with_child(actions_menu);

    // Tabla con el tipo de cada fila en un icono y las acciones como botones sólo con icono.
    let mut files = Table::new()
        .with_column(Lc::t("showcase_col_name", &LOC))
        .with_column(Lc::t("showcase_col_type", &LOC))
        .with_column(Lc::t("showcase_col_actions", &LOC));
    for (name, folder) in [
        ("Backgrounds", true),
        ("Shadows", true),
        ("Document.txt", false),
        ("Projects.draft", false),
    ] {
        let (source, kind) = match folder {
            true => (Folder, "showcase_type_folder"),
            false => (File, "showcase_type_document"),
        };
        files.alter_row(
            table::Row::new()
                .with_cell(table::Cell::new(
                    Flex::new()
                        .with_align(align::Items::Center)
                        .with_gap(align::Gap::Both(UnitValue::RelRem(0.5)))
                        .with_child(Icon::with(source).with_intent(Intent::Neutral))
                        .with_child(Lc::n(name)),
                ))
                .with_cell(Lc::t(kind, &LOC))
                .with_cell(table::Cell::new(
                    Flex::new()
                        .with_child(icon_button(
                            Eye.into(),
                            "showcase_view",
                            button::Style::Link,
                        ))
                        .with_child(icon_button(
                            Pencil.into(),
                            "showcase_edit",
                            button::Style::Link,
                        ))
                        .with_child(icon_button(
                            icon::Core::Close.into(),
                            "showcase_delete",
                            button::Style::Link,
                        )),
                )),
        );
    }

    Block::new()
        .with_title(Lc::t("showcase_block_icons", &LOC))
        .with_child(caption("showcase_icons_sizes"))
        .with_child(sizes)
        .with_child(colors)
        .with_child(caption("showcase_icons_cards"))
        .with_child(cards)
        .with_child(caption("showcase_icons_toolbar"))
        .with_child(actions)
        .with_child(format)
        .with_child(caption("showcase_icons_buttons"))
        .with_child(buttons)
        .with_child(caption("showcase_icons_menus"))
        .with_child(menus)
        .with_child(caption("showcase_icons_table"))
        .with_child(files)
}

// Un recuadro de acceso por color de acento, en la rejilla habitual de una portada (una, dos y tres
// columnas). Alternan los casos: con la imagen de fondo que trae PageTop para `Intro` (difuminada
// bajo el color) o sólo con el color, y con o sin descripción. Al final, uno sin icono y otro
// deshabilitado.
fn tiles_block() -> Block {
    use Accent::*;

    let tracks = |n| grid::Tracks::repeat(n, grid::AxisTrack::Fraction(1.0));
    let mut grid = Grid::new()
        .with_columns(tracks(1))
        .with_columns_at(Breakpoint::Md, tracks(2))
        .with_columns_at(Breakpoint::Lg, tracks(3))
        .with_gap(align::Gap::Both(UnitValue::RelRem(1.5)));
    let accents = [
        (Blue, "Blue", Icon::with(ShowcaseIcon::Folder)),
        (Orange, "Orange", Icon::with(ShowcaseIcon::Image)),
        (Green, "Green", Icon::with(ShowcaseIcon::CheckCircle)),
        (Purple, "Purple", Icon::with(ShowcaseIcon::Lightbulb)),
        (Red, "Red", Icon::with(ShowcaseIcon::AlertCircle)),
        (Teal, "Teal", Icon::with(ShowcaseIcon::Tag)),
        (Indigo, "Indigo", Icon::with(ShowcaseIcon::Chat)),
        (Pink, "Pink", Icon::with(ShowcaseIcon::Bell)),
        (Yellow, "Yellow", Icon::with(ShowcaseIcon::Pin)),
        (Cyan, "Cyan", Icon::with(ShowcaseIcon::Plug)),
        (Lime, "Lime", Icon::with(ShowcaseIcon::Eye)),
        (Magenta, "Magenta", Icon::with(ShowcaseIcon::Search)),
    ];
    for (i, (accent, name, icon)) in accents.into_iter().enumerate() {
        let tile = Tile::link(Lc::n(name), "#")
            .with_icon(icon)
            .with_accent(accent);
        grid = grid.with_child(match i % 3 {
            0 => tile
                .with_description(Lc::t("showcase_tile_image", &LOC))
                .with_image(image::Source::plain("/pagetop/img/intro-header-sm.jpg")),
            1 => tile.with_description(Lc::t("showcase_tile_color", &LOC)),
            _ => tile,
        });
    }
    grid = grid.with_child(
        Tile::link(Lc::n("Gray"), "#")
            .with_description(Lc::t("showcase_tile_no_icon", &LOC))
            .with_accent(Gray),
    );
    grid = grid.with_child(
        Tile::link(Lc::n("Disabled"), "#")
            .with_description(Lc::t("showcase_tile_disabled", &LOC))
            .with_icon(Icon::with(ShowcaseIcon::EyeOff))
            .with_disabled(true),
    );

    Block::new()
        .with_title(Lc::t("showcase_block_tiles", &LOC))
        .with_child(grid)
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
    Application::prepare(&ShowcaseThemes).await.run().await
}
