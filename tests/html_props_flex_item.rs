use pagetop::prelude::*;

#[pagetop::test]
async fn default_flex_item_adds_nothing() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(PropsOp::flex_item(FlexItem::new()));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();

    assert_eq!(html, "<span></span>");
    assert!(cx.render_assets().into_string().is_empty());
}

#[pagetop::test]
async fn grow_adds_flex_grow_style() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_grow(flex::ItemGrow::Is1),
    ));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_flex-item-grow_1_""#));
    assert!(assets.contains("_flex-item-grow_1_{flex-grow:1}"));
}

#[pagetop::test]
async fn shrink_adds_flex_shrink_style() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_shrink(flex::ItemShrink::Is0),
    ));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_flex-item-shrink_0_""#));
    assert!(assets.contains("_flex-item-shrink_0_{flex-shrink:0}"));
}

#[pagetop::test]
async fn align_self_adds_matching_style() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_align_self(flex::ItemAlign::Center),
    ));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_flex-item-align_center_""#));
    assert!(assets.contains("_flex-item-align_center_{align-self:center}"));
}

#[pagetop::test]
async fn order_adds_matching_style() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_order(flex::ItemOrder::First),
    ));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_flex-item-order_-129_""#));
    assert!(assets.contains("_flex-item-order_-129_{order:-129}"));
}

#[pagetop::test]
async fn size_percent_adds_flex_basis_style() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_size(flex::ItemSize::Percent33),
    ));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_flex-item-basis_33_3333pct_""#));
    assert!(assets.contains("_flex-item-basis_33_3333pct_{flex-basis:33.3333%}"));
}

#[pagetop::test]
async fn size_custom_adds_flex_basis_style() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_size(flex::ItemSize::Custom(UnitValue::Zero)),
    ));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_flex-item-basis_0_""#));
    assert!(assets.contains("_flex-item-basis_0_{flex-basis:0}"));
}

#[pagetop::test]
async fn offset_percent_adds_margin_inline_start_style() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_offset(flex::ItemOffset::Percent33),
    ));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_flex-item-offset_33_3333pct_""#));
    assert!(assets.contains("_flex-item-offset_33_3333pct_{margin-inline-start:33.3333%}"));
}

#[pagetop::test]
async fn offset_custom_adds_margin_inline_start_style() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_offset(flex::ItemOffset::Custom(UnitValue::Px(16))),
    ));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_flex-item-offset_16px_""#));
    assert!(assets.contains("_flex-item-offset_16px_{margin-inline-start:16px}"));
}

#[pagetop::test]
async fn combines_several_facets_in_one_call() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new()
            .with_grow(flex::ItemGrow::Is1)
            .with_shrink(flex::ItemShrink::Is0)
            .with_align_self(flex::ItemAlign::Start)
            .with_order(flex::ItemOrder::Is2)
            .with_size(flex::ItemSize::Custom(UnitValue::Zero))
            .with_offset(flex::ItemOffset::Percent10),
    ));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains("_flex-item-grow_1_"));
    assert!(html.contains("_flex-item-shrink_0_"));
    assert!(html.contains("_flex-item-align_flex-start_"));
    assert!(html.contains("_flex-item-order_2_"));
    assert!(html.contains("_flex-item-basis_0_"));
    assert!(html.contains("_flex-item-offset_10pct_"));
    assert!(assets.contains("_flex-item-grow_1_{flex-grow:1}"));
    assert!(assets.contains("_flex-item-shrink_0_{flex-shrink:0}"));
    assert!(assets.contains("_flex-item-align_flex-start_{align-self:flex-start}"));
    assert!(assets.contains("_flex-item-order_2_{order:2}"));
    assert!(assets.contains("_flex-item-basis_0_{flex-basis:0}"));
    assert!(assets.contains("_flex-item-offset_10pct_{margin-inline-start:10%}"));
}

#[pagetop::test]
async fn from_flex_item_for_props_op() {
    let mut cx = Context::default();
    let item = FlexItem::new().with_grow(flex::ItemGrow::Is1);
    let props = Props::default().with_prop(item.into());
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_flex-item-grow_1_""#));
    assert!(assets.contains("_flex-item-grow_1_{flex-grow:1}"));
}
