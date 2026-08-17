use pagetop::prelude::*;

#[pagetop::test]
async fn label_is_absent_by_default() {
    let mut field = form::input::Field::text();
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(!html.contains(r#"class="form-label""#));
}

#[pagetop::test]
async fn label_is_rendered_when_set() {
    let mut field = form::input::Field::text().with_label(Lc::n("Full name"));
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"class="form-label""#));
    assert!(html.contains("Full name"));
}

#[pagetop::test]
async fn label_can_be_cleared_with_lc_none() {
    let mut field = form::input::Field::text()
        .with_label(Lc::n("Full name"))
        .with_label(Lc::none());
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(!html.contains(r#"class="form-label""#));
    assert!(!html.contains("Full name"));
}

#[pagetop::test]
async fn help_text_is_absent_by_default() {
    let mut field = form::input::Field::text();
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(!html.contains(r#"class="form-text""#));
}

#[pagetop::test]
async fn help_text_is_rendered_when_set() {
    let mut field = form::input::Field::text().with_help_text(Lc::n("We never share your data."));
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"class="form-text""#));
    assert!(html.contains("We never share your data."));
}

#[pagetop::test]
async fn placeholder_attribute_is_absent_by_default() {
    let mut field = form::input::Field::text();
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("placeholder="));
}

#[pagetop::test]
async fn placeholder_attribute_is_present_when_set() {
    let mut field = form::input::Field::text().with_placeholder(Lc::n("Enter your name"));
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"placeholder="Enter your name""#));
}

#[pagetop::test]
async fn placeholder_attribute_can_be_cleared_with_lc_none() {
    let mut field = form::input::Field::text()
        .with_placeholder(Lc::n("Enter your name"))
        .with_placeholder(Lc::none());
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("placeholder="));
}
