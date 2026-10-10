use pagetop::prelude::*;

// `form::parts::Label` renders the label of a form field with the required mark, in the three
// shapes used by the fields: linked to a control, next to a checkbox, or naming a group.

fn label() -> Lc {
    Lc::n("Name")
}

#[pagetop::test]
async fn a_control_label_points_to_the_control() {
    let cx = Context::default();
    let text = label();
    let field = form::parts::Label::new(&text, Some("edit-a-input"));
    assert_eq!(field.id(), None);
    assert_eq!(
        field.render(&cx).into_string(),
        r#"<label for="edit-a-input" class="form-label">Name</label>"#
    );
}

#[pagetop::test]
async fn required_fields_get_the_translated_mark() {
    let cx = Context::default().with_langid(&Locale::resolve("en-US"));
    let html = form::parts::Label::new(&label(), Some("edit-a-input"))
        .with_required(true)
        .render(&cx)
        .into_string();
    let title = Lc::l("field_required").lookup(&cx).unwrap();
    assert!(html.contains(&format!(
        r#"<span class="form-required" title="{title}">*</span>"#
    )));
}

#[pagetop::test]
async fn a_checkbox_label_uses_its_own_class() {
    let cx = Context::default();
    let html = form::parts::Label::check(&label(), Some("edit-b-checkbox"))
        .render(&cx)
        .into_string();
    assert_eq!(
        html,
        r#"<label for="edit-b-checkbox" class="form-check-label">Name</label>"#
    );
}

#[pagetop::test]
async fn a_group_label_has_an_id_and_no_for() {
    let cx = Context::default();
    let text = label();
    let field = form::parts::Label::group(&text, "edit-c");
    assert_eq!(field.id(), Some("edit-c-label"));
    assert_eq!(
        field.render(&cx).into_string(),
        r#"<label id="edit-c-label" class="form-label">Name</label>"#
    );
}

#[pagetop::test]
async fn without_text_there_is_no_label_nor_id() {
    let cx = Context::default();
    let none = Lc::none();
    let field = form::parts::Label::group(&none, "edit-c").with_required(true);
    assert_eq!(field.id(), None);
    assert_eq!(field.render(&cx).into_string(), "");
}

#[pagetop::test]
async fn the_class_can_be_replaced() {
    let cx = Context::default();
    let html = form::parts::Label::check(&label(), Some("edit-b-checkbox"))
        .with_class("control-label")
        .render(&cx)
        .into_string();
    assert_eq!(
        html,
        r#"<label for="edit-b-checkbox" class="control-label">Name</label>"#
    );
}

#[pagetop::test]
async fn a_group_label_without_translation_keeps_the_linked_element() {
    let cx = Context::default();
    // A missing translation still renders the element, so `aria-labelledby` never dangles.
    let missing = Lc::l("no_such_key_for_label");
    let field = form::parts::Label::group(&missing, "edit-c");
    assert_eq!(field.id(), Some("edit-c-label"));
    assert_eq!(
        field.render(&cx).into_string(),
        r#"<label id="edit-c-label" class="form-label"></label>"#
    );
}
