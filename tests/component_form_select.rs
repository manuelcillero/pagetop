use pagetop::prelude::*;

// `select_id()` is the id the `<select>` element gets when rendered: from the name, or from the
// field's own id when it has one.
#[pagetop::test]
async fn select_id_is_the_id_of_the_rendered_select() {
    for field in [
        form::select::Field::new().with_name("country"),
        form::select::Field::new()
            .with_name("country")
            .with_id("place"),
    ] {
        let id = field.select_id().expect("a field with a name has an id");
        let html = field
            .clone()
            .render(&mut Context::default())
            .await
            .into_string();
        assert!(
            html.contains(&format!(r#"<select id="{id}""#)),
            "{id}: {html}"
        );
    }
    assert_eq!(form::select::Field::new().select_id(), None);
}
