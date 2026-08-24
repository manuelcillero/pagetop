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
    let mut button =
        Button::submit(Lc::n("Save")).with_style(button::ButtonStyle::Solid(Intent::Danger));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("button-danger"));
}

#[pagetop::test]
async fn outline_style_generates_outline_class() {
    let mut button =
        Button::submit(Lc::n("Save")).with_style(button::ButtonStyle::Outline(Intent::Primary));
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("button-outline-primary"));
}

#[pagetop::test]
async fn link_style_generates_link_class_without_intent() {
    let mut button = Button::plain(Lc::n("Cancel")).with_style(button::ButtonStyle::Link);
    let html = button.render(&mut Context::default()).await.into_string();

    assert!(html.contains("button-link"));
}

// **< ButtonSet >**********************************************************************************

#[pagetop::test]
async fn button_set_is_not_rendered_when_empty() {
    let mut set = button::ButtonSet::new();
    let html = set.render(&mut Context::default()).await;

    assert!(html.is_empty());
}

#[pagetop::test]
async fn button_set_wraps_buttons_in_button_set_class() {
    let mut set = button::ButtonSet::new().with_button(Button::submit(Lc::n("Save")));
    let html = set.render(&mut Context::default()).await.into_string();

    assert!(html.contains("button-set"));
    assert!(html.contains("Save"));
}

#[pagetop::test]
async fn button_set_renders_buttons_in_insertion_order() {
    let mut set = button::ButtonSet::new()
        .with_button(Button::submit(Lc::n("First")))
        .with_button(Button::plain(Lc::n("Second")));
    let html = set.render(&mut Context::default()).await.into_string();

    assert!(html.find("First").unwrap() < html.find("Second").unwrap());
}

#[pagetop::test]
async fn button_set_add_many_appends_all_buttons() {
    let mut set = button::ButtonSet::new().with_button(TypedOp::AddMany(vec![
        Button::submit(Lc::n("Save")),
        Button::reset(Lc::n("Reset")),
        Button::plain(Lc::n("Cancel")),
    ]));
    let html = set.render(&mut Context::default()).await.into_string();

    assert!(html.contains("Save"));
    assert!(html.contains("Reset"));
    assert!(html.contains("Cancel"));
}

#[pagetop::test]
async fn button_set_remove_by_id_drops_matching_button() {
    let mut set = button::ButtonSet::new()
        .with_button(Button::submit(Lc::n("Save")).with_id("save-button"))
        .with_button(Button::plain(Lc::n("Cancel")))
        .with_button(TypedOp::RemoveById("save-button"));
    let html = set.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("Save"));
    assert!(html.contains("Cancel"));
}
