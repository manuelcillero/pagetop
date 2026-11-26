use pagetop::prelude::*;

include_locales!(LOC from "examples/locale");

struct LayoutMosaic;

#[async_trait]
impl Extension for LayoutMosaic {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_bootsier::Bootsier]
    }

    fn configure_router(&self, routes: Routes) -> Routes {
        routes.public_route("/", web::get(layout_mosaic))
    }
}

async fn layout_mosaic(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_assets(demo_box_styles())
        .with_assets(nested_box_styles())
        .with_assets(demo_row_styles())
        .with_child(
            Intro::custom()
                .with_title(Lc::n("PageTop"))
                .with_slogan(Lc::t("mosaic_slogan", &LOC))
                .with_child(columns_block())
                .with_child(gap_block())
                .with_child(items_block())
                .with_child(order_block())
                .with_child(nested_block()),
        )
        .render()
        .await
}

// Alturas de las cajas de muestra, en `rem`, distintas para que se vea el reparto.
const HEIGHTS: [f32; 9] = [4.0, 7.0, 5.0, 9.0, 3.5, 6.0, 8.0, 4.5, 5.5];

// Alturas casi iguales (diferencias de 1 o 2 píxeles), para ver el efecto de la tolerancia.
const NEAR_HEIGHTS: [f32; 9] = [4.0, 4.1, 3.9, 4.0, 4.15, 3.85, 4.0, 4.1, 3.9];

fn columns_block() -> Block {
    Block::new()
        .with_title(Lc::t("mosaic_block_title_columns", &LOC))
        .with_child(caption(
            Lc::t("mosaic_title_columns_auto", &LOC),
            Lc::n("Mosaic::new()"),
        ))
        .with_child(boxes(demo_mosaic(Mosaic::new()), &HEIGHTS))
        .with_child(caption(
            Lc::t("mosaic_title_columns_width", &LOC),
            Lc::n(
                r#"Mosaic::new().with_prop(PropsOp::add_style("--mosaic-column-width", "9rem"))"#,
            ),
        ))
        .with_child(boxes(
            demo_mosaic(
                Mosaic::new().with_prop(PropsOp::add_style("--mosaic-column-width", "9rem")),
            ),
            &HEIGHTS,
        ))
        .with_child(caption(
            Lc::t("mosaic_title_columns_breakpoints", &LOC),
            Lc::n(concat!(
                "Mosaic::new().with_columns(1)",
                ".with_columns_at(Breakpoint::Md, 2)",
                ".with_columns_at(Breakpoint::Xl, 4)",
            )),
        ))
        .with_child(boxes(
            demo_mosaic(
                Mosaic::new()
                    .with_columns(1)
                    .with_columns_at(Breakpoint::Md, 2)
                    .with_columns_at(Breakpoint::Xl, 4),
            ),
            &HEIGHTS,
        ))
}

fn gap_block() -> Block {
    Block::new()
        .with_title(Lc::t("mosaic_block_title_gap", &LOC))
        .with_child(caption(
            Lc::t("mosaic_title_gap_both", &LOC),
            Lc::n("Mosaic::new().with_gap(align::Gap::Both(UnitValue::RelRem(2.0)))"),
        ))
        .with_child(boxes(
            demo_mosaic(Mosaic::new().with_gap(align::Gap::Both(UnitValue::RelRem(2.0)))),
            &HEIGHTS[..6],
        ))
        .with_child(caption(
            Lc::t("mosaic_title_gap_distinct", &LOC),
            Lc::n(concat!(
                "Mosaic::new().with_gap(align::Gap::Distinct {",
                " row: UnitValue::RelRem(2.0), column: UnitValue::RelRem(0.25) })",
            )),
        ))
        .with_child(boxes(
            demo_mosaic(Mosaic::new().with_gap(align::Gap::Distinct {
                row: UnitValue::RelRem(2.0),
                column: UnitValue::RelRem(0.25),
            })),
            &HEIGHTS[..6],
        ))
        .with_child(caption(
            Lc::t("mosaic_title_gap_breakpoints", &LOC),
            Lc::n(concat!(
                "Mosaic::new().with_gap(align::Gap::Both(UnitValue::RelRem(0.25)))",
                ".with_gap_at(Breakpoint::Lg, align::Gap::Both(UnitValue::RelRem(2.0)))",
            )),
        ))
        .with_child(boxes(
            demo_mosaic(
                Mosaic::new()
                    .with_gap(align::Gap::Both(UnitValue::RelRem(0.25)))
                    .with_gap_at(Breakpoint::Lg, align::Gap::Both(UnitValue::RelRem(2.0))),
            ),
            &HEIGHTS[..6],
        ))
}

