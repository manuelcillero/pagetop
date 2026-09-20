use pagetop::prelude::*;

include_locales!(LOC from "examples/locale");

struct IntroGrid;

#[async_trait]
impl Extension for IntroGrid {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_bootsier::Bootsier]
    }

    fn configure_router(&self, router: Router) -> Router {
        router.route("/", web::get(intro_grid))
    }
}

async fn intro_grid(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_assets(demo_box_styles())
        .with_assets(demo_row_styles())
        .with_child(
            Intro::custom()
                .with_title(Lc::n("PageTop"))
                .with_slogan(Lc::t("grid_slogan", &LOC))
                .with_child(columns_block())
                .with_child(rows_gap_block())
                .with_child(placement_block())
                .with_child(alignment_block())
                .with_child(content_alignment_block())
                .with_child(item_alignment_block())
                .with_child(auto_flow_block())
                .with_child(auto_tracks_block())
                .with_child(responsive_block())
                .with_child(layout_block()),
        )
        .render()
        .await
}

fn columns_block() -> Block {
    Block::new()
        .with_title(Lc::t("grid_block_title_columns", &LOC))
        .with_child(caption(
            Lc::t("grid_title_columns_thirds", &LOC),
            Lc::n("Grid::new().with_columns(Tracks::repeat(3, AxisTrack::Fraction(1.0)))"),
        ))
        .with_child(
            demo_grid(
                Grid::new()
                    .with_columns(grid::Tracks::repeat(3, grid::AxisTrack::Fraction(1.0)))
                    .with_gap(align::Gap::Both(UnitValue::RelRem(0.5))),
            )
            .with_child(demo_box(grid_box("1")))
            .with_child(demo_box(grid_box("2")))
            .with_child(demo_box(grid_box("3"))),
        )
        .with_child(caption(
            Lc::t("grid_title_columns_sidebar", &LOC),
            Lc::n(concat!(
                "Grid::new().with_columns(Tracks::new()",
                ".with_track(AxisTrack::Fixed(UnitValue::Px(140)))",
                ".with_track(AxisTrack::Fraction(1.0)))",
            )),
        ))
        .with_child(
            demo_grid(
                Grid::new()
                    .with_columns(
                        grid::Tracks::new()
                            .with_track(grid::AxisTrack::Fixed(UnitValue::Px(140)))
                            .with_track(grid::AxisTrack::Fraction(1.0)),
                    )
                    .with_gap(align::Gap::Both(UnitValue::RelRem(0.5))),
            )
            .with_child(demo_box(Lc::t("grid_box_sidebar", &LOC)))
            .with_child(demo_box(Lc::t("grid_box_content", &LOC))),
        )
        .with_child(caption(
            Lc::t("grid_title_columns_min_max", &LOC),
            Lc::n(concat!(
                "Grid::new().with_columns(Tracks::new()",
                ".with_track(AxisTrack::MinContent)",
                ".with_track(AxisTrack::Fraction(1.0))",
                ".with_track(AxisTrack::MaxContent))",
            )),
        ))
        .with_child(
            demo_grid(
                Grid::new()
                    .with_columns(
                        grid::Tracks::new()
                            .with_track(grid::AxisTrack::MinContent)
                            .with_track(grid::AxisTrack::Fraction(1.0))
                            .with_track(grid::AxisTrack::MaxContent),
                    )
                    .with_gap(align::Gap::Both(UnitValue::RelRem(0.5))),
            )
            .with_child(demo_box(Lc::t("grid_box_min_content", &LOC)))
            .with_child(demo_box(Lc::t("grid_box_fraction", &LOC)))
            .with_child(demo_box(Lc::t("grid_box_max_content", &LOC))),
        )
}

