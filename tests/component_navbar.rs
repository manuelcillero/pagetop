use pagetop::prelude::*;

fn one_link_nav() -> Nav {
    Nav::new().with_item(nav::Item::link(Lc::n("Home"), "/"))
}

// **< Navbar >*************************************************************************************

#[pagetop::test]
async fn is_not_rendered_when_empty() {
    let mut navbar = Navbar::simple();
    let html = navbar.render(&mut Context::default()).await;

    assert!(html.is_empty());
}

// **< Navbar + FlexItem::push_end >****************************************************************

#[pagetop::test]
async fn push_end_adds_an_automatic_start_margin() {
    let mut cx = Context::default();
    let mut nav = one_link_nav().with_prop(FlexItem::push_end());
    let html = nav.render(&mut cx).await.into_string();
    let assets = cx.render_assets().into_string();

    assert!(html.contains("_flex-item-offset_auto_"));
    assert!(assets.contains("_flex-item-offset_auto_{margin-inline-start:auto}"));
}

// **< Navbar::with_expand >************************************************************************

#[pagetop::test]
async fn expand_is_md_by_default_and_can_be_set() {
    assert_eq!(Navbar::simple().expand(), Breakpoint::Md);
    assert_eq!(
        Navbar::simple().with_expand(Breakpoint::Lg).expand(),
        Breakpoint::Lg
    );
}

#[pagetop::test]
async fn expand_class_is_md_by_default() {
    let mut navbar = Navbar::simple_toggle().with_item(navbar::Item::nav(one_link_nav()));
    let html = navbar.render(&mut Context::default()).await.into_string();

    assert!(html.contains("navbar-expand-md"));
}

#[pagetop::test]
async fn expand_class_uses_the_breakpoint_name() {
    let mut navbar = Navbar::simple_toggle()
        .with_expand(Breakpoint::Xl)
        .with_item(navbar::Item::nav(one_link_nav()));
    let html = navbar.render(&mut Context::default()).await.into_string();

    assert!(html.contains("navbar-expand-xl"));
    assert!(!html.contains("navbar-expand-md"));
}

#[pagetop::test]
async fn expand_class_has_no_name_without_a_breakpoint_width() {
    // `Xs` has no minimum width, so the class carries no breakpoint name.
    let mut navbar = Navbar::simple_toggle()
        .with_expand(Breakpoint::Xs)
        .with_item(navbar::Item::nav(one_link_nav()));
    let html = navbar.render(&mut Context::default()).await.into_string();

    assert!(html.contains("navbar-expand"));
    assert!(!html.contains("navbar-expand-"));
}
