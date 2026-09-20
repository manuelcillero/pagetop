use pagetop::prelude::*;

include_locales!(LOC from "examples/locale");

struct IntroSpacing;

#[async_trait]
impl Extension for IntroSpacing {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_bootsier::Bootsier]
    }

    fn configure_router(&self, router: Router) -> Router {
        router.route("/", web::get(intro_spacing))
    }
}

async fn intro_spacing(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_assets(demo_box_styles())
        .with_assets(demo_row_styles())
        .with_assets(demo_code_styles())
        .with_child(
            Intro::custom()
                .with_title(Lc::n("PageTop"))
                .with_slogan(Lc::t("spacing_slogan", &LOC))
                .with_child(Html::with(|cx| {
                    html! {
                        p class="intro-text-lead" {
                            (Lc::t("spacing_note", &LOC).using(cx))
                        }
                    }
                }))
                .with_child(padding_block())
                .with_child(layout_block())
                .with_child(responsive_block())
                .with_child(combined_block()),
        )
        .render()
        .await
}

fn padding_block() -> Block {
    let mut block = Block::new().with_title(Lc::t("spacing_block_title_padding", &LOC));

    let padding_variants: [(&str, UnitValue, &str); 3] = [
        (
            "spacing_title_uniform_small",
            UnitValue::RelRem(0.5),
            "Padding::new().with_all(UnitValue::RelRem(0.5))",
        ),
        (
            "spacing_title_uniform_medium",
            UnitValue::RelRem(1.0),
            "Padding::new().with_all(UnitValue::RelRem(1.0))",
        ),
        (
            "spacing_title_uniform_large",
            UnitValue::RelRem(2.0),
            "Padding::new().with_all(UnitValue::RelRem(2.0))",
        ),
    ];
    for (title_key, size, code) in padding_variants {
        block = block
            .with_child(caption(Lc::t(title_key, &LOC), Lc::n(code)))
            .with_child(
                demo_row(Flex::new().with_gap(align::Gap::Both(UnitValue::RelRem(0.5))))
                    .with_child(demo_box(box_sample()).with_prop(Padding::new().with_all(size))),
            );
    }

    block
        .with_child(caption(
            Lc::t("spacing_title_sides", &LOC),
            Lc::n(concat!(
                "Padding::new()",
                ".with_top(UnitValue::RelRem(0.25))",
                ".with_end(UnitValue::RelRem(2.5))",
                ".with_bottom(UnitValue::RelRem(1.5))",
                ".with_start(UnitValue::RelRem(0.5))",
            )),
        ))
        .with_child(
            demo_row(Flex::new()).with_child(
                demo_box(box_sample()).with_prop(
                    Padding::new()
                        .with_top(UnitValue::RelRem(0.25))
                        .with_end(UnitValue::RelRem(2.5))
                        .with_bottom(UnitValue::RelRem(1.5))
                        .with_start(UnitValue::RelRem(0.5)),
                ),
            ),
        )
}

fn layout_block() -> Block {
    Block::new()
        .with_title(Lc::t("spacing_block_title_layout", &LOC))
        .with_child(caption(
            Lc::t("spacing_title_margin_gap", &LOC),
            Lc::n("Margin::new().with_x(UnitValue::RelRem(1.0))"),
        ))
        .with_child(
            demo_row(Flex::new())
                .with_child(demo_box(box_sample()))
                .with_child(
                    demo_box(box_sample()).with_prop(Margin::new().with_x(UnitValue::RelRem(1.0))),
                )
                .with_child(demo_box(box_sample())),
        )
        .with_child(caption(
            Lc::t("spacing_title_margin_center", &LOC),
            Lc::n("Margin::new().with_x(UnitValue::Auto)"),
        ))
        .with_child(
            demo_row(Flex::new()).with_child(
                demo_box(box_sample())
                    .with_prop(Margin::new().with_x(UnitValue::Auto))
                    .with_prop(
                        FlexItem::new().with_size(flex::ItemSize::Custom(UnitValue::RelRem(8.0))),
                    ),
            ),
        )
}