fn rows_gap_block() -> Block {
    Block::new()
        .with_title(Lc::t("grid_block_title_rows_gap", &LOC))
        .with_child(caption(
            Lc::t("grid_title_rows", &LOC),
            Lc::n(concat!(
                "Grid::new()",
                ".with_columns(Tracks::repeat(2, AxisTrack::Fraction(1.0)))",
                ".with_rows(Tracks::repeat(2, AxisTrack::Fixed(UnitValue::RelRem(3.0))))",
            )),
        ))
        .with_child(
            demo_grid(
                Grid::new()
                    .with_columns(grid::Tracks::repeat(2, grid::AxisTrack::Fraction(1.0)))
                    .with_rows(grid::Tracks::repeat(
                        2,
                        grid::AxisTrack::Fixed(UnitValue::RelRem(3.0)),
                    ))
                    .with_gap(align::Gap::Both(UnitValue::RelRem(0.5))),
            )
            .with_child(demo_box(grid_box("1")))
            .with_child(demo_box(grid_box("2")))
            .with_child(demo_box(grid_box("3")))
            .with_child(demo_box(grid_box("4"))),
        )
        .with_child(caption(
            Lc::t("grid_title_gap_distinct", &LOC),
            Lc::n(concat!(
                "Grid::new().with_gap(align::Gap::Distinct {",
                " row: UnitValue::RelRem(1.5), column: UnitValue::RelRem(0.25) })",
            )),
        ))
        .with_child(
            demo_grid(
                Grid::new()
                    .with_columns(grid::Tracks::repeat(3, grid::AxisTrack::Fraction(1.0)))
                    .with_gap(align::Gap::Distinct {
                        row: UnitValue::RelRem(1.5),
                        column: UnitValue::RelRem(0.25),
                    }),
            )
            .with_child(demo_box(grid_box("1")))
            .with_child(demo_box(grid_box("2")))
            .with_child(demo_box(grid_box("3")))
            .with_child(demo_box(grid_box("4")))
            .with_child(demo_box(grid_box("5")))
            .with_child(demo_box(grid_box("6"))),
        )
}

fn placement_block() -> Block {
    Block::new()
        .with_title(Lc::t("grid_block_title_placement", &LOC))
        .with_child(caption(
            Lc::t("grid_title_placement", &LOC),
            Lc::n(concat!(
                "GridItem::new().with_column(ItemPlacement::Span(2))",
                " / .with_column(ItemPlacement::Range(2, 4))",
            )),
        ))
        .with_child(
            demo_grid(
                Grid::new()
                    .with_columns(grid::Tracks::repeat(4, grid::AxisTrack::Fraction(1.0)))
                    .with_gap(align::Gap::Both(UnitValue::RelRem(0.5))),
            )
            .with_child(demo_box(Lc::t("grid_box_plain", &LOC)))
            .with_child(
                demo_box(Lc::t("grid_box_span_2", &LOC))
                    .with_prop(GridItem::new().with_column(grid::ItemPlacement::Span(2))),
            )
            .with_child(demo_box(Lc::t("grid_box_plain", &LOC)))
            .with_child(
                demo_box(Lc::t("grid_box_range", &LOC)).with_prop(
                    GridItem::new()
                        .with_column(grid::ItemPlacement::Range(2, 4))
                        .with_row(grid::ItemPlacement::Line(2)),
                ),
            ),
        )
}

