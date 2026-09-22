use pagetop::prelude::*;
use pagetop_bootsier::theme::*;

include_locales!(LOC from "examples/locale");

struct NavbarMenus;

#[async_trait]
impl Extension for NavbarMenus {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_bootsier::Bootsier]
    }

    fn configure_router(&self, router: Router) -> Router {
        router.route("/", web::get(navbar_menus))
    }
}

async fn navbar_menus(request: HttpRequest) -> Result<Markup, ErrorPage> {
    let page = Page::new(request)
        .with_assets(demo_title_styles())
        .with_assets(demo_code_styles())
        .with_assets(demo_frame_styles())
        .with_assets(demo_scroll_styles());

    let mut intro = Intro::custom()
        .with_width(intro::Width::Full)
        .with_title(Lc::n("PageTop"))
        .with_slogan(Lc::t("menus_slogan", &LOC))
        .with_child(Html::with(|cx| {
            html! {
                p class="intro-text-lead" { (Lc::t("menus_note", &LOC).using(cx)) }
            }
        }))
        .with_child(menus_block())
        .with_child(brand_block())
        .with_child(container_block())
        .with_child(layout_block())
        .with_child(expand_block())
        .with_child(content_block())
        .with_child(position_block());

    // Demostraciones asociadas a Bootsier: usan sus tipos, o clases y variables CSS de Bootstrap
    // que otros temas no definen. Sólo se muestran con el tema Bootsier activo.
    if is_bootsier() {
        intro = intro
            .with_child(colors_block())
            .with_child(offcanvas_block());
    }

    page.with_child(intro).render().await
}

// **< Menús >**************************************************************************************

fn menus_block() -> Block {
    let full_menu = Navbar::brand_left(demo_brand())
        .with_prop(light_bg())
        .with_expand(Breakpoint::Lg)
        .with_item(navbar::Item::nav(
            Nav::new()
                .with_item(nav::Item::link(Lc::t("menus_item_home", &LOC), "/").with_active(true))
                .with_item(nav::Item::link_blank(
                    Lc::t("menus_item_blank", &LOC),
                    "https://docs.rs/pagetop",
                ))
                .with_item(nav::Item::dropdown(
                    Dropdown::new()
                        .with_title(Lc::t("menus_test_title", &LOC))
                        .with_item(dropdown::Item::header(Lc::t("menus_dev_header", &LOC)))
                        .with_item(dropdown::Item::link(
                            Lc::t("menus_dev_getting_started", &LOC),
                            "/dev/getting-started",
                        ))
                        .with_item(dropdown::Item::link(
                            Lc::t("menus_dev_guides", &LOC),
                            "/dev/guides",
                        ))
                        .with_item(dropdown::Item::link_blank(
                            Lc::t("menus_dev_forum", &LOC),
                            "https://forum.example.dev",
                        ))
                        .with_item(dropdown::Item::divider())
                        .with_item(dropdown::Item::header(Lc::t("menus_sdk_header", &LOC)))
                        .with_item(dropdown::Item::link(
                            Lc::t("menus_sdk_rust", &LOC),
                            "/dev/sdks/rust",
                        ))
                        .with_item(dropdown::Item::link(
                            Lc::t("menus_sdk_js", &LOC),
                            "/dev/sdks/js",
                        ))
                        .with_item(dropdown::Item::link(
                            Lc::t("menus_sdk_python", &LOC),
                            "/dev/sdks/python",
                        ))
                        .with_item(dropdown::Item::divider())
                        .with_item(dropdown::Item::header(Lc::t("menus_plugin_header", &LOC)))
                        .with_item(dropdown::Item::link(
                            Lc::t("menus_plugin_auth", &LOC),
                            "/dev/sdks/rust/plugins/auth",
                        ))
                        .with_item(dropdown::Item::link(
                            Lc::t("menus_plugin_cache", &LOC),
                            "/dev/sdks/rust/plugins/cache",
                        ))
                        .with_item(dropdown::Item::divider())
                        .with_item(dropdown::Item::label(Lc::t("menus_item_label", &LOC)))
                        .with_item(dropdown::Item::link_disabled(
                            Lc::t("menus_item_disabled", &LOC),
                            "#",
                        )),
                ))
                .with_item(nav::Item::link_disabled(
                    Lc::t("menus_item_disabled", &LOC),
                    "#",
                )),
        ))
        .with_item(navbar::Item::nav(
            Nav::new()
                // Empuja este menú (y lo que le siga) al extremo final de la barra.
                .with_prop(FlexItem::push_end())
                .with_item(nav::Item::link(
                    Lc::t("menus_item_sign_up", &LOC),
                    "/auth/sign-up",
                ))
                .with_item(nav::Item::link(
                    Lc::t("menus_item_login", &LOC),
                    "/auth/login",
                )),
        ));

    add_demo(
        Block::new().with_title(Lc::t("menus_block_title_menus", &LOC)),
        Lc::t("menus_title_full", &LOC),
        concat!(
            "Navbar::brand_left(brand)",
            ".with_item(navbar::Item::nav(Nav::new()...with_item(nav::Item::dropdown(...))))",
            ".with_item(navbar::Item::nav(Nav::new().with_prop(FlexItem::push_end())...))",
        ),
        full_menu,
    )
}

