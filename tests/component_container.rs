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
    let mut container = Container::new()
        .with_flex(Flex::row())
        .with_child(Lc::n("x"));
    let html = container
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(html.contains(r#"style="display: flex""#));
}

#[pagetop::test]
async fn column_direction_adds_flex_direction_style() {
    let mut container = Container::new()
        .with_flex(Flex::column())
        .with_child(Lc::n("x"));
    let html = container
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(html.contains("display: flex"));
    assert!(html.contains("flex-direction: column"));
}

#[pagetop::test]
async fn wrap_justify_and_align_add_their_matching_styles() {
    let mut container = Container::new()
        .with_flex(
            Flex::row()
                .with_wrap(flex::Behavior::Wrap)
                .with_justify(flex::ContentJustify::Center)
                .with_align(flex::Align::Center)
                .with_align_content(flex::AlignContent::SpaceBetween),
        )
        .with_child(Lc::n("x"));
    let html = container
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(html.contains("flex-wrap: wrap"));
    assert!(html.contains("justify-content: center"));
    assert!(html.contains("align-items: center"));
    assert!(html.contains("align-content: space-between"));
}

#[pagetop::test]
async fn gap_both_adds_a_single_gap_style() {
    let mut container = Container::new()
        .with_flex(Flex::row().with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))))
        .with_child(Lc::n("x"));
    let html = container
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(html.contains("gap: 0.5rem"));
}

#[pagetop::test]
async fn gap_distinct_adds_row_and_column_gap_styles() {
    let mut container = Container::new()
        .with_flex(Flex::row().with_gap(flex::Gap::Distinct {
            row: UnitValue::Px(4),
            column: UnitValue::Px(8),
        }))
        .with_child(Lc::n("x"));
    let html = container
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(html.contains("row-gap: 4px"));
    assert!(html.contains("column-gap: 8px"));
}

#[pagetop::test]
async fn gap_none_adds_no_gap_style() {
    let mut container = Container::new()
        .with_flex(Flex::row())
        .with_child(Lc::n("x"));
    let html = container
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(!html.contains("gap:"));
}
