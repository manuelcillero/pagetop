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

#[pagetop::test]
async fn nav_root_class_is_unaffected_by_content_flex() {
    let mut navbar = Navbar::simple()
        .with_flex(Flex::row().with_justify(flex::ContentJustify::End))
        .with_item(navbar::Item::nav(one_link_nav()));
    let html = navbar.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"class="navbar""#));
}

// **< Navbar + Flex (content area) >***************************************************************

#[pagetop::test]
async fn without_flex_content_area_has_no_style_attribute() {
    let mut navbar = Navbar::simple().with_item(navbar::Item::nav(one_link_nav()));
    let html = navbar.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"class="navbar-content""#));
    assert!(!html.contains("style="));
}

#[pagetop::test]
async fn flex_adds_its_styles_to_the_content_area() {
    let mut navbar = Navbar::simple()
        .with_flex(Flex::row().with_justify(flex::ContentJustify::End))
        .with_item(navbar::Item::nav(one_link_nav()));
    let html = navbar.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"class="navbar-content""#));
    assert!(html.contains("display: flex"));
    assert!(html.contains("justify-content: flex-end"));
}

#[pagetop::test]
async fn flex_gap_adds_a_style_to_the_content_area() {
    let mut navbar = Navbar::simple()
        .with_flex(Flex::row().with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))))
        .with_item(navbar::Item::nav(one_link_nav()));
    let html = navbar.render(&mut Context::default()).await.into_string();

    assert!(html.contains("gap: 0.5rem"));
}

// **< Navbar + FlexItem::push_end >****************************************************************

#[pagetop::test]
async fn push_end_adds_an_automatic_start_margin() {
    let mut nav = one_link_nav().with_prop(FlexItem::push_end());
    let html = nav.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"style="margin-inline-start: auto""#));
}
