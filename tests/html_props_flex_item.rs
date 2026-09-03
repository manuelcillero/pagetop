use pagetop::prelude::*;

#[pagetop::test]
async fn default_flex_item_adds_nothing() {
    let props = Props::default().with_prop(PropsOp::flex_item(FlexItem::new()));

    assert_eq!(props.get_classes(), None);
    assert_eq!(props.get_styles(), None);
}

#[pagetop::test]
async fn grow_adds_flex_grow_style() {
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_grow(flex::ItemGrow::Is1),
    ));

    assert_eq!(props.get_styles(), Some("flex-grow: 1".to_string()));
}

#[pagetop::test]
async fn shrink_adds_flex_shrink_style() {
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_shrink(flex::ItemShrink::Is0),
    ));

    assert_eq!(props.get_styles(), Some("flex-shrink: 0".to_string()));
}

#[pagetop::test]
async fn align_self_adds_matching_style() {
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_align_self(flex::ItemAlign::Center),
    ));

    assert_eq!(props.get_styles(), Some("align-self: center".to_string()));
}

#[pagetop::test]
async fn order_adds_matching_style() {
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_order(flex::ItemOrder::First),
    ));

    assert_eq!(props.get_styles(), Some("order: -129".to_string()));
}

#[pagetop::test]
async fn size_percent_adds_flex_basis_style() {
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_size(flex::ItemSize::Percent33),
    ));

    assert_eq!(props.get_styles(), Some("flex-basis: 33.3333%".to_string()));
}

#[pagetop::test]
async fn size_custom_adds_flex_basis_style() {
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_size(flex::ItemSize::Custom(UnitValue::Zero)),
    ));

    assert_eq!(props.get_classes(), None);
    assert_eq!(props.get_styles(), Some("flex-basis: 0".to_string()));
}

#[pagetop::test]
async fn offset_percent_adds_margin_inline_start_style() {
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_offset(flex::ItemOffset::Percent33),
    ));

    assert_eq!(
        props.get_styles(),
        Some("margin-inline-start: 33.3333%".to_string())
    );
}

#[pagetop::test]
async fn offset_custom_adds_margin_inline_start_style() {
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new().with_offset(flex::ItemOffset::Custom(UnitValue::Px(16))),
    ));

    assert_eq!(
        props.get_styles(),
        Some("margin-inline-start: 16px".to_string())
    );
}

#[pagetop::test]
async fn combines_several_facets_in_one_call() {
    let props = Props::default().with_prop(PropsOp::flex_item(
        FlexItem::new()
            .with_grow(flex::ItemGrow::Is1)
            .with_shrink(flex::ItemShrink::Is0)
            .with_align_self(flex::ItemAlign::Start)
            .with_order(flex::ItemOrder::Is2)
            .with_size(flex::ItemSize::Custom(UnitValue::Zero))
            .with_offset(flex::ItemOffset::Percent10),
    ));

    assert_eq!(
        props.get_styles(),
        Some(
            "flex-grow: 1; flex-shrink: 0; align-self: flex-start; order: 2; flex-basis: 0; \
             margin-inline-start: 10%"
                .to_string()
        )
    );
    assert_eq!(props.get_classes(), None);
}

#[pagetop::test]
async fn from_flex_item_for_props_op() {
    let item = FlexItem::new().with_grow(flex::ItemGrow::Is1);
    let props = Props::default().with_prop(item.into());

    assert_eq!(props.get_styles(), Some("flex-grow: 1".to_string()));
}
