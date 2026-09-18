use pagetop::prelude::*;

include_locales!(LOC from "examples/locale");

struct IntroResponsive;

#[async_trait]
impl Extension for IntroResponsive {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_bootsier::Bootsier]
    }

    fn configure_router(&self, router: Router) -> Router {
        router.route("/", web::get(intro_responsive))
    }
}

async fn intro_responsive(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_assets(demo_box_styles())
        .with_assets(demo_row_styles())
        .with_assets(demo_code_styles())
        .with_child(
            Intro::default()
                .with_opening(IntroOpening::Custom)
                .with_title(Lc::n("PageTop"))
                .with_slogan(Lc::t("responsive_slogan", &LOC))
                .with_button(None::<(Lc, Route)>)
                .with_child(Html::with(|cx| {
                    html! {
                        p class="intro-text-lead" {
                            (Lc::t("responsive_note", &LOC).using(cx))
                        }
                    }
                }))
                .with_child(activation_block())
                .with_child(direction_block())
                .with_child(grid_block())
                .with_child(justify_align_block())
                .with_child(order_block())
                .with_child(gap_grow_block()),
        )
        .render()
        .await
}

fn activation_block() -> Block {
    Block::new()
        .with_title(Lc::t("responsive_block_title_activation", &LOC))
        .with_child(caption(
            Lc::t("responsive_title_activation", &LOC),
            Lc::n("Flex::at(Breakpoint::Md)"),
        ))
        .with_child(
            demo_row(Flex::at(Breakpoint::Md).with_gap(align::Gap::Both(UnitValue::RelRem(0.5))))
                .with_child(demo_box(Lc::t("responsive_box_nav_home", &LOC)))
                .with_child(demo_box(Lc::t("responsive_box_nav_products", &LOC)))
                .with_child(demo_box(Lc::t("responsive_box_nav_about", &LOC)))
                .with_child(demo_box(Lc::t("responsive_box_nav_contact", &LOC))),
        )
}

fn direction_block() -> Block {
    Block::new()
        .with_title(Lc::t("responsive_block_title_direction", &LOC))
        .with_child(caption(
            Lc::t("responsive_title_direction", &LOC),
            Lc::n(concat!(
                "Flex::new()",
                ".with_direction(Direction::Column)",
                ".with_direction_at(Breakpoint::Md, Direction::RowReverse)",
            )),
        ))
        .with_child(
            demo_row(
                Flex::new()
                    .with_direction(flex::Direction::Column)
                    .with_direction_at(Breakpoint::Md, flex::Direction::RowReverse)
                    .with_gap(align::Gap::Both(UnitValue::RelRem(0.5))),
            )
            .with_child(sized_box(
                Lc::t("responsive_box_image", &LOC),
                "2.5rem 1rem",
            ))
            .with_child(demo_box(Lc::t("responsive_box_text", &LOC))),
        )
}

fn grid_block() -> Block {
    let mut row = demo_row(Flex::new().with_wrap(flex::Behavior::Wrap));
    for n in 1..=6 {
        row = row.with_child(
            Container::new()
                .with_prop(PropsOp::add_style("padding", "0.25rem"))
                .with_prop(
                    FlexItem::new()
                        .with_size(flex::ItemSize::Percent100)
                        .with_size_at(Breakpoint::Sm, flex::ItemSize::Percent50)
                        .with_size_at(Breakpoint::Md, flex::ItemSize::Percent33),
                )
                .with_child(demo_box(card_label(n))),
        );
    }
    Block::new()
        .with_title(Lc::t("responsive_block_title_grid", &LOC))
        .with_child(caption(
            Lc::t("responsive_title_grid", &LOC),
            Lc::n(concat!(
                "FlexItem::new()",
                ".with_size(ItemSize::Percent100)",
                ".with_size_at(Breakpoint::Sm, ItemSize::Percent50)",
                ".with_size_at(Breakpoint::Md, ItemSize::Percent33)",
            )),
        ))
        .with_child(row)
}

fn justify_align_block() -> Block {
    Block::new()
        .with_title(Lc::t("responsive_block_title_justify_align", &LOC))
        .with_child(caption(
            Lc::t("responsive_title_justify_align", &LOC),
            Lc::n(concat!(
                "Flex::new()",
                ".with_justify(ContentJustify::Center)",
                ".with_justify_at(Breakpoint::Md, ContentJustify::SpaceBetween)",
                ".with_align(align::Items::Center)",
            )),
        ))
        .with_child(
            demo_row(
                Flex::new()
                    .with_justify(flex::ContentJustify::Center)
                    .with_justify_at(Breakpoint::Md, flex::ContentJustify::SpaceBetween)
                    .with_align(align::Items::Center)
                    .with_gap(align::Gap::Both(UnitValue::RelRem(0.5))),
            )
            .with_child(demo_box(Lc::t("responsive_box_logo", &LOC)))
            .with_child(demo_box(Lc::t("responsive_box_menu", &LOC))),
        )
}