// **< Marca de identidad >*************************************************************************

fn brand_block() -> Block {
    // Una barra sin contenidos no se renderiza, así que la marca se añade como un elemento más.
    let brand_variants: [(&str, Brand, &str); 4] = [
        (
            "menus_title_brand_link",
            demo_brand(),
            "navbar::Item::brand(Brand::new().with_title(...).with_route(Route::from(\"/\")))",
        ),
        (
            "menus_title_brand_span",
            demo_brand().with_route(None::<Route>),
            "navbar::Item::brand(Brand::new().with_title(...).with_route(None))",
        ),
        (
            "menus_title_brand_image",
            Brand::new()
                .with_title(Lc::none())
                .with_image(demo_logo())
                .with_route(Route::from("/")),
            "navbar::Item::brand(Brand::new().with_title(Lc::none()).with_image(image))",
        ),
        (
            "menus_title_brand_both",
            demo_brand().with_image(demo_logo()),
            "navbar::Item::brand(Brand::new().with_title(...).with_image(image))",
        ),
    ];

    let mut block = Block::new().with_title(Lc::t("menus_block_title_brand", &LOC));
    for (title_key, brand, code) in brand_variants {
        block = add_demo(
            block,
            Lc::t(title_key, &LOC),
            code,
            Navbar::simple()
                .with_prop(light_bg())
                .with_item(navbar::Item::brand(brand)),
        );
    }
    block
}

// **< Otros contenidos >***************************************************************************

fn content_block() -> Block {
    let gap = align::Gap::Both(UnitValue::RelRem(0.5));

    let search_form = Form::new()
        .with_action("/")
        .with_method(form::Method::Get)
        .with_child(
            Flex::new()
                .with_align(align::Items::Center)
                .with_gap(gap)
                .with_child(
                    form::input::Field::search()
                        .with_name("q")
                        .with_placeholder(Lc::t("menus_search_label", &LOC))
                        // Sin el margen inferior con que algunos temas separan los campos.
                        .with_prop(Margin::new().with_bottom(UnitValue::Zero)),
                )
                .with_child(
                    Button::submit(Lc::t("menus_search_button", &LOC))
                        .with_style(button::Style::Outline(Intent::Success)),
                ),
        );

    let buttons = Form::new().with_child(
        Flex::new()
            .with_align(align::Items::Center)
            .with_gap(gap)
            .with_child(
                Button::plain(Lc::t("menus_button_main", &LOC))
                    .with_style(button::Style::Outline(Intent::Success)),
            )
            .with_child(
                Button::plain(Lc::t("menus_button_small", &LOC))
                    .with_size(button::Size::Small)
                    .with_style(button::Style::Outline(Intent::Neutral)),
            ),
    );

    let mut content_variants: Vec<(&str, Navbar, &str)> = vec![
        (
            "menus_title_form_search",
            Navbar::simple_brand_left(demo_brand()).with_item(navbar::Item::form(search_form)),
            "Navbar::simple_brand_left(brand).with_item(navbar::Item::form(Form::new()...))",
        ),
        (
            "menus_title_form_buttons",
            Navbar::simple().with_item(navbar::Item::form(buttons)),
            "Navbar::simple().with_item(navbar::Item::form(Form::new()...))",
        ),
        (
            "menus_title_text",
            Navbar::simple().with_item(navbar::Item::text(Lc::t("menus_text_inline", &LOC))),
            "Navbar::simple().with_item(navbar::Item::text(Lc::t(...)))",
        ),
        (
            "menus_title_brand_and_text",
            Navbar::simple()
                .with_item(navbar::Item::brand(demo_brand()))
                .with_item(navbar::Item::text(Lc::t("menus_text_inline", &LOC))),
            concat!(
                "Navbar::simple().with_item(navbar::Item::brand(brand))",
                ".with_item(navbar::Item::text(...))",
            ),
        ),
        (
            "menus_title_nav_text",
            Navbar::brand_left(demo_brand())
                .with_expand(Breakpoint::Lg)
                .with_item(navbar::Item::nav(
                    demo_nav().with_prop(Margin::new().with_end(UnitValue::Auto)),
                ))
                .with_item(navbar::Item::text(Lc::t("menus_text_inline", &LOC))),
            concat!(
                "Navbar::brand_left(brand).with_item(navbar::Item::nav(",
                "Nav::new().with_prop(Margin::new().with_end(UnitValue::Auto))...))",
                ".with_item(navbar::Item::text(...))",
            ),
        ),
    ];

    // El núcleo no tiene un componente de grupo de entrada, así que esta demostración usa clases de
    // Bootstrap y sólo se muestra con el tema Bootsier.
    if is_bootsier() {
        let input_group = Nav::new().with_item(nav::Item::html(Html::with(|cx| {
            html! {
                form {
                    div class="input-group" {
                        span class="input-group-text" { "@" }
                        input
                            class="form-control"
                            type="text"
                            placeholder=[Lc::t("menus_group_placeholder", &LOC).lookup(cx)]
                            aria-label=[Lc::t("menus_group_placeholder", &LOC).lookup(cx)];
                    }
                }
            }
        })));
        content_variants.insert(
            2,
            (
                "menus_title_form_group",
                Navbar::simple().with_item(navbar::Item::nav(input_group)),
                "Navbar::simple().with_item(navbar::Item::nav(Nav::new()...nav::Item::html(...)))",
            ),
        );
    }

    let mut block = Block::new().with_title(Lc::t("menus_block_title_content", &LOC));
    for (title_key, navbar, code) in content_variants {
        block = add_demo(
            block,
            Lc::t(title_key, &LOC),
            code,
            navbar.with_prop(light_bg()),
        );
    }
    block
}

