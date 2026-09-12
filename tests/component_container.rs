use pagetop::prelude::*;

// **< Container >**********************************************************************************

#[pagetop::test]
async fn is_not_rendered_when_empty() {
    let mut container = Container::new();
    let html = container.render(&mut Context::default()).await;

    assert!(html.is_empty());
}

#[pagetop::test]
async fn default_kind_renders_a_div_element() {
    let mut container = Container::new().with_child(Lc::n("x"));
    let html = container
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(html.starts_with("<div"));
    assert!(html.ends_with("</div>"));
}

#[pagetop::test]
async fn main_kind_renders_a_main_element() {
    let mut container = Container::main().with_child(Lc::n("x"));
    let html = container
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(html.starts_with("<main"));
    assert!(html.ends_with("</main>"));
}

// **< Container + Flex >***************************************************************************

#[pagetop::test]
async fn without_flex_no_style_attribute_is_added() {
    let mut container = Container::new().with_child(Lc::n("x"));
    let html = container
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(!html.contains("style="));
}

#[pagetop::test]
async fn default_flex_adds_only_display_flex() {
    let mut cx = Context::default();
    let mut container = Container::new()
        .with_flex(Flex::new())
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_flex_""#));
    assert!(assets.contains("_flex_{display:flex}"));
}

#[pagetop::test]
async fn column_direction_adds_flex_direction_style() {
    let mut cx = Context::default();
    let mut container = Container::new()
        .with_flex(Flex::new().with_direction(flex::Direction::Column))
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains("_flex_"));
    assert!(html.contains("_flex-direction_column_"));
    assert!(assets.contains("_flex_{display:flex}"));
    assert!(assets.contains("_flex-direction_column_{flex-direction:column}"));
}

#[pagetop::test]
async fn wrap_justify_and_align_add_their_matching_styles() {
    let mut cx = Context::default();
    let mut container = Container::new()
        .with_flex(
            Flex::new()
                .with_wrap(flex::Behavior::Wrap)
                .with_justify(flex::ContentJustify::Center)
                .with_align(flex::Align::Center)
                .with_align_content(flex::AlignContent::SpaceBetween),
        )
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains("_flex-wrap_wrap_"));
    assert!(html.contains("_flex-justify_center_"));
    assert!(html.contains("_flex-align-items_center_"));
    assert!(html.contains("_flex-align-content_space-between_"));
    assert!(assets.contains("_flex-wrap_wrap_{flex-wrap:wrap}"));
    assert!(assets.contains("_flex-justify_center_{justify-content:center}"));
    assert!(assets.contains("_flex-align-items_center_{align-items:center}"));
    assert!(assets.contains("_flex-align-content_space-between_{align-content:space-between}"));
}

#[pagetop::test]
async fn gap_both_adds_a_single_gap_style() {
    let mut cx = Context::default();
    let mut container = Container::new()
        .with_flex(Flex::new().with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))))
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains("_flex-gap_0_5rem_"));
    assert!(assets.contains("_flex-gap_0_5rem_{gap:0.5rem}"));
}

#[pagetop::test]
async fn gap_distinct_adds_row_and_column_gap_styles() {
    let mut cx = Context::default();
    let mut container = Container::new()
        .with_flex(Flex::new().with_gap(flex::Gap::Distinct {
            row: UnitValue::Px(4),
            column: UnitValue::Px(8),
        }))
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains("_flex-row-gap_4px_"));
    assert!(html.contains("_flex-column-gap_8px_"));
    assert!(assets.contains("_flex-row-gap_4px_{row-gap:4px}"));
    assert!(assets.contains("_flex-column-gap_8px_{column-gap:8px}"));
}

#[pagetop::test]
async fn gap_none_adds_no_gap_style() {
    let mut cx = Context::default();
    let mut container = Container::new()
        .with_flex(Flex::new())
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(!html.contains("gap"));
    assert!(!assets.contains("gap"));
}

#[pagetop::test]
async fn inline_flex_uses_its_own_display_value() {
    let mut cx = Context::default();
    let mut container = Container::new()
        .with_flex(Flex::inline())
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_inline-flex_""#));
    assert!(assets.contains("_inline-flex_{display:inline-flex}"));
}

#[pagetop::test]
async fn inline_flex_at_a_breakpoint_adds_its_suffix() {
    let mut cx = Context::default();
    let mut container = Container::new()
        .with_flex(Flex::inline_at(Breakpoint::Lg))
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_inline-flex_lg_""#));
    assert!(assets.contains("@media(min-width:992px){._inline-flex_lg_{display:inline-flex}}"));
}

#[pagetop::test]
async fn flex_and_flex_item_classes_keep_their_order() {
    let mut cx = Context::default();
    let mut container = Container::new()
        .with_prop(PropsOp::add_classes("own"))
        .with_flex(Flex::new().with_direction(flex::Direction::Column))
        .with_prop(PropsOp::flex_item(
            FlexItem::new().with_grow(flex::ItemGrow::Is1),
        ))
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();

    // The component's own classes come first, then those of `Flex`, then those of `FlexItem`,
    // which share a single accumulator in `Props::unpack_with_flex()`.
    assert!(html.contains(r#"class="own _flex_ _flex-direction_column_ _flex-item-grow_1_""#));
}

#[pagetop::test]
async fn flex_without_display_adds_no_class_of_its_own() {
    let mut cx = Context::default();
    let mut container = Container::new()
        .with_prop(PropsOp::add_classes("own"))
        .with_flex(Flex::default())
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();

    assert!(html.contains(r#"class="own""#));
    assert!(cx.render_assets().into_string().is_empty());
}

#[pagetop::test]
async fn breakpoints_add_their_suffix_and_wrap_rules_in_media_queries() {
    let mut cx = Context::default();
    let mut container = Container::new()
        .with_flex(
            Flex::at(Breakpoint::Md).with_direction_at(Breakpoint::Lg, flex::Direction::Column),
        )
        .with_child(Lc::n("x"));
    let html = container.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_flex_md_ _flex-direction_column_lg_""#));
    assert!(assets.contains("@media(min-width:768px){._flex_md_{display:flex}}"));
    assert!(assets.contains("@media(min-width:992px){._flex-direction_column_lg_"));
}