fn alignment_block() -> Block {
    let mut block = Block::new().with_title(Lc::t("grid_block_title_alignment", &LOC));

    let variants: [(&str, grid::DefaultJustify, align::Items, &str); 4] = [
        (
            "grid_title_alignment_start",
            grid::DefaultJustify::Start,
            align::Items::Start,
            concat!(
                "Grid::new().with_justify_items(DefaultJustify::Start)",
                ".with_align_items(align::Items::Start)",
            ),
        ),
        (
            "grid_title_alignment_center",
            grid::DefaultJustify::Center,
            align::Items::Center,
            concat!(
                "Grid::new().with_justify_items(DefaultJustify::Center)",
                ".with_align_items(align::Items::Center)",
            ),
        ),
        (
            "grid_title_alignment_end",
            grid::DefaultJustify::End,
            align::Items::End,
            concat!(
                "Grid::new().with_justify_items(DefaultJustify::End)",
                ".with_align_items(align::Items::End)",
            ),
        ),
        (
            "grid_title_alignment_stretch",
            grid::DefaultJustify::Stretch,
            align::Items::Stretch,
            concat!(
                "Grid::new().with_justify_items(DefaultJustify::Stretch)",
                ".with_align_items(align::Items::Stretch)",
            ),
        ),
    ];
    for (title_key, justify_items, align_items, code) in variants {
        block = block
            .with_child(caption(Lc::t(title_key, &LOC), Lc::n(code)))
            .with_child(
                demo_grid(
                    Grid::new()
                        .with_columns(grid::Tracks::repeat(
                            3,
                            grid::AxisTrack::Fixed(UnitValue::Px(96)),
                        ))
                        .with_rows(grid::Tracks::repeat(
                            1,
                            grid::AxisTrack::Fixed(UnitValue::RelRem(4.5)),
                        ))
                        .with_justify_items(justify_items)
                        .with_align_items(align_items)
                        .with_gap(align::Gap::Both(UnitValue::RelRem(0.5))),
                )
                .with_child(demo_box(grid_box("1")))
                .with_child(demo_box(grid_box("2")))
                .with_child(demo_box(grid_box("3"))),
            );
    }
    block
}

fn content_alignment_block() -> Block {
    let mut block = Block::new().with_title(Lc::t("grid_block_title_content_alignment", &LOC));

    let variants: [(&str, grid::ContentJustify, &str); 4] = [
        (
            "grid_title_content_alignment_start",
            grid::ContentJustify::Start,
            "Grid::new().with_justify_content(ContentJustify::Start)",
        ),
        (
            "grid_title_content_alignment_center",
            grid::ContentJustify::Center,
            "Grid::new().with_justify_content(ContentJustify::Center)",
        ),
        (
            "grid_title_content_alignment_between",
            grid::ContentJustify::SpaceBetween,
            "Grid::new().with_justify_content(ContentJustify::SpaceBetween)",
        ),
        (
            "grid_title_content_alignment_evenly",
            grid::ContentJustify::SpaceEvenly,
            "Grid::new().with_justify_content(ContentJustify::SpaceEvenly)",
        ),
    ];
    for (title_key, justify_content, code) in variants {
        block = block
            .with_child(caption(Lc::t(title_key, &LOC), Lc::n(code)))
            .with_child(
                demo_grid(
                    Grid::new()
                        .with_columns(grid::Tracks::repeat(
                            3,
                            grid::AxisTrack::Fixed(UnitValue::Px(72)),
                        ))
                        .with_justify_content(justify_content)
                        .with_gap(align::Gap::Both(UnitValue::RelRem(0.5))),
                )
                .with_prop(PropsOp::add_style("max-width", "26rem"))
                .with_child(demo_box(grid_box("1")))
                .with_child(demo_box(grid_box("2")))
                .with_child(demo_box(grid_box("3"))),
            );
    }
    block
}

fn item_alignment_block() -> Block {
    Block::new()
        .with_title(Lc::t("grid_block_title_item_alignment", &LOC))
        .with_child(caption(
            Lc::t("grid_title_item_alignment", &LOC),
            Lc::n(concat!(
                "GridItem::new()",
                ".with_justify_self(ItemJustify::End)",
                ".with_align_self(align::ItemSelf::End)",
            )),
        ))
        .with_child(
            demo_grid(
                Grid::new()
                    .with_columns(grid::Tracks::repeat(
                        3,
                        grid::AxisTrack::Fixed(UnitValue::Px(96)),
                    ))
                    .with_rows(grid::Tracks::repeat(
                        1,
                        grid::AxisTrack::Fixed(UnitValue::RelRem(4.5)),
                    ))
                    .with_gap(align::Gap::Both(UnitValue::RelRem(0.5))),
            )
            .with_child(demo_box(grid_box("1")))
            .with_child(
                demo_box(Lc::t("grid_box_overridden", &LOC)).with_prop(
                    GridItem::new()
                        .with_justify_self(grid::ItemJustify::End)
                        .with_align_self(align::ItemSelf::End),
                ),
            )
            .with_child(demo_box(grid_box("3"))),
        )
}