// **< Esquemas de color >**************************************************************************

fn colors_block() -> Block {
    let color_variants: [(&str, Vec<PropsOp>, &str); 4] = [
        (
            "menus_title_color_dark",
            vec![
                PropsOp::add_classes(class::Bg::with(BootsierColors::Dark)),
                PropsOp::set("data-bs-theme", "dark"),
            ],
            concat!(
                "Navbar::brand_left(brand)",
                ".with_prop(PropsOp::add_classes(class::Bg::with(BootsierColors::Dark)))",
                ".with_prop(PropsOp::set(\"data-bs-theme\", \"dark\"))",
            ),
        ),
        (
            "menus_title_color_primary",
            vec![
                PropsOp::add_classes(class::Bg::with(BootsierColors::Primary)),
                PropsOp::set("data-bs-theme", "dark"),
            ],
            concat!(
                "Navbar::brand_left(brand)",
                ".with_prop(PropsOp::add_classes(class::Bg::with(BootsierColors::Primary)))",
                ".with_prop(PropsOp::set(\"data-bs-theme\", \"dark\"))",
            ),
        ),
        (
            "menus_title_color_light",
            vec![light_bg()],
            concat!(
                "Navbar::brand_left(brand)",
                ".with_prop(PropsOp::add_classes(class::Bg::with(class::BgColor::BodyTertiary)))",
            ),
        ),
        (
            "menus_title_color_custom",
            vec![PropsOp::add_style("background-color", "#e3f2fd")],
            concat!(
                "Navbar::brand_left(brand)",
                ".with_prop(PropsOp::add_style(\"background-color\", \"#e3f2fd\"))",
            ),
        ),
    ];

    let mut block = Block::new().with_title(Lc::t("menus_block_title_colors", &LOC));
    for (title_key, props, code) in color_variants {
        let mut navbar = Navbar::brand_left(demo_brand())
            .with_expand(Breakpoint::Lg)
            .with_item(navbar::Item::nav(demo_nav()));
        for op in props {
            navbar = navbar.with_prop(op);
        }
        block = add_demo(block, Lc::t(title_key, &LOC), code, navbar);
    }
    block
}

// **< Contenedores >*******************************************************************************

