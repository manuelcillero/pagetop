use pagetop::prelude::*;

// **< Mosaic >*************************************************************************************

#[pagetop::test]
async fn wraps_each_child_in_its_own_item() {
    let mut mosaic = Mosaic::new()
        .with_id("news")
        .with_child(Html::with(|_| html! { p { "One" } }))
        .with_child(Html::with(|_| html! { p { "Two" } }));
    let html = mosaic.render(&mut Context::default()).await.into_string();

    assert_eq!(
        html,
        concat!(
            r#"<div id="news" class="mosaic">"#,
            r#"<div class="mosaic-item"><p>One</p></div>"#,
            r#"<div class="mosaic-item"><p>Two</p></div>"#,
            "</div>",
        )
    );
}

#[pagetop::test]
async fn children_that_render_nothing_take_no_room() {
    let mut mosaic = Mosaic::new()
        .with_child(Html::with(|_| html! {}))
        .with_child(Html::with(|_| html! { p { "Only" } }));
    let html = mosaic.render(&mut Context::default()).await.into_string();

    assert_eq!(html.matches("mosaic-item").count(), 1, "{html}");
}

#[pagetop::test]
async fn without_children_to_show_it_renders_nothing_nor_its_assets() {
    let mut cx = Context::default();
    let mut mosaic = Mosaic::new().with_child(Html::with(|_| html! {}));
    let html = mosaic.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.is_empty(), "{html}");
    assert!(!assets.contains("mosaic"), "{assets}");
}

#[pagetop::test]
async fn it_brings_its_own_stylesheet_and_script() {
    let mut cx = Context::default();
    let mut mosaic = Mosaic::new().with_child(Html::with(|_| html! { p { "One" } }));
    mosaic.render(&mut cx).await;
    let assets = cx.render_assets().into_string();

    assert!(assets.contains("/pagetop/css/mosaic.min.css"), "{assets}");
    assert!(assets.contains("/pagetop/js/mosaic.min.js"), "{assets}");
}

