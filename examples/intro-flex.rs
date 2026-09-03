use pagetop::prelude::*;

include_locales!(LOC from "examples/locale");

struct IntroFlex;

#[async_trait]
impl Extension for IntroFlex {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_bootsier::Bootsier]
    }

    fn configure_router(&self, router: Router) -> Router {
        router.route("/", web::get(intro_flex))
    }
}

async fn intro_flex(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_assets(AssetsOp::AddStyleSheet(demo_styles()))
        .with_child(
            Intro::default()
                .with_opening(IntroOpening::Custom)
                .with_title(Lc::n("PageTop"))
                .with_slogan(Lc::t("flex_slogan", &LOC))
                .with_button(None::<(Lc, Route)>)
                .with_child(direction_block())
                .with_child(justify_block())
                .with_child(align_block())
                .with_child(align_self_block())
                .with_child(align_content_block())
                .with_child(grow_shrink_block())
                .with_child(other_block()),
        )
        .render()
        .await
}

fn direction_block() -> Block {
    let mut block = Block::new().with_title(Lc::t("flex_block_title_direction", &LOC));

    let direction_variants: [(&str, Flex, &str); 4] = [
        ("flex_title_direction_row", Flex::row(), "Flex::row()"),
        (
            "flex_title_direction_row_reverse",
            Flex::row().with_direction(flex::Direction::RowReverse),
            "Flex::row().with_direction(Direction::RowReverse)",
        ),
        (
            "flex_title_direction_column",
            Flex::column(),
            "Flex::column()",
        ),
        (
            "flex_title_direction_column_reverse",
            Flex::column().with_direction(flex::Direction::ColumnReverse),
            "Flex::column().with_direction(Direction::ColumnReverse)",
        ),
    ];
    for (title_key, flex, code) in direction_variants {
        block = block
            .with_child(caption(Lc::t(title_key, &LOC), code))
            .with_child(
                demo_row(flex)
                    .with_child(demo_box(flex_item("1")))
                    .with_child(demo_box(flex_item("2")))
                    .with_child(demo_box(flex_item("3"))),
            );
    }
    block
}

fn justify_block() -> Block {
    let mut block = Block::new().with_title(Lc::t("flex_block_title_justify", &LOC));

    let justify_variants: [(&str, flex::ContentJustify, &str); 6] = [
        (
            "flex_title_justify_start",
            flex::ContentJustify::Start,
            "Flex::row().with_justify(ContentJustify::Start)",
        ),
        (
            "flex_title_justify_center",
            flex::ContentJustify::Center,
            "Flex::row().with_justify(ContentJustify::Center)",
        ),
        (
            "flex_title_justify_end",
            flex::ContentJustify::End,
            "Flex::row().with_justify(ContentJustify::End)",
        ),
        (
            "flex_title_justify_between",
            flex::ContentJustify::SpaceBetween,
            "Flex::row().with_justify(ContentJustify::SpaceBetween)",
        ),
        (
            "flex_title_justify_around",
            flex::ContentJustify::SpaceAround,
            "Flex::row().with_justify(ContentJustify::SpaceAround)",
        ),
        (
            "flex_title_justify_evenly",
            flex::ContentJustify::SpaceEvenly,
            "Flex::row().with_justify(ContentJustify::SpaceEvenly)",
        ),
    ];
    for (title_key, justify, code) in justify_variants {
        block = block
            .with_child(caption(Lc::t(title_key, &LOC), code))
            .with_child(
                demo_row(
                    Flex::row()
                        .with_justify(justify)
                        .with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))),
                )
                .with_child(demo_box(flex_item("1")))
                .with_child(demo_box(flex_item("2")))
                .with_child(demo_box(flex_item("3"))),
            );
    }
    block
}

fn align_block() -> Block {
    let mut block = Block::new().with_title(Lc::t("flex_block_title_align", &LOC));

    let align_variants: [(&str, flex::Align, &str); 4] = [
        (
            "flex_title_align_start",
            flex::Align::Start,
            "Flex::row().with_align(Align::Start)",
        ),
        (
            "flex_title_align_center",
            flex::Align::Center,
            "Flex::row().with_align(Align::Center)",
        ),
        (
            "flex_title_align_end",
            flex::Align::End,
            "Flex::row().with_align(Align::End)",
        ),
        (
            "flex_title_align_stretch",
            flex::Align::Stretch,
            "Flex::row().with_align(Align::Stretch)",
        ),
    ];
    for (title_key, align, code) in align_variants {
        block = block
            .with_child(caption(Lc::t(title_key, &LOC), code))
            .with_child(
                demo_row(
                    Flex::row()
                        .with_align(align)
                        .with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))),
                )
                .with_child(sized_box(Lc::t("flex_box_tall", &LOC), "2.5rem 1rem"))
                .with_child(demo_box(Lc::t("flex_box_medium", &LOC)))
                .with_child(sized_box(Lc::t("flex_box_short", &LOC), "0.15rem 1rem")),
            );
    }
    block
        .with_child(caption(
            Lc::t("flex_title_align_baseline", &LOC),
            "Flex::row().with_align(Align::Baseline)",
        ))
        .with_child(
            demo_row(
                Flex::row()
                    .with_align(flex::Align::Baseline)
                    .with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))),
            )
            .with_child(
                sized_box(Lc::t("flex_box_tall", &LOC), "2.5rem 1rem")
                    .with_prop(PropsOp::add_style("font-size", "1.75rem")),
            )
            .with_child(demo_box(Lc::t("flex_box_medium", &LOC)))
            .with_child(sized_box(Lc::t("flex_box_short", &LOC), "0.15rem 1rem")),
        )
}

