use pagetop::prelude::*;
use pagetop_htmx::prelude::*;

// Forces an effective language different from the default negotiated one (en-US, with no `?lang` in
// the request), so that `Context::route()` decides to propagate `?lang=...` in local routes.
fn cx_with_lang(lang: &str) -> Context {
    Context::default().with_langid(&Locale::resolve(lang))
}

// **< hx_pager::boost() >**************************************************************************

#[pagetop::test]
async fn boost_targets_the_list_from_the_pager_nav() {
    let mut pager = hx_pager::boost(
        Pager::new()
            .with_base_path("/items")
            .with_current_page(1)
            .with_items_per_page(10)
            .with_total_items(50),
        "#item-list",
    );
    let html = pager.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"hx-boost="true""#), "{html}");
    assert!(html.contains(r##"hx-target="#item-list""##), "{html}");
    assert!(html.contains(r#"hx-swap="outerHTML scroll:top""#), "{html}");
    assert!(html.contains(r#"hx-push-url="true""#), "{html}");
}

// **< hx_filter::boost() >*************************************************************************

#[pagetop::test]
async fn boost_sends_the_filter_form_to_the_list() {
    let mut filter = hx_filter::boost(
        ListFilter::new()
            .with_filter_action("/items")
            .with_field(form::input::Field::search().with_name("q")),
        "#item-list",
    );
    let html = filter.render(&mut Context::default()).await.into_string();

    // `hx-boost` on the form sends its fields to its own `action`, so no `hx-get` is needed.
    assert!(html.contains(r#"hx-boost="true""#), "{html}");
    assert!(!html.contains("hx-get"), "{html}");
    // When it applies is up to the filter (button or auto submit), not to HTMX.
    assert!(!html.contains("hx-trigger"), "{html}");
    assert!(html.contains(r##"hx-target="#item-list""##), "{html}");
    assert!(html.contains(r#"hx-push-url="true""#), "{html}");
}

#[pagetop::test]
async fn boosted_filter_keeps_the_language_in_a_hidden_field() {
    // A boosted GET form drops the query of its `action`, the same as the browser does.
    let mut filter = hx_filter::boost(
        ListFilter::new()
            .with_filter_action("/items")
            .with_field(form::input::Field::search().with_name("q")),
        "#item-list",
    );
    let html = filter
        .render(&mut cx_with_lang("es-ES"))
        .await
        .into_string();

    assert!(html.contains(r#"name="lang""#), "{html}");
    assert!(html.contains(r#"value="es-ES""#), "{html}");
}

// **< Overlapping requests and loading indicator >*************************************************

#[pagetop::test]
async fn list_helpers_cancel_overlapping_requests_and_mark_the_list_while_loading() {
    let mut pager = hx_pager::boost(
        Pager::new()
            .with_base_path("/items")
            .with_current_page(1)
            .with_items_per_page(10)
            .with_total_items(50),
        "#item-list",
    );
    let mut filter = hx_filter::boost(ListFilter::new().with_filter_action("/items"), "#item-list");
    let mut table = Table::new().with_column(
        table::Column::new(Lc::n("Name")).with_sort(hx_table::sort_link(
            "/items",
            "#item-list",
            None,
        )),
    );

    let mut cx = Context::default();
    for html in [
        pager.render(&mut cx).await.into_string(),
        filter.render(&mut cx).await.into_string(),
        table.render(&mut cx).await.into_string(),
    ] {
        // Every request against the list cancels the one in progress, wherever it comes from.
        assert!(html.contains(r##"hx-sync="#item-list:replace""##), "{html}");
        assert!(html.contains(r##"hx-indicator="#item-list""##), "{html}");
    }
}
