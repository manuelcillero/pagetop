use pagetop::prelude::*;

// **< ListView >***********************************************************************************

fn pager() -> Pager {
    Pager::new()
        .with_base_path("/items")
        .with_current_page(1)
        .with_items_per_page(10)
        .with_total_items(50)
}

fn table() -> Table {
    Table::new()
        .with_column(Lc::n("Name"))
        .with_empty(Lc::n("No items"))
}

#[pagetop::test]
async fn renders_its_id_and_class() {
    let mut list = ListView::new().with_id("item-list").with_table(table());
    let html = list.render(&mut Context::default()).await.into_string();

    assert!(html.starts_with("<div"), "{html}");
    assert!(html.contains(r#"id="item-list""#), "{html}");
    assert!(html.contains(r#"class="list-view""#), "{html}");
}

#[pagetop::test]
async fn renders_the_parts_in_a_fixed_order() {
    // Added in reverse order on purpose: the output order must not depend on it.
    let mut list = ListView::new()
        .with_pager(pager())
        .with_table(table())
        .with_action(Button::anchor(Lc::n("New item"), "/items/new"));
    let mut cx = Context::default();
    cx.push_message(MessageLevel::Error, Lc::n("Something failed"));
    let html = list.render(&mut cx).await.into_string();

    let actions = html.find("list-view-actions").expect("actions");
    let message = html.find("Something failed").expect("message");
    let table = html.find("<table").expect("table");
    let pager = html.find("<nav").expect("pager");
    assert!(actions < message && message < table && table < pager);
}

#[pagetop::test]
async fn without_actions_there_is_no_actions_row() {
    let mut list = ListView::new().with_table(table());
    let html = list.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("list-view-actions"));
    assert!(html.contains("No items"));
}

#[pagetop::test]
async fn it_carries_a_summary_of_the_results_and_its_script() {
    let mut cx = Context::default();
    let mut list = ListView::new().with_table(table()).with_pager(pager());
    let html = list.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    // The total of the pager, not only the rows on this page.
    assert!(html.contains(r#"data-summary="50 results""#), "{html}");
    assert!(assets.contains("/pagetop/js/list-view.min.js"), "{assets}");
}

#[pagetop::test]
async fn without_a_pager_the_summary_counts_the_rows() {
    let mut list = ListView::new().with_table(
        table()
            .with_row(table::Row::new().with_cell(Lc::n("A")))
            .with_row(table::Row::new().with_cell(Lc::n("B"))),
    );
    let html = list.render(&mut Context::default()).await.into_string();
    assert!(html.contains(r#"data-summary="2 results""#), "{html}");

    let mut one =
        ListView::new().with_table(table().with_row(table::Row::new().with_cell(Lc::n("A"))));
    let html = one.render(&mut Context::default()).await.into_string();
    assert!(html.contains(r#"data-summary="1 result""#), "{html}");

    let mut none = ListView::new().with_table(table());
    let html = none.render(&mut Context::default()).await.into_string();
    assert!(html.contains(r#"data-summary="No results""#), "{html}");
}

// **< ListView - state kept by the filter >********************************************************

#[pagetop::test]
async fn filter_params_are_hidden_fields_of_the_filter_form() {
    let mut list = ListView::new()
        .with_table(table())
        .with_filter_id("item-filter")
        .with_filter_param("sort", "name")
        .with_filter_param("dir", "desc")
        .with_filter_param("empty", "");
    let html = list.render(&mut Context::default()).await.into_string();

    // Tied to the filter with `form`, so the browser submits them with it, from outside the form.
    assert!(
        html.contains(r#"<input type="hidden" form="item-filter" name="sort" value="name">"#),
        "{html}"
    );
    assert!(
        html.contains(r#"<input type="hidden" form="item-filter" name="dir" value="desc">"#),
        "{html}"
    );
    assert!(!html.contains(r#"name="empty""#), "{html}");
}

#[pagetop::test]
async fn without_filter_params_there_are_no_hidden_fields() {
    let mut list = ListView::new()
        .with_table(table())
        .with_filter_id("item-filter");
    let html = list.render(&mut Context::default()).await.into_string();

    assert!(!html.contains(r#"type="hidden""#), "{html}");
}

#[pagetop::test]
#[should_panic(expected = "ListView has filter params but no filter id")]
async fn filter_params_without_a_filter_id_are_a_misuse() {
    let mut list = ListView::new()
        .with_table(table())
        .with_filter_param("sort", "name");
    list.render(&mut Context::default()).await;
}