fn align_self_block() -> Block {
    let mut block = Block::new().with_title(Lc::t("flex_block_title_align_self", &LOC));

    let align_self_variants: [(&str, flex::ItemAlign, &str); 4] = [
        (
            "flex_title_align_self_start",
            flex::ItemAlign::Start,
            "FlexItem::new().with_align_self(flex::ItemAlign::Start)",
        ),
        (
            "flex_title_align_self_end",
            flex::ItemAlign::End,
            "FlexItem::new().with_align_self(flex::ItemAlign::End)",
        ),
        (
            "flex_title_align_self_center",
            flex::ItemAlign::Center,
            "FlexItem::new().with_align_self(flex::ItemAlign::Center)",
        ),
        (
            "flex_title_align_self_stretch",
            flex::ItemAlign::Stretch,
            "FlexItem::new().with_align_self(flex::ItemAlign::Stretch)",
        ),
    ];
    for (title_key, align_self, code) in align_self_variants {
        block = block
            .with_child(caption(Lc::t(title_key, &LOC), code))
            .with_child(
                demo_row(
                    Flex::row()
                        .with_align(flex::Align::Start)
                        .with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))),
                )
                .with_child(sized_box(Lc::t("flex_box_tall", &LOC), "2.5rem 1rem"))
                .with_child(demo_box(flex_item("1")).with_prop(PropsOp::flex_item(
                    FlexItem::new().with_align_self(align_self),
                )))
                .with_child(sized_box(Lc::t("flex_box_tall", &LOC), "2.5rem 1rem")),
            );
    }
    block
        .with_child(caption(
            Lc::t("flex_title_align_self_baseline", &LOC),
            "FlexItem::new().with_align_self(flex::ItemAlign::Baseline)",
        ))
        .with_child(
            demo_row(
                Flex::row()
                    .with_align(flex::Align::Start)
                    .with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))),
            )
            .with_child(
                sized_box(Lc::t("flex_box_tall", &LOC), "2.5rem 1rem")
                    .with_prop(PropsOp::add_style("font-size", "1.75rem")),
            )
            .with_child(demo_box(flex_item("1")).with_prop(PropsOp::flex_item(
                FlexItem::new().with_align_self(flex::ItemAlign::Baseline),
            )))
            .with_child(sized_box(Lc::t("flex_box_tall", &LOC), "2.5rem 1rem")),
        )
}

fn align_content_block() -> Block {
    let mut block = Block::new().with_title(Lc::t("flex_block_title_align_content", &LOC));

    let align_content_variants: [(&str, flex::AlignContent, &str); 7] = [
        (
            "flex_title_align_content_start",
            flex::AlignContent::Start,
            "Flex::row().with_wrap(Behavior::Wrap).with_align_content(AlignContent::Start)",
        ),
        (
            "flex_title_align_content_end",
            flex::AlignContent::End,
            "Flex::row().with_wrap(Behavior::Wrap).with_align_content(AlignContent::End)",
        ),
        (
            "flex_title_align_content_center",
            flex::AlignContent::Center,
            "Flex::row().with_wrap(Behavior::Wrap).with_align_content(AlignContent::Center)",
        ),
        (
            "flex_title_align_content_between",
            flex::AlignContent::SpaceBetween,
            "Flex::row().with_wrap(Behavior::Wrap).with_align_content(AlignContent::SpaceBetween)",
        ),
        (
            "flex_title_align_content_around",
            flex::AlignContent::SpaceAround,
            "Flex::row().with_wrap(Behavior::Wrap).with_align_content(AlignContent::SpaceAround)",
        ),
        (
            "flex_title_align_content_evenly",
            flex::AlignContent::SpaceEvenly,
            "Flex::row().with_wrap(Behavior::Wrap).with_align_content(AlignContent::SpaceEvenly)",
        ),
        (
            "flex_title_align_content_stretch",
            flex::AlignContent::Stretch,
            "Flex::row().with_wrap(Behavior::Wrap).with_align_content(AlignContent::Stretch)",
        ),
    ];
    for (title_key, align_content, code) in align_content_variants {
        let mut row = demo_row(
            Flex::row()
                .with_wrap(flex::Behavior::Wrap)
                .with_align_content(align_content)
                .with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))),
        )
        .with_prop(PropsOp::add_style("max-width", "21rem"))
        .with_prop(PropsOp::add_style("min-height", "11rem"));
        for label in ["1", "2", "3", "4"] {
            row = row.with_child(
                sized_box(flex_item(label), "0.5rem 1rem")
                    .with_prop(PropsOp::add_style("width", "9rem")),
            );
        }
        block = block
            .with_child(caption(Lc::t(title_key, &LOC), code))
            .with_child(row);
    }
    block
}