fn items_block() -> Block {
    Block::new()
        .with_title(Lc::t("mosaic_block_title_items", &LOC))
        .with_child(caption(
            Lc::t("mosaic_title_items_span", &LOC),
            Lc::n("mosaic::Item::new().with_span(2)"),
        ))
        .with_child(boxes(
            demo_mosaic(Mosaic::new().with_columns(3)).with_child(
                mosaic::Item::new()
                    .with_span(2)
                    .with_child(demo_box(Lc::t("mosaic_box_span_2", &LOC), 5.0)),
            ),
            &HEIGHTS[..6],
        ))
        .with_child(caption(
            Lc::t("mosaic_title_items_full", &LOC),
            Lc::n("mosaic::Item::new().with_span(mosaic::Span::Full)"),
        ))
        .with_child(boxes(
            demo_mosaic(Mosaic::new()).with_child(
                mosaic::Item::new()
                    .with_span(mosaic::Span::Full)
                    .with_child(demo_box(Lc::t("mosaic_box_full", &LOC), 3.0)),
            ),
            &HEIGHTS[..6],
        ))
        .with_child(caption(
            Lc::t("mosaic_title_items_span_breakpoints", &LOC),
            Lc::n("mosaic::Item::new().with_span_at(Breakpoint::Xl, 2)"),
        ))
        .with_child(boxes(
            demo_mosaic(Mosaic::new().with_columns(3)).with_child(
                mosaic::Item::new()
                    .with_span_at(Breakpoint::Xl, 2)
                    .with_child(demo_box(Lc::t("mosaic_box_span_xl", &LOC), 5.0)),
            ),
            &HEIGHTS[..6],
        ))
        .with_child(caption(
            Lc::t("mosaic_title_items_pinned", &LOC),
            Lc::n("mosaic::Item::new().with_column(-1)"),
        ))
        .with_child(
            boxes(demo_mosaic(Mosaic::new()), &HEIGHTS[..6]).with_child(
                mosaic::Item::new()
                    .with_column(-1)
                    .with_child(demo_box(Lc::t("mosaic_box_pinned", &LOC), 3.0)),
            ),
        )
}

