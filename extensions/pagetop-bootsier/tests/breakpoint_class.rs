// Verifies that Bootsier builds its breakpoint-based Bootstrap classes from the breakpoint names
// of `Bootsier::breakpoint_entry()`, and that `Xxxl` (absent in Bootstrap) resolves as `xxl`.

use pagetop::prelude::*;
use pagetop_bootsier::theme::*;

fn cx() -> Context {
    Context::default().with_theme(&pagetop_bootsier::Bootsier)
}

#[pagetop::test]
async fn container_width_uses_breakpoint_name() {
    let cx = cx();
    assert_eq!(bs::container::Width::Default.to_class(&cx), "container");
    assert_eq!(bs::container::Width::Fluid.to_class(&cx), "container-fluid");
    assert_eq!(
        bs::container::Width::From(Breakpoint::Lg).to_class(&cx),
        "container-lg"
    );
}

#[pagetop::test]
async fn container_width_xxxl_resolves_as_xxl() {
    let cx = cx();
    assert_eq!(
        bs::container::Width::From(Breakpoint::Xxxl).to_class(&cx),
        "container-xxl"
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

#[pagetop::test]
async fn navbar_expands_at_md_by_default_and_at_the_chosen_breakpoint() {
    let item =
        || bs::navbar::Item::nav(bs::Nav::new().with_item(bs::nav::Item::link(Lc::n("Home"), "/")));

    let mut navbar = bs::Navbar::simple().with_item(item());
    let html = navbar.render(&mut cx()).await.into_string();
    assert!(html.contains("navbar-expand-md"));
    // The expansion class comes from the `Navbar` component only, so it is not duplicated.
    assert_eq!(html.matches("navbar-expand").count(), 1);

    let mut navbar = bs::Navbar::simple()
        .with_expand(Breakpoint::Xxxl)
        .with_item(item());
    let html = navbar.render(&mut cx()).await.into_string();
    assert!(html.contains("navbar-expand-xxl"));

    // Bootstrap needs a plain `navbar-expand` to keep the bar always expanded.
    let mut navbar = bs::Navbar::simple()
        .with_expand(Breakpoint::Xs)
        .with_item(item());
    let html = navbar.render(&mut cx()).await.into_string();
    assert!(html.contains("navbar-expand"));
    assert!(!html.contains("navbar-expand-"));
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