fn auto_flow_block() -> Block {
    Block::new()
        .with_title(Lc::t("grid_block_title_auto_flow", &LOC))
        .with_child(caption(
            Lc::t("grid_title_auto_flow_row", &LOC),
            Lc::n("Grid::new().with_auto_flow(AutoFlow::Row) (por defecto)"),
        ))
        .with_child(dense_demo(grid::AutoFlow::Row))
        .with_child(caption(
            Lc::t("grid_title_auto_flow_dense", &LOC),
            Lc::n("Grid::new().with_auto_flow(AutoFlow::RowDense)"),
        ))
        .with_child(dense_demo(grid::AutoFlow::RowDense))
}

fn dense_demo(auto_flow: grid::AutoFlow) -> Grid {
    demo_grid(
        Grid::new()
            .with_columns(grid::Tracks::repeat(4, grid::AxisTrack::Fraction(1.0)))
            .with_auto_flow(auto_flow)
            .with_gap(align::Gap::Both(UnitValue::RelRem(0.5))),
    )
    .with_child(
        demo_box(Lc::t("grid_box_span_2", &LOC))
            .with_prop(GridItem::new().with_column(grid::ItemPlacement::Span(2))),
    )
    .with_child(
        demo_box(Lc::t("grid_box_span_2", &LOC))
            .with_prop(GridItem::new().with_column(grid::ItemPlacement::Span(2))),
    )
    .with_child(demo_box(grid_box("1")))
    .with_child(demo_box(grid_box("2")))
    .with_child(demo_box(grid_box("3")))
}

fn auto_tracks_block() -> Block {
    Block::new()
        .with_title(Lc::t("grid_block_title_auto_tracks", &LOC))
        .with_child(caption(
            Lc::t("grid_title_auto_tracks", &LOC),
            Lc::n(concat!(
                "Grid::new()",
                ".with_columns(Tracks::repeat(3, AxisTrack::Fraction(1.0)))",
                ".with_auto_rows(AxisTrack::Fixed(UnitValue::RelRem(3.0)))",
            )),
        ))
        .with_child(
            demo_grid(
                Grid::new()
                    .with_columns(grid::Tracks::repeat(3, grid::AxisTrack::Fraction(1.0)))
                    .with_auto_rows(grid::AxisTrack::Fixed(UnitValue::RelRem(3.0)))
                    .with_gap(align::Gap::Both(UnitValue::RelRem(0.5))),
            )
            .with_child(demo_box(grid_box("1")))
            .with_child(demo_box(grid_box("2")))
            .with_child(demo_box(grid_box("3")))
            .with_child(demo_box(grid_box("4")))
            .with_child(demo_box(grid_box("5")))
            .with_child(demo_box(grid_box("6"))),
        )
}

fn responsive_block() -> Block {
    let mut row = demo_grid(
        Grid::new()
            .with_columns(grid::Tracks::repeat(1, grid::AxisTrack::Fraction(1.0)))
            .with_columns_at(
                Breakpoint::Sm,
                grid::Tracks::repeat(2, grid::AxisTrack::Fraction(1.0)),
            )
            .with_columns_at(
                Breakpoint::Md,
                grid::Tracks::repeat(3, grid::AxisTrack::Fraction(1.0)),
            )
            .with_gap(align::Gap::Both(UnitValue::RelRem(0.5))),
    );
    for n in 1..=6 {
        row = row.with_child(demo_box(card_label(n)));
    }
    Block::new()
        .with_title(Lc::t("grid_block_title_responsive", &LOC))
        .with_child(caption(
            Lc::t("grid_title_responsive", &LOC),
            Lc::n(concat!(
                "Grid::new()",
                ".with_columns(Tracks::repeat(1, AxisTrack::Fraction(1.0)))",
                ".with_columns_at(Breakpoint::Sm, Tracks::repeat(2, AxisTrack::Fraction(1.0)))",
                ".with_columns_at(Breakpoint::Md, Tracks::repeat(3, AxisTrack::Fraction(1.0)))",
            )),
        ))
        .with_child(row)
}