fn order_block() -> Block {
    Block::new()
        .with_title(Lc::t("mosaic_block_title_order", &LOC))
        .with_child(caption(
            Lc::t("mosaic_title_order_tolerance", &LOC),
            Lc::n("Mosaic::new()"),
        ))
        .with_child(boxes(
            demo_mosaic(Mosaic::new().with_columns(3)),
            &NEAR_HEIGHTS,
        ))
        .with_child(caption(
            Lc::t("mosaic_title_order_shortest", &LOC),
            Lc::n(r#"Mosaic::new().with_prop(PropsOp::add_style("--mosaic-tolerance", "0px"))"#),
        ))
        .with_child(boxes(
            demo_mosaic(
                Mosaic::new()
                    .with_columns(3)
                    .with_prop(PropsOp::add_style("--mosaic-tolerance", "0px")),
            ),
            &NEAR_HEIGHTS,
        ))
        .with_child(caption(
            Lc::t("mosaic_title_order_sequential", &LOC),
            Lc::n(r#"Mosaic::new().with_prop(PropsOp::add_style("--mosaic-tolerance", "9999px"))"#),
        ))
        .with_child(boxes(
            demo_mosaic(
                Mosaic::new()
                    .with_columns(3)
                    .with_prop(PropsOp::add_style("--mosaic-tolerance", "9999px")),
            ),
            &HEIGHTS,
        ))
}

fn nested_block() -> Block {
    Block::new()
        .with_title(Lc::t("mosaic_block_title_nested", &LOC))
        .with_child(caption(
            Lc::t("mosaic_title_nested", &LOC),
            Lc::n(concat!(
                "mosaic::Item::new().with_span(mosaic::Span::Full)",
                ".with_child(Mosaic::new().with_prop(",
                r#"PropsOp::add_style("--mosaic-column-width", "7rem")))"#,
            )),
        ))
        .with_child(boxes(
            demo_mosaic(Mosaic::new()).with_child(
                mosaic::Item::new()
                    .with_span(mosaic::Span::Full)
                    .with_child(nested_boxes(
                        Mosaic::new()
                            .with_prop(PropsOp::add_style("--mosaic-column-width", "7rem")),
                        &[2.0, 3.0, 2.5, 3.5, 2.0, 3.0, 2.5, 2.0],
                    )),
            ),
            &HEIGHTS[..6],
        ))
}

// **< HELPERS >************************************************************************************

// Aspecto fijo de las cajas de muestra: verdes y de esquinas suaves, para distinguir a simple vista
// los ejemplos de Mosaic de los de Grid (`layout-grid.rs`) y Flex (`layout-flex.rs`).
fn demo_box_styles() -> AssetsOp {
    AssetsOp::add_responsive_styles(
        None,
        "mosaic-demo-box",
        [
            ("background-color", "#198754"),
            ("color", "#fff"),
            ("border-radius", "0.375rem"),
            ("display", "flex"),
            ("align-items", "center"),
            ("justify-content", "center"),
            ("padding", "0.5rem 1rem"),
            ("margin", "0"),
            ("text-align", "center"),
        ],
    )
}

// Color de las cajas del mosaico anidado, para distinguirlas de las del mosaico que lo contiene. La
// regla es para las dos clases a la vez, así que tiene más especificidad que la de las cajas.
fn nested_box_styles() -> AssetsOp {
    AssetsOp::add_responsive_styles(
        None,
        "mosaic-demo-box mosaic-demo-box-nested",
        [("background-color", "#6f42c1")],
    )
}

// Aspecto fijo de los mosaicos de muestra: fondo gris para ver los límites del propio mosaico.
fn demo_row_styles() -> AssetsOp {
    AssetsOp::add_responsive_styles(
        None,
        "mosaic-demo-row",
        [
            ("background-color", "#f1f3f5"),
            ("margin", "0 0 1.5rem"),
            ("padding", "0.75rem"),
        ],
    )
}

// Caja con la etiqueta indicada y una altura en `rem`. De ancho completo (`Width::Fluid`): con el
// ancho por defecto, el tema le daría un ancho máximo por punto de corte y la caja no llenaría el
// bloque del mosaico cuando éste fuera más ancho.
fn demo_box(label: Lc, height: f32) -> Container {
    Container::new()
        .with_width(container::Width::Fluid)
        .with_prop(PropsOp::add_classes("mosaic-demo-box"))
        .with_prop(PropsOp::add_style(
            "min-height",
            util::join!(height.to_string(), "rem"),
        ))
        .with_child(Html::with(move |cx| html! { (label.using(cx)) }))
}

// Añade al mosaico una caja numerada por cada altura, en orden.
fn boxes(mut mosaic: Mosaic, heights: &[f32]) -> Mosaic {
    for (i, height) in heights.iter().enumerate() {
        let label = Lc::t("mosaic_box_label", &LOC).with_arg("n", (i + 1).to_string());
        mosaic = mosaic.with_child(demo_box(label, *height));
    }
    mosaic
}

// Como `boxes()`, con las cajas del color del mosaico anidado.
fn nested_boxes(mut mosaic: Mosaic, heights: &[f32]) -> Mosaic {
    for (i, height) in heights.iter().enumerate() {
        let label = Lc::t("mosaic_box_label", &LOC).with_arg("n", (i + 1).to_string());
        let item =
            demo_box(label, *height).with_prop(PropsOp::add_classes("mosaic-demo-box-nested"));
        mosaic = mosaic.with_child(item);
    }
    mosaic
}

// Mosaico de demostración con fondo gris para ver sus límites.
fn demo_mosaic(mosaic: Mosaic) -> Mosaic {
    mosaic.with_prop(PropsOp::add_classes("mosaic-demo-row"))
}

// Título y fragmento de código que introducen cada demostración.
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
    Application::prepare(&LayoutMosaic).await.run().await
}
