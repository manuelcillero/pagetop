// Verifies that the breakpoint-based Bootstrap classes of `Container`, `Navbar`, `Dropdown` and
// `Offcanvas` use the breakpoint names of `Bootsier::breakpoint_entry()`, and that `Xxxl` (absent
// in Bootstrap) resolves as `xxl`.

use pagetop::prelude::*;
use pagetop_bootsier::theme::*;

fn cx() -> Context {
    Context::default().with_theme(&pagetop_bootsier::Bootsier)
}

async fn container_html(width: impl Into<Option<bs::container::Width>>) -> String {
    let mut container = bs::Container::new()
        .with_width(width)
        .with_child(Html::with(|_| html! { p { "Content" } }));
    container.render(&mut cx()).await.into_string()
}

#[pagetop::test]
async fn container_without_width_gets_the_bootstrap_default() {
    assert!(container_html(None).await.contains(r#"class="container""#));
    assert!(
        container_html(bs::container::Width::Responsive)
            .await
            .contains(r#"class="container""#)
    );
}

#[pagetop::test]
async fn container_width_uses_breakpoint_name() {
    assert!(
        container_html(bs::container::Width::Fluid)
            .await
            .contains(r#"class="container-fluid""#)
    );
    assert!(
        container_html(bs::container::Width::From(Breakpoint::Lg))
            .await
            .contains(r#"class="container-lg""#)
    );
}

#[pagetop::test]
async fn container_width_xxxl_resolves_as_xxl() {
    assert!(
        container_html(bs::container::Width::From(Breakpoint::Xxxl))
            .await
            .contains(r#"class="container-xxl""#)
    );
}

#[pagetop::test]
async fn dropdown_menu_align_combines_breakpoint_classes() {
    let cx = cx();
    assert_eq!(bs::dropdown::MenuAlign::Start.to_class(&cx), "");
    assert_eq!(
        bs::dropdown::MenuAlign::EndAt(Breakpoint::Md).to_class(&cx),
        "dropdown-menu-md-end"
    );
    assert_eq!(
        bs::dropdown::MenuAlign::StartAndEnd(Breakpoint::Xl).to_class(&cx),
        "dropdown-menu-start dropdown-menu-xl-end"
    );
}

async fn dropdown_menu_html(dropdown: bs::Dropdown) -> String {
    let mut dropdown = dropdown
        .with_title(Lc::n("Menu"))
        .with_item(bs::dropdown::Item::link(Lc::n("Home"), "/"));
    dropdown.render(&mut cx()).await.into_string()
}

#[pagetop::test]
async fn dropdown_menu_end_maps_to_the_end_alignment() {
    let html = dropdown_menu_html(bs::Dropdown::new().with_menu_end(true)).await;
    assert!(html.contains(r#"<ul class="dropdown-menu dropdown-menu-end">"#));

    // Without `with_menu_end()` the menu keeps the default (start) alignment.
    let html = dropdown_menu_html(bs::Dropdown::new()).await;
    assert!(html.contains(r#"<ul class="dropdown-menu">"#));
    assert!(!html.contains("dropdown-menu-end"));
}

#[pagetop::test]
async fn dropdown_menu_end_is_ignored_without_title() {
    // A titleless menu is static (no button to align to), so it never gets the end alignment.
    let mut dropdown = bs::Dropdown::new()
        .with_menu_end(true)
        .with_item(bs::dropdown::Item::link(Lc::n("Home"), "/"));
    let html = dropdown.render(&mut cx()).await.into_string();
    assert!(html.contains(r#"<ul class="dropdown-menu">"#));
    assert!(!html.contains("dropdown-menu-end"));
}

#[pagetop::test]
async fn dropdown_own_menu_align_wins_over_menu_end() {
    let html = dropdown_menu_html(
        bs::Dropdown::new()
            .with_menu_end(true)
            .with_menu_align(bs::dropdown::MenuAlign::StartAt(Breakpoint::Md)),
    )
    .await;
    assert!(html.contains("dropdown-menu-md-start"));
    assert!(!html.contains("dropdown-menu-end"));
}

#[pagetop::test]
async fn nav_dropdown_menu_end_adds_the_end_class() {
    let mut nav = bs::Nav::new().with_item(bs::nav::Item::dropdown(
        bs::Dropdown::new()
            .with_title(Lc::n("Menu"))
            .with_menu_end(true)
            .with_item(bs::dropdown::Item::link(Lc::n("Home"), "/")),
    ));
    let html = nav.render(&mut cx()).await.into_string();
    assert!(html.contains(r#"<ul class="dropdown-menu dropdown-menu-end">"#));
}

#[pagetop::test]
async fn navbar_expands_at_md_by_default_and_at_the_chosen_breakpoint() {
    let item =
        || bs::navbar::Item::nav(bs::Nav::new().with_item(bs::nav::Item::link(Lc::n("Home"), "/")));

    let mut navbar = bs::Navbar::simple_toggle().with_item(item());
    let html = navbar.render(&mut cx()).await.into_string();
    assert!(html.contains("navbar-expand-md"));
    // The expansion class comes from the `Navbar` component only, so it is not duplicated.
    assert_eq!(html.matches("navbar-expand").count(), 1);

    let mut navbar = bs::Navbar::simple_toggle()
        .with_expand(Breakpoint::Xxxl)
        .with_item(item());
    let html = navbar.render(&mut cx()).await.into_string();
    assert!(html.contains("navbar-expand-xxl"));

    // Bootstrap needs a plain `navbar-expand` to keep the bar always expanded.
    let mut navbar = bs::Navbar::simple_toggle()
        .with_expand(Breakpoint::Xs)
        .with_item(item());
    let html = navbar.render(&mut cx()).await.into_string();
    assert!(html.contains("navbar-expand"));
    assert!(!html.contains("navbar-expand-"));
}

#[pagetop::test]
async fn navbar_without_toggle_ignores_the_breakpoint() {
    let item =
        || bs::navbar::Item::nav(bs::Nav::new().with_item(bs::nav::Item::link(Lc::n("Home"), "/")));
    let brand = || Brand::new().with_title(Lc::n("PageTop"));

    // With nothing to collapse, Bootstrap would still stack the menu below the breakpoint.
    let mut navbar = bs::Navbar::simple()
        .with_expand(Breakpoint::Lg)
        .with_item(item());
    let html = navbar.render(&mut cx()).await.into_string();
    assert!(html.contains("navbar-expand"));
    assert!(!html.contains("navbar-expand-"));

    let mut navbar = bs::Navbar::simple_brand_left(brand())
        .with_expand(Breakpoint::Lg)
        .with_item(item());
    let html = navbar.render(&mut cx()).await.into_string();
    assert!(html.contains("navbar-expand"));
    assert!(!html.contains("navbar-expand-"));
    // The menu stays next to the brand instead of being pushed to the opposite end.
    assert!(html.contains(r#"<div class="navbar-collapse">"#));

    // The layouts with a toggle keep their breakpoint.
    let mut navbar = bs::Navbar::brand_left(brand())
        .with_expand(Breakpoint::Lg)
        .with_item(item());
    let html = navbar.render(&mut cx()).await.into_string();
    assert!(html.contains("navbar-expand-lg"));
}

#[pagetop::test]
async fn offcanvas_uses_either_the_plain_or_the_breakpoint_class() {
    // Bootstrap's `.offcanvas` is always fixed, so it must not accompany `.offcanvas-{bp}`.
    let panel =
        |offcanvas: bs::Offcanvas| offcanvas.with_child(Html::with(|_| html! { p { "Content" } }));

    let mut plain = panel(bs::Offcanvas::new());
    let html = plain.render(&mut cx()).await.into_string();
    assert!(html.contains(r#"class="offcanvas "#));
    assert!(!html.contains("offcanvas-lg"));

    let mut responsive = panel(bs::Offcanvas::new().with_breakpoint(Breakpoint::Lg));
    let html = responsive.render(&mut cx()).await.into_string();
    assert!(html.contains(r#"class="offcanvas-lg "#));
    assert!(!html.contains(r#"class="offcanvas "#));
}
