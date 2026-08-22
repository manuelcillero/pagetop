use pagetop::prelude::*;

#[pagetop::test]
async fn label_is_rendered_when_set() {
    let mut badge = Badge::labeled(Lc::n("Admin"));
    let html = badge.render(&mut Context::default()).await.into_string();

    assert!(html.contains("Admin"));
}

#[pagetop::test]
async fn label_can_be_cleared_with_lc_none() {
    let mut badge = Badge::labeled(Lc::n("Admin")).with_label(Lc::none());
    let html = badge.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("Admin"));
    // The badge itself must still render.
    assert!(html.contains("<span"));
}

#[pagetop::test]
async fn renders_as_a_span_element() {
    let mut badge = Badge::labeled(Lc::n("Admin"));
    let html = badge.render(&mut Context::default()).await.into_string();

    assert!(html.starts_with("<span"));
    assert!(html.ends_with("</span>"));
}

#[pagetop::test]
async fn default_intent_is_secondary() {
    let mut badge = Badge::labeled(Lc::n("Admin"));
    let html = badge.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"class="badge badge-secondary""#));
}

#[pagetop::test]
async fn each_direct_constructor_sets_its_intent() {
    let cases = [
        (Badge::primary(Lc::n("x")), "badge-primary"),
        (Badge::secondary(Lc::n("x")), "badge-secondary"),
        (Badge::success(Lc::n("x")), "badge-success"),
        (Badge::info(Lc::n("x")), "badge-info"),
        (Badge::warning(Lc::n("x")), "badge-warning"),
        (Badge::danger(Lc::n("x")), "badge-danger"),
    ];
    for (mut badge, expected_class) in cases {
        let html = badge.render(&mut Context::default()).await.into_string();
        assert!(
            html.contains(expected_class),
            "expected `{expected_class}` in `{html}`"
        );
    }
}

#[pagetop::test]
async fn with_intent_overrides_the_default() {
    let mut badge = Badge::labeled(Lc::n("Admin")).with_intent(Intent::Danger);
    let html = badge.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"class="badge badge-danger""#));
}

#[pagetop::test]
async fn direct_constructor_is_equivalent_to_labeled_with_intent() {
    let mut from_constructor = Badge::danger(Lc::n("Admin"));
    let mut from_builder = Badge::labeled(Lc::n("Admin")).with_intent(Intent::Danger);

    assert_eq!(
        from_constructor
            .render(&mut Context::default())
            .await
            .into_string(),
        from_builder
            .render(&mut Context::default())
            .await
            .into_string(),
    );
}

#[pagetop::test]
async fn with_id_sets_the_identifier() {
    let mut badge = Badge::labeled(Lc::n("Admin")).with_id("my-badge");
    let html = badge.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"id="my-badge""#));
}

#[pagetop::test]
async fn with_prop_adds_extra_classes_alongside_the_intent_class() {
    let mut badge = Badge::danger(Lc::n("Admin")).with_prop(PropsOp::add_classes("custom"));
    let html = badge.render(&mut Context::default()).await.into_string();

    assert!(html.contains("badge-danger"));
    assert!(html.contains("custom"));
}
