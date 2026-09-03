use pagetop::prelude::*;

// **< Button >*************************************************************************************

#[pagetop::test]
async fn label_is_rendered_when_set() {
    let mut button = Button::submit(Lc::n("Save"));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("Save"));
}

#[pagetop::test]
async fn label_can_be_cleared_with_lc_none() {
    let mut button = Button::submit(Lc::n("Save")).with_label(Lc::none());
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("Save"));
    // The button itself must still render.
    assert!(html.contains("<button"));
}

#[pagetop::test]
async fn title_attribute_is_absent_by_default() {
    let mut button = Button::submit(Lc::n("Save"));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("title="));
}

#[pagetop::test]
async fn title_attribute_is_present_when_set() {
    let mut button = Button::submit(Lc::n("Save")).with_title(Lc::n("Save changes"));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"title="Save changes""#));
}

#[pagetop::test]
async fn title_attribute_can_be_cleared_with_lc_none() {
    let mut button = Button::submit(Lc::n("Save"))
        .with_title(Lc::n("Save changes"))
        .with_title(Lc::none());
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("title="));
}

#[pagetop::test]
async fn style_class_reflects_intent_and_style() {
    let mut button = Button::submit(Lc::n("Save")).with_style(button::Style::Solid(Intent::Severe));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("button-severe"));
}

#[pagetop::test]
async fn outline_style_generates_outline_class() {
    let mut button =
        Button::submit(Lc::n("Save")).with_style(button::Style::Outline(Intent::Primary));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("button-outline-primary"));
}

#[pagetop::test]
async fn link_style_generates_link_class_without_intent() {
    let mut button = Button::plain(Lc::n("Cancel")).with_style(button::Style::Link);
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("button-link"));
}

#[pagetop::test]
async fn size_small_generates_button_sm_class() {
    let mut button = Button::submit(Lc::n("Save")).with_size(button::Size::Small);
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("button-sm"));
}

#[pagetop::test]
async fn size_large_generates_button_lg_class() {
    let mut button = Button::submit(Lc::n("Save")).with_size(button::Size::Large);
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("button-lg"));
}

#[pagetop::test]
async fn size_none_omits_size_class_by_default() {
    let mut button = Button::submit(Lc::n("Save"));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("button-sm"));
    assert!(!html.contains("button-lg"));
}

#[pagetop::test]
async fn anchor_renders_as_a_tag_with_href() {
    let mut button = Button::anchor(Lc::n("Edit"), "/items/1/edit");
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.starts_with("<a "));
    assert!(html.contains(r#"href="/items/1/edit""#));
    assert!(html.contains("Edit"));
}

#[pagetop::test]
async fn button_without_href_renders_as_button_tag() {
    let mut button = Button::submit(Lc::n("Save"));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.starts_with("<button"));
}

#[pagetop::test]
async fn anchor_falls_back_to_button_when_href_is_reset() {
    // `Route::default()` resuelve a una ruta vacía; `prepare()` debe ignorar `href` y volver a
    // renderizar un `<button>`, tal y como documenta `Button::with_href()`.
    let mut button = Button::anchor(Lc::n("Edit"), "/items/1/edit").with_href(Route::default());
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.starts_with("<button"));
}

#[pagetop::test]
async fn enabled_anchor_has_no_aria_disabled_or_tabindex() {
    let mut button = Button::anchor(Lc::n("Edit"), "/items/1/edit");
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("aria-disabled"));
    assert!(!html.contains("tabindex"));
}

#[pagetop::test]
async fn disabled_anchor_omits_href_and_sets_aria_disabled() {
    let mut button = Button::anchor(Lc::n("Edit"), "/items/1/edit").with_disabled(true);
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("href="));
    assert!(html.contains(r#"aria-disabled="true""#));
    assert!(html.contains(r#"tabindex="-1""#));
}
