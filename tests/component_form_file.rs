use pagetop::prelude::*;

#[pagetop::test]
async fn file_field_renders_a_file_input() {
    let mut field = form::File::new().with_name("photo");
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"class="form-field form-field-file""#));
    assert!(html.contains(r#"id="edit-photo""#));
    assert!(html.contains(r#"type="file""#));
    assert!(html.contains(r#"id="edit-photo-file""#));
    assert!(html.contains(r#"name="photo""#));
}

#[pagetop::test]
async fn file_field_optional_attributes_are_absent_by_default() {
    let mut field = form::File::new().with_name("photo");
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("accept="));
    assert!(!html.contains("multiple"));
    assert!(!html.contains("required"));
    assert!(!html.contains("disabled"));
    assert!(!html.contains(r#"class="form-label""#));
}

#[pagetop::test]
async fn file_field_renders_accept_multiple_and_required() {
    let mut field = form::File::new()
        .with_name("docs")
        .with_label(Lc::n("Documents"))
        .with_accept(".pdf,image/*")
        .with_multiple(true)
        .with_required(true);
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"accept=".pdf,image/*""#));
    assert!(html.contains("multiple"));
    assert!(html.contains("required"));
    assert!(html.contains(r#"for="edit-docs-file""#));
    assert!(html.contains("Documents"));
}

#[pagetop::test]
async fn file_field_empty_accept_removes_the_attribute() {
    let mut field = form::File::new()
        .with_name("photo")
        .with_accept("image/*")
        .with_accept("");
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("accept="));
}

#[pagetop::test]
async fn form_enctype_is_omitted_by_default() {
    let mut form = Form::new().with_action("/send");
    let html = form.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("enctype="));
}

#[pagetop::test]
async fn form_multipart_enctype_is_rendered() {
    let mut form = Form::new()
        .with_action("/send")
        .with_enctype(form::Enctype::Multipart);
    let html = form.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"enctype="multipart/form-data""#));
}