fn grow_shrink_block() -> Block {
    Block::new()
        .with_title(Lc::t("flex_block_title_grow_shrink", &LOC))
        .with_child(caption(
            Lc::t("flex_title_grow", &LOC),
            "FlexItem::new().with_grow(flex::ItemGrow::Is1)",
        ))
        .with_child(
            demo_row(Flex::row().with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))))
                .with_child(demo_box(Lc::t("flex_box_fixed", &LOC)))
                .with_child(
                    demo_box(Lc::t("flex_box_grows", &LOC)).with_prop(PropsOp::flex_item(
                        FlexItem::new().with_grow(flex::ItemGrow::Is1),
                    )),
                )
                .with_child(demo_box(Lc::t("flex_box_fixed", &LOC))),
        )
        .with_child(caption(
            Lc::t("flex_title_shrink", &LOC),
            "FlexItem::new().with_shrink(flex::ItemShrink::Is0)",
        ))
        .with_child(
            demo_row(Flex::row().with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))))
                .with_prop(PropsOp::add_style("max-width", "31rem"))
                .with_child(
                    demo_box(flex_item("1")).with_prop(PropsOp::add_style("width", "10.5rem")),
                )
                .with_child(
                    demo_box(flex_item("2"))
                        .with_prop(PropsOp::add_style("width", "10.5rem"))
                        .with_prop(PropsOp::flex_item(
                            FlexItem::new().with_shrink(flex::ItemShrink::Is0),
                        )),
                )
                .with_child(
                    demo_box(flex_item("3")).with_prop(PropsOp::add_style("width", "10.5rem")),
                ),
        )
}