fn container_block() -> Block {
    let navbar = || {
        Navbar::brand_left(demo_brand())
            .with_prop(light_bg())
            .with_expand(Breakpoint::Lg)
            .with_item(navbar::Item::nav(demo_nav()))
    };

    let block = Block::new().with_title(Lc::t("menus_block_title_container", &LOC));
    let block = add_demo(
        block,
        Lc::t("menus_title_container_full", &LOC),
        "Navbar::brand_left(brand)...",
        navbar(),
    );
    add_demo(
        block,
        Lc::t("menus_title_container_max", &LOC),
        concat!(
            "Container::new()",
            ".with_width(container::Width::FluidMax(UnitValue::RelRem(40.0)))",
            ".with_child(Navbar::brand_left(brand)...)",
        ),
        Container::new()
            .with_width(container::Width::FluidMax(UnitValue::RelRem(40.0)))
            .with_child(navbar()),
    )
}

// **< Posición >***********************************************************************************

fn position_block() -> Block {
    let navbar = |position| {
        Navbar::simple()
            .with_position(position)
            .with_prop(light_bg())
            .with_item(navbar::Item::nav(demo_nav()))
    };

    // Las barras fijas se anclan al marco y no a la ventana, para no tapar el resto de la página.
    let fixed_variants: [(&str, navbar::Position, &str); 2] = [
        (
            "menus_title_position_fixed_top",
            navbar::Position::FixedTop,
            "Navbar::simple().with_position(navbar::Position::FixedTop)",
        ),
        (
            "menus_title_position_fixed_bottom",
            navbar::Position::FixedBottom,
            "Navbar::simple().with_position(navbar::Position::FixedBottom)",
        ),
    ];

    let mut block = Block::new()
        .with_title(Lc::t("menus_block_title_position", &LOC))
        .with_child(Html::with(|cx| {
            html! {
                p { (Lc::t("menus_position_note", &LOC).using(cx)) }
            }
        }));
    for (title_key, position, code) in fixed_variants {
        block = add_demo(
            block,
            Lc::t(title_key, &LOC),
            code,
            Container::new()
                .with_width(container::Width::Fluid)
                .with_prop(PropsOp::add_classes("menus-demo-frame"))
                .with_child(navbar(position))
                .with_child(filler()),
        );
    }

    // Con `sticky` la barra ocupa su lugar en el flujo y se queda pegada al borde del contenedor
    // que se desplaza (aquí un marco con scroll).
    block = add_demo(
        block,
        Lc::t("menus_title_position_sticky_top", &LOC),
        "Navbar::simple().with_position(navbar::Position::StickyTop)",
        Container::new()
            .with_width(container::Width::Fluid)
            .with_prop(PropsOp::add_classes("menus-demo-scroll"))
            .with_child(navbar(navbar::Position::StickyTop))
            .with_child(filler())
            .with_child(filler())
            .with_child(filler())
            .with_child(filler()),
    );
    add_demo(
        block,
        Lc::t("menus_title_position_sticky_bottom", &LOC),
        "Navbar::simple().with_position(navbar::Position::StickyBottom)",
        Container::new()
            .with_width(container::Width::Fluid)
            .with_prop(PropsOp::add_classes("menus-demo-scroll"))
            .with_child(filler())
            .with_child(filler())
            .with_child(filler())
            .with_child(filler())
            .with_child(navbar(navbar::Position::StickyBottom)),
    )
}

// **< Disposición >********************************************************************************

fn layout_block() -> Block {
    let layout_variants: [(&str, Navbar, &str); 4] = [
        (
            "menus_title_layout_toggle",
            Navbar::simple_toggle(),
            "Navbar::simple_toggle()",
        ),
        (
            "menus_title_layout_brand_left",
            Navbar::brand_left(demo_brand()),
            "Navbar::brand_left(brand)",
        ),
        (
            "menus_title_layout_brand_right",
            Navbar::brand_right(demo_brand()),
            "Navbar::brand_right(brand)",
        ),
        (
            "menus_title_layout_brand_always",
            Navbar::simple_brand_left(demo_brand()),
            "Navbar::simple_brand_left(brand)",
        ),
    ];

    let mut block = Block::new().with_title(Lc::t("menus_block_title_layout", &LOC));
    for (title_key, navbar, code) in layout_variants {
        block = add_demo(
            block,
            Lc::t(title_key, &LOC),
            code,
            navbar
                .with_prop(light_bg())
                .with_expand(Breakpoint::Lg)
                .with_item(navbar::Item::nav(demo_nav())),
        );
    }
    block
}

// **< Punto de corte >*****************************************************************************

