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