fn other_block() -> Block {
    let mut block = Block::new().with_title(Lc::t("flex_block_title_other", &LOC));

    block = block
        .with_child(caption(
            Lc::t("flex_title_push_end", &LOC),
            "FlexItem::push_end()",
        ))
        .with_child(
            demo_row(Flex::row().with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))))
                .with_child(demo_box(Lc::t("flex_box_start_1", &LOC)))
                .with_child(demo_box(Lc::t("flex_box_start_2", &LOC)))
                .with_child(demo_box(Lc::t("flex_box_end", &LOC)).with_prop(FlexItem::push_end())),
        );

    let mut wrap_row = demo_row(
        Flex::row()
            .with_wrap(flex::Behavior::Wrap)
            .with_align_content(flex::AlignContent::SpaceBetween)
            .with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))),
    )
    .with_prop(PropsOp::add_style("max-width", "20rem"))
    .with_prop(PropsOp::add_style("min-height", "11rem"));
    for label in ["1", "2", "3", "4", "5", "6", "7", "8"] {
        wrap_row = wrap_row.with_child(
            sized_box(flex_item(label), "0.5rem 1rem")
                .with_prop(PropsOp::add_style("width", "4rem")),
        );
    }
    block
        .with_child(caption(
            Lc::t("flex_title_wrap", &LOC),
            "Flex::row().with_wrap(Behavior::Wrap).with_align_content(AlignContent::SpaceBetween)",
        ))
        .with_child(wrap_row)
        .with_child(caption(
            Lc::t("flex_title_order", &LOC),
            "FlexItem::new().with_order(ItemOrder::First) / .with_order(ItemOrder::Last)",
        ))
        .with_child(
            demo_row(Flex::row().with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))))
                .with_child(demo_box(Lc::n("A")).with_prop(PropsOp::flex_item(
                    FlexItem::new().with_order(flex::ItemOrder::Last),
                )))
                .with_child(demo_box(Lc::n("B")))
                .with_child(demo_box(Lc::n("C")))
                .with_child(demo_box(Lc::n("D")))
                .with_child(demo_box(Lc::n("E")).with_prop(PropsOp::flex_item(
                    FlexItem::new().with_order(flex::ItemOrder::First),
                ))),
        )
        .with_child(caption(
            Lc::t("flex_title_gap_none", &LOC),
            "Flex::row() (Gap::None por defecto)",
        ))
        .with_child(
            demo_row(Flex::row())
                .with_child(demo_box(flex_item("1")))
                .with_child(demo_box(flex_item("2")))
                .with_child(demo_box(flex_item("3"))),
        )
        .with_child(caption(
            Lc::t("flex_title_gap_some", &LOC),
            "Flex::row().with_gap(flex::Gap::Both(UnitValue::RelRem(1.5)))",
        ))
        .with_child(
            demo_row(Flex::row().with_gap(flex::Gap::Both(UnitValue::RelRem(1.5))))
                .with_child(demo_box(flex_item("1")))
                .with_child(demo_box(flex_item("2")))
                .with_child(demo_box(flex_item("3"))),
        )
        .with_child(caption(
            Lc::t("flex_title_grid_thirds", &LOC),
            "FlexItem::new().with_size(flex::ItemSize::Percent33)",
        ))
        .with_child(
            demo_row(Flex::row())
                .with_child(demo_box(Lc::n("1/3")).with_prop(PropsOp::flex_item(
                    FlexItem::new().with_size(flex::ItemSize::Percent33),
                )))
                .with_child(demo_box(Lc::n("1/3")).with_prop(PropsOp::flex_item(
                    FlexItem::new().with_size(flex::ItemSize::Percent33),
                )))
                .with_child(demo_box(Lc::n("1/3")).with_prop(PropsOp::flex_item(
                    FlexItem::new().with_size(flex::ItemSize::Percent33),
                ))),
        )
        .with_child(caption(
            Lc::t("flex_title_grid_offset", &LOC),
            "FlexItem::new().with_size(ItemSize::Percent50).with_offset(ItemOffset::Percent25)",
        ))
        .with_child(
            demo_row(Flex::row()).with_child(
                demo_box(Lc::t("flex_box_half_centered", &LOC)).with_prop(PropsOp::flex_item(
                    FlexItem::new()
                        .with_size(flex::ItemSize::Percent50)
                        .with_offset(flex::ItemOffset::Percent25),
                )),
            ),
        )
        .with_child(caption(
            Lc::t("flex_title_toolbar", &LOC),
            "Container con Flex anidado dentro de otro Container con Flex, y push_end()",
        ))
        .with_child(
            demo_row(Flex::row().with_align(flex::Align::Center))
                .with_child(
                    Container::new()
                        .with_flex(Flex::row().with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))))
                        .with_child(demo_box(Lc::t("flex_box_file", &LOC)))
                        .with_child(demo_box(Lc::t("flex_box_edit", &LOC)))
                        .with_child(demo_box(Lc::t("flex_box_view", &LOC))),
                )
                .with_child(
                    Container::new()
                        .with_prop(FlexItem::push_end())
                        .with_flex(Flex::row().with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))))
                        .with_child(demo_box(Lc::t("flex_box_profile", &LOC)))
                        .with_child(demo_box(Lc::t("flex_box_logout", &LOC))),
                ),
        )
}

// **< HELPERS >************************************************************************************

// Aspecto fijo de las cajas y filas de muestra.
fn demo_styles() -> StyleSheet {
    StyleSheet::inline("intro-flex", |_| {
        util::indoc!(
            r#"
            .flex-demo-box {
                background-color: #0d6efd;
                color: #fff;
                min-width: 3rem;
                width: auto;
                max-width: none;
                margin: 0;
                border-radius: 0.375rem;
                text-align: center;
            }
            .flex-demo-row {
                background-color: #f1f3f5;
                width: 100%;
                max-width: none;
                margin: 0 0 1.5rem;
                padding: 0.75rem;
            }
            "#
        )
        .to_string()
    })
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

// Etiqueta "Flex item N" para las cajas que solo se distinguen por su posicion.
fn flex_item(n: impl Into<CowStr>) -> Lc {
    Lc::t("flex_item_label", &LOC).with_arg("n", n)
}

// Fila de demostracion con fondo gris para visualizar los limites del propio contenedor flex.
fn demo_row(flex: Flex) -> Container {
    Container::new()
        .with_prop(PropsOp::add_classes("flex-demo-row"))
        .with_flex(flex)
}

// Titulo y fragmento de codigo que introducen cada demostracion.
fn caption(title: Lc, code: &'static str) -> Html {
    Html::with(move |cx| {
        html! {
            h3 { (title.using(cx)) }
            p { code { (code) } }
        }
    })
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&IntroFlex).await.run().await
}
