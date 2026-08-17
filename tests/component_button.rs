use pagetop::prelude::*;

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