#[pagetop::test]
async fn without_columns_it_lays_out_by_its_minimum_column_width() {
    let mut cx = Context::default();
    let mut mosaic = Mosaic::new().with_child(Html::with(|_| html! { p { "One" } }));
    let html = mosaic.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.starts_with(r#"<div class="mosaic">"#), "{html}");
    assert!(!assets.contains("<style>"), "{assets}");
}

#[pagetop::test]
async fn columns_are_set_from_each_breakpoint_for_both_layouts() {
    let mut cx = Context::default();
    let mut mosaic = Mosaic::new()
        .with_columns(1)
        .with_columns_at(Breakpoint::Md, 2)
        .with_columns_at(Breakpoint::Xl, 3)
        .with_child(Html::with(|_| html! { p { "One" } }));
    let html = mosaic.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(
        html.contains(
            r#"class="mosaic _mosaic-columns_1_ _mosaic-columns_2_md_ _mosaic-columns_3_xl_""#
        ),
        "{html}"
    );
    // Mobile-first: the base rule first, then each breakpoint wrapped in its own media query.
    let base = assets
        .find("._mosaic-columns_1_{columns:1;grid-template-columns:repeat(1,minmax(0,1fr))}")
        .expect(&assets);
    let md = assets
        .find(concat!(
            "@media(min-width:768px){._mosaic-columns_2_md_",
            "{columns:2;grid-template-columns:repeat(2,minmax(0,1fr))}}",
        ))
        .expect(&assets);
    let xl = assets
        .find(concat!(
            "@media(min-width:1200px){._mosaic-columns_3_xl_",
            "{columns:3;grid-template-columns:repeat(3,minmax(0,1fr))}}",
        ))
        .expect(&assets);
    assert!(base < md && md < xl, "{assets}");
}

#[pagetop::test]
#[should_panic(expected = "Mosaic needs at least one column")]
async fn zero_columns_is_a_programming_error() {
    let _ = Mosaic::new().with_columns_at(Breakpoint::Md, 0);
}

#[pagetop::test]
async fn gaps_are_set_from_each_breakpoint_through_the_mosaic_properties() {
    let mut cx = Context::default();
    let mut mosaic = Mosaic::new()
        .with_gap(align::Gap::Both(UnitValue::RelRem(1.0)))
        .with_gap_at(
            Breakpoint::Md,
            align::Gap::Distinct {
                row: UnitValue::Px(24),
                column: UnitValue::None,
            },
        )
        .with_child(Html::with(|_| html! { p { "One" } }));
    let html = mosaic.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(
        html.contains(r#"class="mosaic _mosaic-gap_1rem-1rem_ _mosaic-gap_24px-none_md_""#),
        "{html}"
    );
    assert!(
        assets.contains("._mosaic-gap_1rem-1rem_{--mosaic-row-gap:1rem;--mosaic-column-gap:1rem}"),
        "{assets}"
    );
    // Only the measurable value: the gap between columns keeps the one below this breakpoint.
    assert!(
        assets
            .contains("@media(min-width:768px){._mosaic-gap_24px-none_md_{--mosaic-row-gap:24px}}"),
        "{assets}"
    );
}

// **< mosaic::Item >*******************************************************************************

#[pagetop::test]
async fn an_item_is_the_block_itself_and_carries_its_settings() {
    let mut mosaic = Mosaic::new()
        .with_child(
            mosaic::Item::new()
                .with_id("wide")
                .with_span(2)
                .with_child(Html::with(|_| html! { p { "Wide" } })),
        )
        .with_child(mosaic::Item::new().with_child(Html::with(|_| html! { p { "Narrow" } })))
        .with_child(Html::with(|_| html! { p { "Plain" } }));
    let html = mosaic.render(&mut Context::default()).await.into_string();

    assert_eq!(
        html,
        concat!(
            r#"<div class="mosaic">"#,
            r#"<div id="wide" class="mosaic-item _mosaic-span_2_"><p>Wide</p></div>"#,
            r#"<div class="mosaic-item"><p>Narrow</p></div>"#,
            r#"<div class="mosaic-item"><p>Plain</p></div>"#,
            "</div>",
        )
    );
}

#[pagetop::test]
async fn an_item_span_is_set_from_each_breakpoint_and_can_take_every_column() {
    let mut cx = Context::default();
    let mut item = mosaic::Item::new()
        .with_span(mosaic::Span::Full)
        .with_span_at(Breakpoint::Md, 2)
        .with_child(Html::with(|_| html! { p { "Item" } }));
    let html = item.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(
        html.contains(r#"class="mosaic-item _mosaic-span_full_ _mosaic-span_2_md_""#),
        "{html}"
    );
    // Every column also without JavaScript (`column-span`), undone where it takes a number.
    assert!(
        assets.contains("._mosaic-span_full_{--mosaic-span:full;column-span:all}"),
        "{assets}"
    );
    assert!(
        assets.contains(
            "@media(min-width:768px){._mosaic-span_2_md_{--mosaic-span:2;column-span:none}}"
        ),
        "{assets}"
    );
}

#[pagetop::test]
async fn an_item_can_be_pinned_to_a_column_from_a_breakpoint() {
    let mut cx = Context::default();
    let mut item = mosaic::Item::new()
        .with_column_at(Breakpoint::Md, -1)
        .with_column_at(Breakpoint::Xl, 0)
        .with_child(Html::with(|_| html! { p { "Notice" } }));
    let html = item.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(
        html.contains(r#"class="mosaic-item _mosaic-column_-1_md_ _mosaic-column_0_xl_""#),
        "{html}"
    );
    assert!(
        assets.contains("@media(min-width:768px){._mosaic-column_-1_md_{--mosaic-column:-1}}"),
        "{assets}"
    );
    // Column 0 lays it out with the others again from that breakpoint.
    assert!(
        assets.contains("@media(min-width:1200px){._mosaic-column_0_xl_{--mosaic-column:0}}"),
        "{assets}"
    );
}

#[pagetop::test]
async fn an_item_without_content_takes_no_room() {
    let mut cx = Context::default();
    let mut mosaic = Mosaic::new()
        .with_child(mosaic::Item::new().with_span(2))
        .with_child(Html::with(|_| html! { p { "Only" } }));
    let html = mosaic.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert_eq!(html.matches("mosaic-item").count(), 1, "{html}");
    assert!(!assets.contains("_mosaic-span_"), "{assets}");
}

#[pagetop::test]
#[should_panic(expected = "A mosaic item needs at least one column")]
async fn an_item_of_zero_columns_is_a_programming_error() {
    let _ = mosaic::Item::new().with_span_at(Breakpoint::Md, 0);
}
