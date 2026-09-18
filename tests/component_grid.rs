use pagetop::prelude::*;

// **< Grid >***************************************************************************************

#[pagetop::test]
async fn is_not_rendered_when_empty() {
    let mut container = Grid::default();
    let html = container.render(&mut Context::default()).await;

    assert!(html.is_empty());
}

#[pagetop::test]
async fn renders_a_div_element() {
    let mut container = Grid::default().with_child(Lc::n("x"));
    let html = container
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(html.starts_with("<div"));
    assert!(html.ends_with("</div>"));
}

#[pagetop::test]
async fn without_grid_no_style_attribute_is_added() {
    let mut container = Grid::default().with_child(Lc::n("x"));
    let html = container
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(!html.contains("style="));
}

#[pagetop::test]
async fn default_grid_adds_only_display_grid() {
    let mut cx = Context::default();
    let mut container = Grid::new().with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_grid_""#));
    assert!(assets.contains("_grid_{display:grid}"));
}

#[pagetop::test]
async fn columns_and_rows_add_their_matching_styles() {
    let mut cx = Context::default();
    let mut container = Grid::new()
        .with_columns(grid::Tracks::repeat(3, grid::AxisTrack::Fraction(1.0)))
        .with_rows(grid::Tracks::new().with_track(grid::AxisTrack::Fixed(UnitValue::Px(80))))
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains("_grid-columns_1fr-1fr-1fr_"));
    assert!(html.contains("_grid-rows_80px_"));
    assert!(assets.contains("_grid-columns_1fr-1fr-1fr_{grid-template-columns:1fr 1fr 1fr}"));
    assert!(assets.contains("_grid-rows_80px_{grid-template-rows:80px}"));
}

#[pagetop::test]
async fn auto_columns_and_auto_rows_add_their_matching_styles() {
    let mut cx = Context::default();
    let mut container = Grid::new()
        .with_auto_columns(grid::AxisTrack::MinContent)
        .with_auto_rows(grid::AxisTrack::Fixed(UnitValue::RelRem(3.0)))
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains("_grid-auto-columns_min-content_"));
    assert!(html.contains("_grid-auto-rows_3rem_"));
    assert!(assets.contains("_grid-auto-columns_min-content_{grid-auto-columns:min-content}"));
    assert!(assets.contains("_grid-auto-rows_3rem_{grid-auto-rows:3rem}"));
}

#[pagetop::test]
async fn auto_flow_dense_sanitizes_the_space_in_its_class_name() {
    let mut cx = Context::default();
    let mut container = Grid::new()
        .with_auto_flow(grid::AutoFlow::RowDense)
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    // `AutoFlow::RowDense` resolves to "row dense" (with a space); as a class name it must stay a
    // single, valid token (regression test: this used to split into two class list entries).
    assert!(html.contains(r#"class="_grid_ _grid-auto-flow_row-dense_""#));
    assert!(assets.contains("_grid-auto-flow_row-dense_{grid-auto-flow:row dense}"));
}

#[pagetop::test]
async fn justify_items_and_align_items_add_their_matching_styles() {
    let mut cx = Context::default();
    let mut container = Grid::new()
        .with_justify_items(grid::DefaultJustify::Center)
        .with_align_items(align::Items::Stretch)
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains("_grid-justify-items_center_"));
    assert!(html.contains("_grid-align-items_stretch_"));
    assert!(assets.contains("_grid-justify-items_center_{justify-items:center}"));
    assert!(assets.contains("_grid-align-items_stretch_{align-items:stretch}"));
}

#[pagetop::test]
async fn justify_content_and_align_content_add_their_matching_styles() {
    let mut cx = Context::default();
    let mut container = Grid::new()
        .with_justify_content(grid::ContentJustify::SpaceBetween)
        .with_align_content(align::Content::Center)
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains("_grid-justify-content_space-between_"));
    assert!(html.contains("_grid-align-content_center_"));
    assert!(assets.contains("_grid-justify-content_space-between_{justify-content:space-between}"));
    assert!(assets.contains("_grid-align-content_center_{align-content:center}"));
}

#[pagetop::test]
async fn gap_both_adds_a_single_gap_style() {
    let mut cx = Context::default();
    let mut container = Grid::new()
        .with_gap(align::Gap::Both(UnitValue::RelRem(0.5)))
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains("_grid-gap_0_5rem_"));
    assert!(assets.contains("_grid-gap_0_5rem_{gap:0.5rem}"));
}

#[pagetop::test]
async fn gap_distinct_adds_row_and_column_gap_styles() {
    let mut cx = Context::default();
    let mut container = Grid::new()
        .with_gap(align::Gap::Distinct {
            row: UnitValue::Px(4),
            column: UnitValue::Px(8),
        })
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains("_grid-row-gap_4px_"));
    assert!(html.contains("_grid-column-gap_8px_"));
    assert!(assets.contains("_grid-row-gap_4px_{row-gap:4px}"));
    assert!(assets.contains("_grid-column-gap_8px_{column-gap:8px}"));
}

#[pagetop::test]
async fn inline_grid_uses_its_own_display_value() {
    let mut cx = Context::default();
    let mut container = Grid::inline().with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_inline-grid_""#));
    assert!(assets.contains("_inline-grid_{display:inline-grid}"));
}

#[pagetop::test]
async fn inline_grid_at_a_breakpoint_adds_its_suffix() {
    let mut cx = Context::default();
    let mut container = Grid::inline_at(Breakpoint::Lg).with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_inline-grid_lg_""#));
    assert!(assets.contains("@media(min-width:992px){._inline-grid_lg_{display:inline-grid}}"));
}

#[pagetop::test]
async fn grid_and_grid_item_classes_keep_their_order() {
    let mut cx = Context::default();
    let mut container = Grid::new()
        .with_columns(grid::Tracks::repeat(2, grid::AxisTrack::Fraction(1.0)))
        .with_prop(PropsOp::add_classes("own"))
        .with_prop(GridItem::new().with_column(grid::ItemPlacement::Span(2)))
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();

    // The component's own classes come first, then those of `Grid`, then those of `GridItem`,
    // which share a single accumulator in `Props::unpack_with_classes()`.
    assert!(
        html.contains(r#"class="own _grid_ _grid-columns_1fr-1fr_ _grid-item-column_span-2_""#)
    );
}

#[pagetop::test]
async fn grid_without_display_adds_no_class_of_its_own() {
    let mut cx = Context::default();
    let mut container = Grid::default()
        .with_prop(PropsOp::add_classes("own"))
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();

    assert!(html.contains(r#"class="own""#));
    assert!(cx.render_assets().into_string().is_empty());
}

#[pagetop::test]
async fn grid_breakpoints_add_their_suffix_and_wrap_rules_in_media_queries() {
    let mut cx = Context::default();
    let mut container = Grid::at(Breakpoint::Md)
        .with_columns_at(
            Breakpoint::Lg,
            grid::Tracks::repeat(3, grid::AxisTrack::Fraction(1.0)),
        )
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_grid_md_ _grid-columns_1fr-1fr-1fr_lg_""#));
    assert!(assets.contains("@media(min-width:768px){._grid_md_{display:grid}}"));
    assert!(assets.contains("@media(min-width:992px){._grid-columns_1fr-1fr-1fr_lg_"));
}
