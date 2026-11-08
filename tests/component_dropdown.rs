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

// **< Separators >*********************************************************************************

// A route that nobody can see without a request: its links are always hidden in these tests.
#[derive(Clone, Copy)]
struct Hidden;

impl Endpoint for Hidden {
    fn path(&self) -> CowStr {
        "/hidden".into()
    }

    fn access(&self) -> Access {
        Access::Authenticated
    }
}

fn shown(label: &'static str) -> dropdown::Item {
    dropdown::Item::link(Lc::n(label), "/shown")
}

fn hidden(label: &'static str) -> dropdown::Item {
    dropdown::Item::link(Lc::n(label), Hidden)
}

fn header(label: &'static str) -> dropdown::Item {
    dropdown::Item::header(Lc::n(label))
}

// Position of `text` in `html`, which must contain it.
fn at(html: &str, text: &str) -> usize {
    html.find(text)
        .unwrap_or_else(|| panic!("`{text}` is missing in {html}"))
}

async fn render_menu(items: Vec<dropdown::Item>) -> String {
    Dropdown::new()
        .with_title(Lc::n("Menu"))
        .with_item(TypedOp::AddMany(items))
        .render(&mut Context::default())
        .await
        .into_string()
}

#[pagetop::test]
async fn a_header_without_visible_items_is_not_shown() {
    let html = render_menu(vec![
        header("group-empty"),
        hidden("item-hidden"),
        header("group-full"),
        shown("item-shown"),
    ])
    .await;

    assert!(!html.contains("group-empty"), "{html}");
    assert!(!html.contains("item-hidden"), "{html}");
    assert!(html.contains("group-full"), "{html}");
    assert!(at(&html, "group-full") < at(&html, "item-shown"), "{html}");
}

#[pagetop::test]
async fn dividers_only_separate_visible_items() {
    let html = render_menu(vec![
        dropdown::Item::divider(),
        shown("first"),
        dropdown::Item::divider(),
        hidden("item-hidden"),
        dropdown::Item::divider(),
        shown("second"),
        dropdown::Item::divider(),
    ])
    .await;

    assert_eq!(html.matches("dropdown-divider").count(), 1, "{html}");
    let divider = at(&html, "dropdown-divider");
    assert!(
        at(&html, "first") < divider && divider < at(&html, "second"),
        "{html}"
    );
}

#[pagetop::test]
async fn a_divider_and_a_header_are_shown_before_the_next_visible_group() {
    let html = render_menu(vec![
        shown("first"),
        dropdown::Item::divider(),
        header("group"),
        shown("second"),
    ])
    .await;

    let (divider, group) = (at(&html, "dropdown-divider"), at(&html, "group"));
    assert!(at(&html, "first") < divider && divider < group, "{html}");
    assert!(group < at(&html, "second"), "{html}");
}

#[pagetop::test]
async fn labels_and_buttons_count_as_visible_items() {
    let html = render_menu(vec![
        header("group-label"),
        dropdown::Item::label(Lc::n("text")),
        header("group-button"),
        dropdown::Item::button(Lc::n("action")),
    ])
    .await;

    assert!(html.contains("group-label"), "{html}");
    assert!(html.contains("group-button"), "{html}");
}

#[pagetop::test]
async fn a_dropdown_with_only_separators_left_is_not_rendered() {
    let html = render_menu(vec![
        header("group"),
        hidden("item-hidden"),
        dropdown::Item::divider(),
    ])
    .await;

    assert!(html.is_empty(), "{html}");
}

#[pagetop::test]
async fn separators_are_also_cleaned_in_a_dropdown_inside_a_nav() {
    let menu = Dropdown::new()
        .with_title(Lc::n("menu-empty"))
        .with_item(header("group-empty"))
        .with_item(hidden("item-hidden"));
    let partial = Dropdown::new()
        .with_title(Lc::n("menu-partial"))
        .with_item(header("group-empty"))
        .with_item(hidden("item-hidden"))
        .with_item(dropdown::Item::divider())
        .with_item(shown("item-shown"));
    let mut nav = Nav::new()
        .with_item(nav::Item::dropdown(menu))
        .with_item(nav::Item::dropdown(partial));
    let html = nav.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("menu-empty"), "{html}");
    assert!(html.contains("menu-partial"), "{html}");
    assert!(!html.contains("group-empty"), "{html}");
    assert!(!html.contains("dropdown-divider"), "{html}");
    assert!(html.contains("item-shown"), "{html}");
}