fn layout_block() -> Block {
    Block::new()
        .with_title(Lc::t("grid_block_title_layout", &LOC))
        .with_child(caption(
            Lc::t("grid_title_layout", &LOC),
            Lc::t("grid_desc_layout", &LOC),
        ))
        .with_child(
            demo_grid(
                Grid::new()
                    .with_columns(
                        grid::Tracks::new()
                            .with_track(grid::AxisTrack::Fixed(UnitValue::Px(120)))
                            .with_track(grid::AxisTrack::Fraction(1.0))
                            .with_track(grid::AxisTrack::Fixed(UnitValue::Px(120))),
                    )
                    .with_rows(
                        grid::Tracks::new()
                            .with_track(grid::AxisTrack::Auto)
                            .with_track(grid::AxisTrack::Fraction(1.0))
                            .with_track(grid::AxisTrack::Auto),
                    )
                    .with_gap(align::Gap::Both(UnitValue::RelRem(0.5))),
            )
            .with_prop(PropsOp::add_style("min-height", "18rem"))
            .with_child(
                demo_box(Lc::t("grid_box_header", &LOC))
                    .with_prop(GridItem::new().with_column(grid::ItemPlacement::Span(3))),
            )
            .with_child(demo_box(Lc::t("grid_box_nav", &LOC)))
            .with_child(demo_box(Lc::t("grid_box_content", &LOC)))
            .with_child(demo_box(Lc::t("grid_box_aside", &LOC)))
            .with_child(
                demo_box(Lc::t("grid_box_footer", &LOC))
                    .with_prop(GridItem::new().with_column(grid::ItemPlacement::Span(3))),
            ),
        )
}

// **< HELPERS >************************************************************************************

// Aspecto fijo de las cajas de muestra: esquinas rectas y borde claro, a diferencia de las cajas
// redondeadas de `intro-flex.rs`, para distinguir a simple vista los ejemplos de Grid de los de
// Flex.
fn demo_box_styles() -> AssetsOp {
    AssetsOp::add_responsive_styles(
        None,
        "grid-demo-box",
        [
            ("background-color", "#0d6efd"),
            ("color", "#fff"),
            ("border", "2px solid #6ea8fe"),
            ("min-width", "3rem"),
            ("width", "auto"),
            ("max-width", "none"),
            ("margin", "0"),
            ("text-align", "center"),
        ],
    )
}

// Aspecto fijo de las filas de muestra.
fn demo_row_styles() -> AssetsOp {
    AssetsOp::add_responsive_styles(
        None,
        "grid-demo-row",
        [
            ("background-color", "#f1f3f5"),
            ("width", "100%"),
            ("max-width", "none"),
            ("margin", "0 0 1.5rem"),
            ("padding", "0.75rem"),
        ],
    )
}

// Caja con el relleno vertical estandar del resto de ejemplos.
fn demo_box(label: Lc) -> Container {
    Container::new()
        .with_prop(PropsOp::add_classes("grid-demo-box"))
        .with_prop(PropsOp::add_style("padding", "0.5rem 1rem"))
        .with_child(Html::with(move |cx| html! { (label.using(cx)) }))
}

// Etiqueta "Box N" para las cajas que sólo se distinguen por su posición.
fn grid_box(n: impl Into<CowStr>) -> Lc {
    Lc::t("grid_box_label", &LOC).with_arg("n", n)
}

// Etiqueta "Tarjeta N" para las cajas de la rejilla responsive.
fn card_label(n: usize) -> Lc {
    Lc::t("grid_box_card", &LOC).with_arg("n", n.to_string())
}

// Rejilla de demostracion con fondo gris para visualizar los limites del propio contenedor grid.
fn demo_grid(grid: Grid) -> Grid {
    grid.with_prop(PropsOp::add_classes("grid-demo-row"))
}

// Titulo y fragmento de codigo que introducen cada demostracion.
fn caption(title: Lc, code: Lc) -> Html {
    Html::with(move |cx| {
        html! {
            h3 { (title.using(cx)) }
            p { code { (code.using(cx)) } }
        }
    })
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&IntroGrid).await.run().await
}