fn order_block() -> Block {
    Block::new()
        .with_title(Lc::t("responsive_block_title_order", &LOC))
        .with_child(caption(
            Lc::t("responsive_title_order", &LOC),
            Lc::n(concat!(
                "Flex::at(Breakpoint::Lg)",
                " + FlexItem::new().with_order_at(Breakpoint::Lg, ItemOrder::First)",
            )),
        ))
        .with_child(
            demo_row(Flex::at(Breakpoint::Lg).with_gap(align::Gap::Both(UnitValue::RelRem(0.5))))
                .with_child(demo_box(Lc::t("responsive_box_content", &LOC)))
                .with_child(demo_box(Lc::t("responsive_box_sidebar", &LOC)).with_prop(
                    FlexItem::new().with_order_at(Breakpoint::Lg, flex::ItemOrder::First),
                )),
        )
}

fn gap_grow_block() -> Block {
    Block::new()
        .with_title(Lc::t("responsive_block_title_gap_grow", &LOC))
        .with_child(caption(
            Lc::t("responsive_title_gap_grow", &LOC),
            Lc::n(concat!(
                "Flex::new()",
                ".with_gap(align::Gap::Both(RelRem(0.5)))",
                ".with_gap_at(Breakpoint::Md, Gap::Both(RelRem(1.5)))",
                " + FlexItem::new().with_grow_at(Breakpoint::Md, ItemGrow::Is1)",
            )),
        ))
        .with_child(
            demo_row(
                Flex::new()
                    .with_gap(align::Gap::Both(UnitValue::RelRem(0.5)))
                    .with_gap_at(Breakpoint::Md, align::Gap::Both(UnitValue::RelRem(1.5))),
            )
            .with_child(demo_box(Lc::t("responsive_box_file", &LOC)))
            .with_child(demo_box(Lc::t("responsive_box_edit", &LOC)))
            .with_child(
                demo_box(Lc::t("responsive_box_search", &LOC))
                    .with_prop(FlexItem::new().with_grow_at(Breakpoint::Md, flex::ItemGrow::Is1)),
            ),
        )
}

// **< HELPERS >************************************************************************************

// Aspecto fijo de las cajas de muestra.
fn demo_box_styles() -> AssetsOp {
    AssetsOp::add_responsive_styles(
        None,
        "flex-demo-box",
        [
            ("background-color", "#0d6efd"),
            ("color", "#fff"),
            ("min-width", "3rem"),
            ("width", "auto"),
            ("max-width", "none"),
            ("margin", "0"),
            ("border-radius", "0.375rem"),
            ("text-align", "center"),
        ],
    )
}

// Aspecto fijo de las filas de muestra.
fn demo_row_styles() -> AssetsOp {
    AssetsOp::add_responsive_styles(
        None,
        "flex-demo-row",
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
    AssetsOp::add_responsive_styles(None, "flex-demo-code", [("overflow-wrap", "anywhere")])
}

// Caja con fondo azul y relleno vertical configurable, para mostrar diferencias de altura.
fn sized_box(label: Lc, padding: &'static str) -> Container {
    Container::new()
        .with_prop(PropsOp::add_classes("flex-demo-box"))
        .with_prop(PropsOp::add_style("padding", padding))
        .with_child(Html::with(move |cx| html! { (label.using(cx)) }))
}

// Caja con el relleno vertical estandar del resto de ejemplos.
fn demo_box(label: Lc) -> Container {
    sized_box(label, "0.5rem 1rem")
}

// Etiqueta "Tarjeta N" para las cajas de la rejilla responsive.
fn card_label(n: usize) -> Lc {
    Lc::t("responsive_box_card", &LOC).with_arg("n", n.to_string())
}

// Fila de demostracion con fondo gris para visualizar los limites del propio contenedor flex.
fn demo_row(flex: Flex) -> Flex {
    flex.with_prop(PropsOp::add_classes("flex-demo-row"))
}

// Titulo y fragmento de codigo que introducen cada demostracion.
fn caption(title: Lc, code: Lc) -> Html {
    Html::with(move |cx| {
        html! {
            h3 { (title.using(cx)) }
            p { code class="flex-demo-code" { (code.using(cx)) } }
        }
    })
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&IntroResponsive).await.run().await
}
