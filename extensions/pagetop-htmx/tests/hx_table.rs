use pagetop::prelude::*;
use pagetop_htmx::prelude::*;

// Forces an effective language different from the default negotiated one (en-US, with no `?lang` in
// the request), so that `Context::route()` decides to propagate `?lang=...` in local routes.
fn cx_with_lang(lang: &str) -> Context {
    Context::default().with_langid(&Locale::resolve(lang))
}

async fn render_column(column: table::Column) -> String {
    let mut table = Table::new().with_column(column);
    table.render(&mut Context::default()).await.into_string()
}

// **< sort_link() - htmx attributes >**************************************************************

#[pagetop::test]
async fn sort_link_sets_the_four_fixed_htmx_attributes() {
    let column = table::Column::new(L10n::n("User")).with_sort(hx_table::sort_link(
        "/admin/users",
        "#user-table",
        None,
    ));

    let html = render_column(column).await;

    assert!(html.contains(r#"hx-get="/admin/users""#));
    assert!(html.contains(r##"hx-target="#user-table""##));
    assert!(html.contains(r#"hx-swap="outerHTML scroll:top""#));
    assert!(html.contains(r#"hx-push-url="true""#));
}

#[pagetop::test]
async fn sort_link_href_matches_the_hx_get_value() {
    // The link must work with or without HTMX: `href` is the real destination, and `hx-get` must
    // request that very same URL so both navigation paths land on the same state.
    let column = table::Column::new(L10n::n("User")).with_sort(hx_table::sort_link(
        "/admin/users?sort=username",
        "#user-table",
        None,
    ));

    let html = render_column(column).await;

    assert!(html.contains(r#"href="/admin/users?sort=username""#));
    assert!(html.contains(r#"hx-get="/admin/users?sort=username""#));
}

#[pagetop::test]
async fn sort_link_target_is_configurable_per_table() {
    let column = table::Column::new(L10n::n("Email")).with_sort(hx_table::sort_link(
        "/admin/users",
        "#other-wrapper",
        None,
    ));

    let html = render_column(column).await;

    assert!(html.contains(r##"hx-target="#other-wrapper""##));
}

// **< sort_link() - sort direction propagation >***************************************************

#[pagetop::test]
async fn sort_link_without_active_direction_marks_aria_sort_none() {
    let column = table::Column::new(L10n::n("User")).with_sort(hx_table::sort_link(
        "/admin/users",
        "#user-table",
        None,
    ));

    let html = render_column(column).await;

    assert!(html.contains(r#"aria-sort="none""#));
}

#[pagetop::test]
async fn sort_link_with_active_direction_marks_aria_sort_and_css_class() {
    let column = table::Column::new(L10n::n("User")).with_sort(hx_table::sort_link(
        "/admin/users",
        "#user-table",
        SortDir::Desc,
    ));

    let html = render_column(column).await;

    assert!(html.contains(r#"aria-sort="descending""#));
    assert!(html.contains("table-sort table-sort-desc"));
}

// **< sort_link() - RoutePath / Context::route() integration >*************************************

#[pagetop::test]
async fn sort_link_with_a_bare_literal_href_never_adds_lang() {
    // `sort_link()` does not receive `cx`, so it cannot add `lang` on its own: passing a raw
    // literal must leave both `href` and `hx-get` exactly as given.
    let column = table::Column::new(L10n::n("User")).with_sort(hx_table::sort_link(
        "/admin/users",
        "#user-table",
        None,
    ));

    let html = render_column(column).await;

    assert!(html.contains(r#"href="/admin/users""#));
    assert!(!html.contains("lang="));
}

#[pagetop::test]
async fn sort_link_carries_through_a_lang_aware_href_unchanged() {
    // The caller is expected to resolve `href` with `cx.route(...)` beforehand (see the type's own
    // doc example); `sort_link()` must not re-encode or otherwise alter what it receives.
    let cx = cx_with_lang("es-ES");
    let href = cx.route("/admin/users");

    let column = table::Column::new(L10n::n("User")).with_sort(hx_table::sort_link(
        href,
        "#user-table",
        None,
    ));

    let html = render_column(column).await;

    assert!(html.contains(r#"href="/admin/users?lang=es-ES""#));
    assert!(html.contains(r#"hx-get="/admin/users?lang=es-ES""#));
}

#[pagetop::test]
async fn sort_link_carries_through_extra_query_params_in_order() {
    let cx = cx_with_lang("es-ES");
    let href = cx
        .route("/admin/users")
        .with_param("sort", "username")
        .with_param("dir", "desc");

    let column = table::Column::new(L10n::n("User")).with_sort(hx_table::sort_link(
        href,
        "#user-table",
        SortDir::Desc,
    ));

    let html = render_column(column).await;

    // `&` is escaped to `&amp;` because this ends up inside an HTML attribute value.
    assert!(html.contains(r#"href="/admin/users?lang=es-ES&amp;sort=username&amp;dir=desc""#));
}

#[pagetop::test]
async fn sort_link_with_an_external_href_is_left_untouched() {
    // `Context::route()` never adds `lang` to a URL that looks external; `sort_link()` must not
    // reintroduce it either, since it only forwards whatever `RoutePath` it receives.
    let cx = cx_with_lang("es-ES");
    let href = cx.route("https://example.com/export");

    let column =
        table::Column::new(L10n::n("Export")).with_sort(hx_table::sort_link(href, "#table", None));

    let html = render_column(column).await;

    assert!(html.contains(r#"href="https://example.com/export""#));
    assert!(!html.contains("lang="));
}
