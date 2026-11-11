use pagetop::prelude::*;

// **< ListFilter >*********************************************************************************

#[pagetop::test]
async fn renders_a_get_form_with_its_fields_and_the_button() {
    let mut filter = ListFilter::new()
        .with_filter_action("/items")
        .with_field(form::input::Field::search().with_name("q"));
    let html = filter.render(&mut Context::default()).await.into_string();

    assert!(html.starts_with("<form"), "{html}");
    assert!(html.contains(r#"class="list-filter""#), "{html}");
    assert!(html.contains(r#"action="/items""#), "{html}");
    assert!(
        !html.contains("method="),
        "a GET form needs no method: {html}"
    );
    assert!(html.contains(r#"name="q""#), "{html}");
    assert!(html.contains(r#"type="submit""#), "{html}");
}

#[pagetop::test]
async fn keeps_the_language_of_the_action_in_a_hidden_field() {
    // A GET form drops the query of its action, so the language must travel as a field.
    let mut filter =
        ListFilter::new().with_filter_action(RoutePath::new("/items").with_param("lang", "es-ES"));
    let html = filter.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"type="hidden""#), "{html}");
    assert!(html.contains(r#"name="lang""#), "{html}");
    assert!(html.contains(r#"value="es-ES""#), "{html}");
}

#[pagetop::test]
async fn the_button_label_can_be_changed() {
    let mut filter = ListFilter::new()
        .with_filter_action("/items")
        .with_submit(Lc::n("Search"));
    let html = filter.render(&mut Context::default()).await.into_string();

    assert!(html.contains("Search"), "{html}");
}

#[pagetop::test]
async fn by_default_it_applies_with_the_button() {
    let mut filter = ListFilter::new().with_filter_action("/items");
    let html = filter.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("onchange"), "{html}");
    assert!(!html.contains("<noscript>"), "{html}");
}

#[pagetop::test]
async fn auto_submit_applies_on_change_and_keeps_the_button_only_without_javascript() {
    let mut filter = ListFilter::new()
        .with_filter_action("/items")
        .with_auto_submit(true);
    let html = filter.render(&mut Context::default()).await.into_string();

    assert!(
        html.contains(r#"onchange="this.requestSubmit()""#),
        "{html}"
    );
    assert!(
        html.contains(r#"<noscript><button type="submit""#),
        "{html}"
    );
}

#[pagetop::test]
async fn the_reset_link_goes_back_to_the_list_with_no_filters() {
    let mut filter = ListFilter::new().with_filter_action("/items");
    let html = filter.render(&mut Context::default()).await.into_string();

    let link = html
        .find("<a")
        .map(|start| &html[start..])
        .expect("reset link");
    // `reset` also forgets the state a list may remember with `web::remember_query()`.
    assert!(link.contains(r#"href="/items?reset""#), "{html}");
    // Not intercepted by libraries that load only part of the page, so the fields are emptied too.
    assert!(link.contains(r#"target="_top""#), "{html}");
}

#[pagetop::test]
async fn the_reset_link_can_be_removed() {
    let mut filter = ListFilter::new()
        .with_filter_action("/items")
        .with_reset(false);
    let html = filter.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("<a"), "{html}");
}

#[pagetop::test]
async fn with_the_button_it_marks_pending_changes() {
    let mut cx = Context::default();
    let mut filter = ListFilter::new().with_filter_action("/items");
    let html = filter.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    // The script marks changed fields; the hidden status announces them to screen readers.
    assert!(
        assets.contains("/pagetop/js/list-filter.min.js"),
        "{assets}"
    );
    assert!(
        html.contains(r#"class="list-filter-status visually-hidden""#),
        "{html}"
    );
    assert!(html.contains(r#"role="status""#), "{html}");
    assert!(html.contains("data-pending="), "{html}");
}

#[pagetop::test]
async fn auto_submit_has_nothing_pending_to_mark() {
    let mut cx = Context::default();
    let mut filter = ListFilter::new()
        .with_filter_action("/items")
        .with_auto_submit(true);
    let html = filter.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(!assets.contains("list-filter.min.js"), "{assets}");
    assert!(!html.contains("list-filter-status"), "{html}");
}
