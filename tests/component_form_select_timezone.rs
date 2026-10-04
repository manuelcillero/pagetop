use pagetop::prelude::*;

#[pagetop::test]
async fn groups_zones_by_region_and_marks_the_selected_one() {
    let mut field = form::SelectTimezone::new()
        .with_name("timezone")
        .with_selected("Europe/Madrid");
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"<optgroup label="Europe">"#));
    assert!(html.contains(r#"<option value="Europe/Madrid" selected>"#));
    assert!(html.contains(r#"value="Etc/UTC""#));
}

#[pagetop::test]
async fn region_labels_are_translated() {
    let mut field = form::SelectTimezone::new();
    let mut cx = Context::default().with_langid(&Locale::resolve("es-ES"));
    let html = field.render(&mut cx).await.into_string();

    assert!(html.contains(r#"<optgroup label="Europa">"#));
    assert!(html.contains(r#"<optgroup label="Otras">"#));
}

#[pagetop::test]
async fn legacy_and_fixed_offset_zones_are_not_offered() {
    let mut field = form::SelectTimezone::new();
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(!html.contains(r#"value="US/Eastern""#));
    assert!(!html.contains(r#"value="Japan""#));
    assert!(!html.contains(r#"value="Etc/GMT+1""#));
}

#[pagetop::test]
async fn optional_field_offers_the_site_time_zone_first() {
    let mut field = form::SelectTimezone::new();
    let html = field.render(&mut Context::default()).await.into_string();

    let site = html.find(r#"<option value="" selected>Use the site time zone: "#);
    let first_group = html.find("<optgroup");
    assert!(site.is_some());
    assert!(site < first_group);
}

#[pagetop::test]
async fn required_field_asks_to_choose_only_when_nothing_valid_is_selected() {
    let mut empty = form::SelectTimezone::new().with_required(true);
    let html = empty.render(&mut Context::default()).await.into_string();
    assert!(html.contains(r#"<option value="" selected>Choose a time zone...</option>"#));

    let mut unknown = form::SelectTimezone::new()
        .with_required(true)
        .with_selected("Mars/Olympus");
    let html = unknown.render(&mut Context::default()).await.into_string();
    assert!(html.contains(r#"<option value="" selected>Choose a time zone...</option>"#));

    let mut chosen = form::SelectTimezone::new()
        .with_required(true)
        .with_selected("Europe/Madrid");
    let html = chosen.render(&mut Context::default()).await.into_string();
    assert!(!html.contains(r#"value="""#));
}