fn expand_block() -> Block {
    let expand_variants: [(Lc, Breakpoint, &str); 5] = [
        (
            expand_title("sm", "576px"),
            Breakpoint::Sm,
            "Navbar::brand_left(brand).with_expand(Breakpoint::Sm)",
        ),
        (
            expand_title("md", "768px"),
            Breakpoint::Md,
            "Navbar::brand_left(brand).with_expand(Breakpoint::Md)",
        ),
        (
            expand_title("lg", "992px"),
            Breakpoint::Lg,
            "Navbar::brand_left(brand).with_expand(Breakpoint::Lg)",
        ),
        (
            expand_title("xl", "1200px"),
            Breakpoint::Xl,
            "Navbar::brand_left(brand).with_expand(Breakpoint::Xl)",
        ),
        (
            expand_title("xxl", "1400px"),
            Breakpoint::Xxl,
            "Navbar::brand_left(brand).with_expand(Breakpoint::Xxl)",
        ),
    ];

    let mut block = Block::new().with_title(Lc::t("menus_block_title_expand", &LOC));
    for (title, breakpoint, code) in expand_variants {
        block = add_demo(
            block,
            title,
            code,
            Navbar::brand_left(demo_brand())
                .with_prop(light_bg())
                .with_expand(breakpoint)
                .with_item(navbar::Item::nav(demo_nav())),
        );
    }
    block
}

fn expand_title(name: &'static str, width: &'static str) -> Lc {
    Lc::t("menus_title_expand", &LOC)
        .with_arg("name", name)
        .with_arg("width", width)
}

// **< Offcanvas >**********************************************************************************

fn offcanvas_block() -> Block {
    let panel = |placement, backdrop| {
        bs::navbar::Panel::new()
            .with_title(Lc::t("menus_offcanvas_title", &LOC))
            .with_placement(placement)
            .with_backdrop(backdrop)
    };

    let offcanvas_variants: [(&str, Navbar, &str); 4] = [
        (
            "menus_title_offcanvas_start",
            Navbar::offcanvas(panel(
                bs::offcanvas::Placement::Start,
                bs::offcanvas::Backdrop::Enabled,
            )),
            concat!(
                "Navbar::offcanvas(bs::navbar::Panel::new()",
                ".with_placement(bs::offcanvas::Placement::Start))",
            ),
        ),
        (
            "menus_title_offcanvas_brand_left",
            Navbar::offcanvas_brand_left(
                demo_brand(),
                panel(
                    bs::offcanvas::Placement::End,
                    bs::offcanvas::Backdrop::Enabled,
                ),
            ),
            concat!(
                "Navbar::offcanvas_brand_left(brand, bs::navbar::Panel::new()",
                ".with_placement(bs::offcanvas::Placement::End))",
            ),
        ),
        (
            "menus_title_offcanvas_brand_right",
            Navbar::offcanvas_brand_right(
                demo_brand(),
                panel(
                    bs::offcanvas::Placement::Start,
                    bs::offcanvas::Backdrop::Disabled,
                ),
            ),
            concat!(
                "Navbar::offcanvas_brand_right(brand, bs::navbar::Panel::new()",
                ".with_backdrop(bs::offcanvas::Backdrop::Disabled))",
            ),
        ),
        (
            "menus_title_offcanvas_dark",
            Navbar::offcanvas_brand_left(
                demo_brand(),
                panel(
                    bs::offcanvas::Placement::End,
                    bs::offcanvas::Backdrop::Static,
                ),
            )
            .with_prop(PropsOp::add_classes(class::Bg::with(BootsierColors::Dark)))
            .with_prop(PropsOp::set("data-bs-theme", "dark")),
            concat!(
                "Navbar::offcanvas_brand_left(brand, bs::navbar::Panel::new()",
                ".with_backdrop(bs::offcanvas::Backdrop::Static))",
                ".with_prop(PropsOp::set(\"data-bs-theme\", \"dark\"))",
            ),
        ),
    ];

    let mut block = Block::new().with_title(Lc::t("menus_block_title_offcanvas", &LOC));
    for (title_key, navbar, code) in offcanvas_variants {
        let navbar = navbar
            .with_expand(Breakpoint::Lg)
            .with_item(navbar::Item::nav(demo_nav()));
        // La barra oscura ya define su propio fondo.
        let navbar = if title_key == "menus_title_offcanvas_dark" {
            navbar
        } else {
            navbar.with_prop(light_bg())
        };
        block = add_demo(block, Lc::t(title_key, &LOC), code, navbar);
    }
    block
}

// **< HELPERS >************************************************************************************

// Marca de identidad de las demostraciones.
fn demo_brand() -> Brand {
    Brand::new()
        .with_title(Lc::n("PageTop"))
        .with_route(Route::from("/"))
}

