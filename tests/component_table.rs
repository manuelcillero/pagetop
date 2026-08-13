use pagetop::prelude::*;

#[pagetop::test]
async fn table_without_columns_or_rows_renders_empty_shell() {
    let mut table = Table::new().with_empty(None);

    let html = table.render(&mut Context::default()).await.into_string();

    assert_eq!(
        html,
        r#"<div class="table-responsive"><table class="table"></table></div>"#
    );
}

#[pagetop::test]
async fn table_without_rows_uses_default_empty_message() {
    let mut table = Table::new();

    let html = table.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"<td class="table-empty" colspan="1">No data to display</td>"#));
}

#[pagetop::test]
async fn table_with_columns_and_no_rows_shows_empty_message() {
    let mut table = Table::new()
        .with_column(Lc::n("User"))
        .with_column(Lc::n("Email"))
        .with_empty(Lc::n("No users to show."));

    let html = table.render(&mut Context::default()).await.into_string();

    assert!(html.contains("<thead>"));
    assert!(html.contains("<th scope=\"col\">User</th>"));
    assert!(html.contains("<th scope=\"col\">Email</th>"));
    assert!(html.contains(r#"<td class="table-empty" colspan="2">No users to show.</td>"#));
}

#[pagetop::test]
async fn table_with_columns_and_no_rows_and_no_empty_message_omits_tbody() {
    let mut table = Table::new().with_column(Lc::n("User")).with_empty(None);

    let html = table.render(&mut Context::default()).await.into_string();

    assert!(html.contains("<thead>"));
    assert!(!html.contains("<tbody>"));
}

#[pagetop::test]
async fn table_renders_rows_and_cells_in_order() {
    let mut table = Table::new()
        .with_column(Lc::n("User"))
        .with_column(Lc::n("Email"));

    table.alter_row(
        table::Row::new()
            .with_cell(table::Cell::new(Html::with(|_| html! { "alice" })))
            .with_cell(table::Cell::new(Html::with(
                |_| html! { "alice@example.com" },
            ))),
    );
    table.alter_row(
        table::Row::new()
            .with_cell(table::Cell::new(Html::with(|_| html! { "bob" })))
            .with_cell(table::Cell::new(Html::with(
                |_| html! { "bob@example.com" },
            ))),
    );

    let html = table.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("table-empty"));
    let alice = html
        .find("alice@example.com")
        .expect("Expected alice's row");
    let bob = html.find("bob@example.com").expect("Expected bob's row");
    assert!(alice < bob, "Rows should keep insertion order");
}

#[pagetop::test]
async fn sortable_column_without_active_direction_marks_aria_sort_none() {
    let mut table = Table::new().with_column(
        table::Column::new(Lc::n("User")).with_sort(table::SortLink::new("/users?sort=username")),
    );

    let html = table.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"aria-sort="none""#));
    assert!(html.contains(r#"<a href="/users?sort=username" class="table-sort">User</a>"#));
    assert!(!html.contains("table-sort-asc"));
    assert!(!html.contains("table-sort-desc"));
}

#[pagetop::test]
async fn sortable_column_with_active_direction_marks_aria_sort_and_css_class() {
    let mut table = Table::new().with_column(
        table::Column::new(Lc::n("User"))
            .with_sort(table::SortLink::new("/users?sort=username").with_dir(SortDir::Desc)),
    );

    let html = table.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"aria-sort="descending""#));
    assert!(html.contains("table-sort table-sort-desc"));
}

#[pagetop::test]
async fn sort_link_props_are_ready_for_htmx_without_hardcoding_it() {
    let mut table = Table::new().with_column(
        table::Column::new(Lc::n("User")).with_sort(
            table::SortLink::new("/users?sort=username")
                .with_prop(PropsOp::set("hx-get", "/users?sort=username"))
                .with_prop(PropsOp::set("hx-target", "#user-table")),
        ),
    );

    let html = table.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r##"hx-get="/users?sort=username""##));
    assert!(html.contains(r##"hx-target="#user-table""##));
}

#[pagetop::test]
async fn row_and_cell_props_flow_through_to_attributes() {
    let mut table = Table::new().with_column(Lc::n("User"));

    table.alter_row(
        table::Row::new()
            .with_prop(PropsOp::set("hx-target", "#row-1"))
            .with_cell(
                table::Cell::new(Html::with(|_| html! { "alice" }))
                    .with_prop(PropsOp::add_classes("is-admin")),
            ),
    );

    let html = table.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r##"<tr hx-target="#row-1">"##));
    assert!(html.contains(r##"<td class="is-admin">alice</td>"##));
}
