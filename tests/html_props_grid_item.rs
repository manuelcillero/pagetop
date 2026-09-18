use pagetop::prelude::*;

#[pagetop::test]
async fn default_grid_item_adds_nothing() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(GridItem::new());
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();

    assert_eq!(html, "<span></span>");
    assert!(cx.render_assets().into_string().is_empty());
}

#[pagetop::test]
async fn column_span_adds_grid_column_style() {
    let mut cx = Context::default();
    let props =
        Props::default().with_prop(GridItem::new().with_column(grid::ItemPlacement::Span(2)));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_grid-item-column_span-2_""#));
    assert!(assets.contains("_grid-item-column_span-2_{grid-column:span 2}"));
}

#[pagetop::test]
async fn column_line_adds_grid_column_style() {
    let mut cx = Context::default();
    let props =
        Props::default().with_prop(GridItem::new().with_column(grid::ItemPlacement::Line(3)));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_grid-item-column_3_""#));
    assert!(assets.contains("_grid-item-column_3_{grid-column:3}"));
}

#[pagetop::test]
async fn column_range_sanitizes_the_slash_and_space_in_its_class_name() {
    let mut cx = Context::default();
    let props =
        Props::default().with_prop(GridItem::new().with_column(grid::ItemPlacement::Range(2, 4)));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    // "2 / 4" is a valid CSS value (with its spaces and slash), but not a valid class name
    // fragment: `value_to_token()` replaces both characters so the class stays a single token.
    assert!(html.contains(r#"class="_grid-item-column_2---4_""#));
    assert!(assets.contains("_grid-item-column_2---4_{grid-column:2 / 4}"));
}

#[pagetop::test]
async fn row_adds_grid_row_style() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(GridItem::new().with_row(grid::ItemPlacement::Line(2)));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_grid-item-row_2_""#));
    assert!(assets.contains("_grid-item-row_2_{grid-row:2}"));
}

#[pagetop::test]
async fn justify_self_adds_matching_style() {
    let mut cx = Context::default();
    let props =
        Props::default().with_prop(GridItem::new().with_justify_self(grid::ItemJustify::End));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_grid-item-justify-self_end_""#));
    assert!(assets.contains("_grid-item-justify-self_end_{justify-self:end}"));
}

#[pagetop::test]
async fn align_self_adds_matching_style() {
    let mut cx = Context::default();
    let props =
        Props::default().with_prop(GridItem::new().with_align_self(align::ItemSelf::Center));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_grid-item-align-self_center_""#));
    assert!(assets.contains("_grid-item-align-self_center_{align-self:center}"));
}

#[pagetop::test]
async fn combines_several_facets_in_one_call() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(
        GridItem::new()
            .with_column(grid::ItemPlacement::Span(2))
            .with_row(grid::ItemPlacement::Line(2))
            .with_justify_self(grid::ItemJustify::Stretch)
            .with_align_self(align::ItemSelf::Start),
    );
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains("_grid-item-column_span-2_"));
    assert!(html.contains("_grid-item-row_2_"));
    assert!(html.contains("_grid-item-justify-self_stretch_"));
    assert!(html.contains("_grid-item-align-self_flex-start_"));
    assert!(assets.contains("_grid-item-column_span-2_{grid-column:span 2}"));
    assert!(assets.contains("_grid-item-row_2_{grid-row:2}"));
    assert!(assets.contains("_grid-item-justify-self_stretch_{justify-self:stretch}"));
    assert!(assets.contains("_grid-item-align-self_flex-start_{align-self:flex-start}"));
}

#[pagetop::test]
async fn column_at_a_breakpoint_combines_token_and_suffix() {
    let mut cx = Context::default();
    let props = Props::default()
        .with_prop(GridItem::new().with_column_at(Breakpoint::Md, grid::ItemPlacement::Span(3)));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_grid-item-column_span-3_md_""#));
    assert!(
        assets
            .contains("@media(min-width:768px){._grid-item-column_span-3_md_{grid-column:span 3}}")
    );
}

#[pagetop::test]
async fn base_and_breakpoint_values_generate_one_class_each() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(
        GridItem::new()
            .with_align_self(align::ItemSelf::Default)
            .with_column(grid::ItemPlacement::Span(2))
            .with_column_at(Breakpoint::Lg, grid::ItemPlacement::Span(4)),
    );
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();

    // `ItemSelf::Default` resolves to an empty CSS value, so it adds no class at all.
    assert!(html.contains(r#"class="_grid-item-column_span-2_ _grid-item-column_span-4_lg_""#));
    assert!(!html.contains("_grid-item-align-self_"));
}

#[pagetop::test]
async fn from_grid_item_for_props_op() {
    let mut cx = Context::default();
    let item = GridItem::new().with_column(grid::ItemPlacement::Span(2));
    let props = Props::default().with_prop(item);
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_grid-item-column_span-2_""#));
    assert!(assets.contains("_grid-item-column_span-2_{grid-column:span 2}"));
}