// Logotipo de las demostraciones, con el tamaño habitual de una marca en una barra.
fn demo_logo() -> Image {
    Image::with(image::Source::logo(PageTopSvg::Tile))
        .with_size(image::Size::Both(UnitValue::Px(36)))
}

// Menú común a la mayoría de las demostraciones: activo, enlace, desplegable y deshabilitado.
fn demo_nav() -> Nav {
    Nav::new()
        .with_item(nav::Item::link(Lc::t("menus_item_home", &LOC), "/").with_active(true))
        .with_item(nav::Item::link(Lc::t("menus_item_link", &LOC), "/link"))
        .with_item(nav::Item::dropdown(
            Dropdown::new()
                .with_title(Lc::t("menus_test_title", &LOC))
                .with_item(dropdown::Item::link(
                    Lc::t("menus_item_action", &LOC),
                    "/action",
                ))
                .with_item(dropdown::Item::link(
                    Lc::t("menus_item_another", &LOC),
                    "/another",
                ))
                .with_item(dropdown::Item::divider())
                .with_item(dropdown::Item::link(
                    Lc::t("menus_item_something", &LOC),
                    "/something",
                )),
        ))
        .with_item(nav::Item::link_disabled(
            Lc::t("menus_item_disabled", &LOC),
            "#",
        ))
}

fn is_bootsier() -> bool {
    global::SETTINGS.app.theme.eq_ignore_ascii_case("bootsier")
}

// Fondo claro para que las barras destaquen sobre el fondo de la página. Basic no da fondo a
// `.navbar`, así que con otros temas se usa el color de fondo de Basic (o un gris claro si el tema
// activo tampoco lo define).
fn light_bg() -> PropsOp {
    if is_bootsier() {
        PropsOp::add_classes(class::Bg::with(class::BgColor::BodyTertiary))
    } else {
        PropsOp::add_style("background-color", "var(--val-color--bg, #f8f9fa)")
    }
}

// Título y fragmento de código que introducen cada demostración, seguidos de ésta.
fn add_demo(block: Block, title: Lc, code: &'static str, demo: impl Into<ChildOp>) -> Block {
    block
        .with_child(caption(title, Lc::n(code)))
        .with_child(demo)
}

// Párrafo de relleno para que los marcos de demostración tengan contenido que desplazar.
fn filler() -> Html {
    Html::with(|cx| {
        html! {
            p { (Lc::t("menus_filler", &LOC).using(cx)) }
        }
    })
}

// Título y fragmento de código que introducen cada demostración.
fn caption(title: Lc, code: Lc) -> Html {
    Html::with(move |cx| {
        html! {
            h3 class="menus-demo-title" { (title.using(cx)) }
            p { code class="menus-demo-code" { (code.using(cx)) } }
        }
    })
}

// Separa el título de cada demostración de la barra (o marco) que le precede.
fn demo_title_styles() -> AssetsOp {
    AssetsOp::add_responsive_styles(None, "menus-demo-title", [("margin-top", "1.5rem")])
}

// Evita que los fragmentos de código largos desborden su contenedor.
fn demo_code_styles() -> AssetsOp {
    AssetsOp::add_responsive_styles(None, "menus-demo-code", [("overflow-wrap", "anywhere")])
}

// Marco de tamaño fijo para las barras con posición `fixed`.
fn demo_frame_styles() -> AssetsOp {
    AssetsOp::add_responsive_styles(
        None,
        "menus-demo-frame",
        [
            ("position", "relative"),
            // Un `transform` convierte al marco en el bloque contenedor de sus descendientes con
            // `position: fixed`, así la barra se ancla al marco en vez de a la ventana.
            ("transform", "translateZ(0)"),
            ("height", "12rem"),
            ("padding", "4rem 1rem"),
            ("overflow", "hidden"),
            ("border", "1px solid #dee2e6"),
            ("margin", "0 0 1.5rem"),
        ],
    )
}

// Marco de tamaño fijo con desplazamiento, para las barras con posición `sticky`.
fn demo_scroll_styles() -> AssetsOp {
    AssetsOp::add_responsive_styles(
        None,
        "menus-demo-scroll",
        [
            ("height", "12rem"),
            ("overflow-y", "auto"),
            ("padding", "0 1rem"),
            ("border", "1px solid #dee2e6"),
            ("margin", "0 0 1.5rem"),
        ],
    )
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&NavbarMenus).await.run().await
}