fn responsive_block() -> Block {
    Block::new()
        .with_title(Lc::t("spacing_block_title_responsive", &LOC))
        .with_child(caption(
            Lc::t("spacing_title_responsive", &LOC),
            Lc::n(concat!(
                "Padding::new()",
                ".with_all(UnitValue::RelRem(0.5))",
                ".with_all_at(Breakpoint::Md, UnitValue::RelRem(2.0))",
                ".with_all_at(Breakpoint::Lg, UnitValue::RelRem(4.0))",
            )),
        ))
        .with_child(
            demo_row(Flex::new()).with_child(
                demo_box(box_sample()).with_prop(
                    Padding::new()
                        .with_all(UnitValue::RelRem(0.5))
                        .with_all_at(Breakpoint::Md, UnitValue::RelRem(2.0))
                        .with_all_at(Breakpoint::Lg, UnitValue::RelRem(4.0)),
                ),
            ),
        )
}

fn combined_block() -> Block {
    Block::new()
        .with_title(Lc::t("spacing_block_title_combined", &LOC))
        .with_child(caption(
            Lc::t("spacing_title_combined", &LOC),
            Lc::n(concat!(
                "Container::new()",
                ".with_prop(Margin::new().with_y(UnitValue::RelRem(1.0)))",
                ".with_prop(Padding::new().with_all(UnitValue::RelRem(1.5)))",
            )),
        ))
        .with_child(
            demo_row(Flex::new()).with_child(
                Container::new()
                    .with_prop(PropsOp::add_classes("spacing-demo-box"))
                    .with_prop(Margin::new().with_y(UnitValue::RelRem(1.0)))
                    .with_prop(Padding::new().with_all(UnitValue::RelRem(1.5)))
                    .with_child(
                        Button::plain(Lc::t("spacing_box_card_button", &LOC))
                            .with_style(button::Style::Solid(Intent::Warning)),
                    ),
            ),
        )
}

// **< HELPERS >************************************************************************************

// Caja de muestra sin relleno ni margen propios, para que sólo se vea el efecto de `Margin`/
// `Padding` aplicado en cada demostración.
fn demo_box_styles() -> AssetsOp {
    AssetsOp::add_responsive_styles(
        None,
        "spacing-demo-box",
        [
            ("background-color", "#0d6efd"),
            ("color", "#fff"),
            ("min-width", "3rem"),
            ("width", "auto"),
            ("max-width", "none"),
            ("margin", "0"),
            ("padding", "0"),
            ("border-radius", "0.375rem"),
            ("text-align", "center"),
        ],
    )
}

// Aspecto fijo de las filas de muestra.
fn demo_row_styles() -> AssetsOp {
    AssetsOp::add_responsive_styles(
        None,
        "spacing-demo-row",
        [
            ("background-color", "#f1f3f5"),
            ("width", "100%"),
            ("max-width", "none"),
            ("margin", "0 0 1.5rem"),
            ("padding", "0.75rem"),
        ],
    )
}

// Evita que los fragmentos de código largos desborden su contenedor.
fn demo_code_styles() -> AssetsOp {
    AssetsOp::add_responsive_styles(None, "spacing-demo-code", [("overflow-wrap", "anywhere")])
}

// Caja azul de muestra, sin relleno ni margen propios.
fn demo_box(label: Lc) -> Container {
    Container::new()
        .with_prop(PropsOp::add_classes("spacing-demo-box"))
        .with_child(Html::with(move |cx| html! { (label.using(cx)) }))
}

// Etiqueta genérica reutilizada en la mayoría de cajas de muestra.
fn box_sample() -> Lc {
    Lc::t("spacing_box_sample", &LOC)
}

// Fila de demostración con fondo gris para visualizar los límites de cada caja.
fn demo_row(flex: Flex) -> Flex {
    flex.with_prop(PropsOp::add_classes("spacing-demo-row"))
}

// Título y fragmento de código que introducen cada demostración.
fn caption(title: Lc, code: Lc) -> Html {
    Html::with(move |cx| {
        html! {
            h3 { (title.using(cx)) }
            p { code class="spacing-demo-code" { (code.using(cx)) } }
        }
    })
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&IntroSpacing).await.run().await
}
