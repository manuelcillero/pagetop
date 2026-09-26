use pagetop::prelude::*;

fn dropdown() -> Dropdown {
    Dropdown::new()
        .with_title(Lc::n("Menu"))
        .with_item(dropdown::Item::link(Lc::n("Home"), "/"))
}

#[pagetop::test]
async fn menu_is_aligned_to_the_start_by_default() {
    let html = dropdown()
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(html.contains(r#"<ul class="dropdown-menu">"#));
    assert!(!html.contains("dropdown-menu-end"));
}

#[pagetop::test]
async fn menu_end_adds_the_end_class() {
    let html = dropdown()
        .with_menu_end(true)
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(html.contains(r#"<ul class="dropdown-menu dropdown-menu-end">"#));
}

#[pagetop::test]
async fn menu_end_applies_to_a_dropdown_inside_a_nav() {
    let mut nav = Nav::new().with_item(nav::Item::dropdown(dropdown().with_menu_end(true)));
    let html = nav.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"<ul class="dropdown-menu dropdown-menu-end">"#));
}

#[pagetop::test]
async fn menu_end_is_ignored_without_title() {
    let html = Dropdown::new()
        .with_menu_end(true)
        .with_item(dropdown::Item::link(Lc::n("Home"), "/"))
        .render(&mut Context::default())
        .await
        .into_string();

    assert!(html.contains(r#"<ul class="dropdown-menu">"#));
    assert!(!html.contains("dropdown-menu-end"));
}
