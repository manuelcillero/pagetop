use pagetop::prelude::*;
use pagetop_bootsier::theme::*;

#[pagetop::test]
async fn bi_creates_a_bootstrap_icons_glyph() {
    let mut icon = bs::Icon::bi("house-fill");
    let html = icon.render(&mut Context::default()).await.into_string();

    assert!(html.starts_with("<i"));
    assert!(html.contains(r#"class="icon bi bi-house-fill""#));
}

#[pagetop::test]
async fn bootsier_nav_item_renders_its_icons() {
    let mut cx = Context::default().with_theme(&pagetop_bootsier::Bootsier);
    let mut item = bs::nav::Item::link(Lc::n("Home"), "/")
        .with_icon_start(bs::Icon::bi("house"))
        .with_icon_end(bs::Icon::bi("box-arrow-up-right"));
    let html = item.render(&mut cx).await.into_string();

    let start = html.find("bi-house").unwrap();
    let label = html.find("Home").unwrap();
    let end = html.find("bi-box-arrow-up-right").unwrap();
    assert!(start < label && label < end);
}

#[pagetop::test]
async fn bootsier_nav_dropdown_title_renders_its_icons() {
    let mut cx = Context::default().with_theme(&pagetop_bootsier::Bootsier);
    let mut item = bs::nav::Item::dropdown(
        bs::Dropdown::new()
            .with_title(Lc::n("More"))
            .with_item(bs::dropdown::Item::link(Lc::n("One"), "/one")),
    )
    .with_icon_start(bs::Icon::bi("list"))
    .with_icon_end(bs::Icon::bi("star"));
    let html = item.render(&mut cx).await.into_string();

    let start = html.find("bi-list").unwrap();
    let title = html.find("More").unwrap();
    let end = html.find("bi-star").unwrap();
    assert!(start < title && title < end, "{html}");
}

#[pagetop::test]
async fn core_icons_are_shown_with_their_bootstrap_icons_glyph() {
    let mut cx = Context::default().with_theme(&pagetop_bootsier::Bootsier);
    let mut icon = Icon::with(icon::Core::Gear).with_intent(Intent::Primary);
    let html = icon.render(&mut cx).await.into_string();

    assert!(html.starts_with("<i"));
    assert!(html.contains("bi bi-gear"));
    assert!(html.contains("icon-primary"));
    assert!(!html.contains("<svg"));
}

#[pagetop::test]
async fn core_icons_keep_the_icon_class_and_their_label_as_glyphs() {
    let mut cx = Context::default().with_theme(&pagetop_bootsier::Bootsier);
    let mut icon = Icon::with(icon::Core::Close).with_label(Lc::n("Close"));
    let html = icon.render(&mut cx).await.into_string();

    // The menu styles find the icons by the `icon` class.
    assert!(html.contains(r#"class="icon bi bi-x-lg""#), "{html}");
    assert!(html.contains(r#"role="img""#), "{html}");
    assert!(html.contains(r#"aria-label="Close""#), "{html}");
}
