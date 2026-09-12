use pagetop::prelude::*;

#[pagetop::test]
async fn default_margin_adds_nothing() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(Margin::new());
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();

    assert_eq!(html, "<span></span>");
    assert!(cx.render_assets().into_string().is_empty());
}

#[pagetop::test]
async fn default_padding_adds_nothing() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(Padding::new());
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();

    assert_eq!(html, "<span></span>");
    assert!(cx.render_assets().into_string().is_empty());
}

#[pagetop::test]
async fn margin_top_adds_matching_style() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(Margin::new().with_top(UnitValue::RelRem(1.5)));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_margin-top_1_5rem_""#));
    assert!(assets.contains("_margin-top_1_5rem_{margin-top:1.5rem}"));
}

#[pagetop::test]
async fn margin_x_sets_start_and_end() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(Margin::new().with_x(UnitValue::Auto));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_margin-start_auto_ _margin-end_auto_""#));
    assert!(assets.contains("_margin-start_auto_{margin-inline-start:auto}"));
    assert!(assets.contains("_margin-end_auto_{margin-inline-end:auto}"));
}

#[pagetop::test]
async fn margin_y_sets_top_and_bottom() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(Margin::new().with_y(UnitValue::Zero));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_margin-top_0_ _margin-bottom_0_""#));
    assert!(assets.contains("_margin-top_0_{margin-top:0}"));
    assert!(assets.contains("_margin-bottom_0_{margin-bottom:0}"));
}

#[pagetop::test]
async fn margin_all_sets_every_side() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(Margin::new().with_all(UnitValue::Px(8)));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();

    assert!(html.contains(
        r#"class="_margin-top_8px_ _margin-bottom_8px_ _margin-start_8px_ _margin-end_8px_""#
    ));
}

#[pagetop::test]
async fn margin_at_a_breakpoint_combines_token_and_suffix() {
    let mut cx = Context::default();
    let props =
        Props::default().with_prop(Margin::new().with_top_at(Breakpoint::Md, UnitValue::Px(16)));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_margin-top_16px_md_""#));
    assert!(assets.contains("@media(min-width:768px){._margin-top_16px_md_{margin-top:16px}}"));
}

#[pagetop::test]
async fn successive_with_prop_calls_merge_margin_sides() {
    let mut cx = Context::default();
    let props = Props::default()
        .with_prop(Margin::new().with_top(UnitValue::Px(8)))
        .with_prop(Margin::new().with_bottom(UnitValue::Px(16)));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();

    // Each `with_prop()` only sets one side; `Margin::merge()` keeps both instead of letting the
    // second call overwrite the first one entirely.
    assert!(html.contains(r#"class="_margin-top_8px_ _margin-bottom_16px_""#));
}

#[pagetop::test]
async fn later_with_prop_call_overrides_same_side() {
    let mut cx = Context::default();
    let props = Props::default()
        .with_prop(Margin::new().with_top(UnitValue::Px(8)))
        .with_prop(Margin::new().with_top(UnitValue::Px(24)));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();

    assert!(html.contains(r#"class="_margin-top_24px_""#));
    assert!(!html.contains("_margin-top_8px_"));
}

#[pagetop::test]
async fn padding_top_adds_matching_style() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(Padding::new().with_top(UnitValue::RelRem(1.0)));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_padding-top_1rem_""#));
    assert!(assets.contains("_padding-top_1rem_{padding-top:1rem}"));
}

#[pagetop::test]
async fn padding_all_sets_every_side() {
    let mut cx = Context::default();
    let props = Props::default().with_prop(Padding::new().with_all(UnitValue::RelRem(1.0)));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();

    assert!(html.contains(
        r#"class="_padding-top_1rem_ _padding-bottom_1rem_ _padding-start_1rem_ _padding-end_1rem_""#
    ));
}

#[pagetop::test]
async fn padding_at_a_breakpoint_combines_token_and_suffix() {
    let mut cx = Context::default();
    let props = Props::default()
        .with_prop(Padding::new().with_bottom_at(Breakpoint::Lg, UnitValue::RelRem(2.0)));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains(r#"class="_padding-bottom_2rem_lg_""#));
    assert!(
        assets.contains("@media(min-width:992px){._padding-bottom_2rem_lg_{padding-bottom:2rem}}")
    );
}

#[pagetop::test]
async fn padding_auto_is_ignored() {
    let mut cx = Context::default();
    // CSS does not support `padding: auto`; unlike `Margin`, a side set to `Auto` adds nothing.
    let props = Props::default().with_prop(Padding::new().with_all(UnitValue::Auto));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();

    assert_eq!(html, "<span></span>");
    assert!(cx.render_assets().into_string().is_empty());
}

#[pagetop::test]
async fn margin_and_padding_combine_on_same_component() {
    let mut cx = Context::default();
    let props = Props::default()
        .with_prop(Margin::new().with_bottom(UnitValue::Px(8)))
        .with_prop(Padding::new().with_top(UnitValue::Px(4)));
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();

    // Class order follows `Props::unpack()`: margin before padding.
    assert!(html.contains(r#"class="_margin-bottom_8px_ _padding-top_4px_""#));
}

#[pagetop::test]
async fn from_margin_for_props_op() {
    let mut cx = Context::default();
    let margin = Margin::new().with_top(UnitValue::Px(8));
    let props = Props::default().with_prop(margin);
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();

    assert!(html.contains(r#"class="_margin-top_8px_""#));
}

#[pagetop::test]
async fn from_padding_for_props_op() {
    let mut cx = Context::default();
    let padding = Padding::new().with_top(UnitValue::Px(8));
    let props = Props::default().with_prop(padding);
    let html = html! { span (props.unpack(&mut cx)) {} }.into_string();

    assert!(html.contains(r#"class="_padding-top_8px_""#));
}
