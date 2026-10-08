use pagetop::prelude::*;

fn cx(language: &str, timezone: &str) -> Context {
    Context::default()
        .with_langid(&Locale::resolve(language))
        .with_timezone(timezone.parse().unwrap())
}

#[pagetop::test]
async fn dates_are_shown_in_the_format_of_the_language() {
    let date = NaiveDate::from_ymd_opt(2026, 10, 6).unwrap();

    let mut field = form::date::Field::date()
        .with_name("expires")
        .with_date(date);
    let html = field.render(&mut cx("es-ES", "UTC")).await.into_string();
    assert!(html.contains(r#"value="06/10/2026""#));
    assert!(html.contains(r#"placeholder="dd/mm/aaaa""#));
    assert!(html.contains("form-field form-field-date"));

    let html = field.render(&mut cx("en-US", "UTC")).await.into_string();
    assert!(html.contains(r#"value="10/06/2026""#));
    assert!(html.contains(r#"placeholder="mm/dd/yyyy""#));
}

#[pagetop::test]
async fn datetimes_are_shown_in_the_local_time_of_the_user() {
    let dt = Utc.with_ymd_and_hms(2026, 10, 6, 12, 30, 0).unwrap();

    let mut field = form::date::Field::datetime()
        .with_name("at")
        .with_datetime(dt);
    let html = field
        .render(&mut cx("es-ES", "Europe/Madrid"))
        .await
        .into_string();
    assert!(html.contains(r#"value="06/10/2026 14:30""#));
    assert!(html.contains(r#"placeholder="dd/mm/aaaa hh:mm""#));
}

#[pagetop::test]
async fn shown_values_are_read_back_unchanged() {
    let dt = Utc.with_ymd_and_hms(2026, 10, 6, 12, 30, 0).unwrap();
    let mut cx = cx("en-US", "America/New_York");

    let mut field = form::date::Field::datetime()
        .with_name("at")
        .with_datetime(dt);
    let html = field.render(&mut cx).await.into_string();
    let value = html
        .split(r#"value=""#)
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    assert_eq!(cx.parse_datetime(value), Ok(Some(dt)));
}

#[pagetop::test]
async fn text_is_shown_as_written_and_empty_fields_have_no_value() {
    let mut field = form::date::Field::time().with_name("at").with_text("25:99");
    let html = field.render(&mut cx("es-ES", "UTC")).await.into_string();
    assert!(html.contains(r#"value="25:99""#));
    assert!(html.contains("form-field form-field-time"));

    let mut empty = form::date::Field::date().with_name("expires");
    let html = empty.render(&mut cx("es-ES", "UTC")).await.into_string();
    assert!(!html.contains("value="));
}

#[pagetop::test]
async fn a_value_of_another_kind_is_ignored() {
    let date = NaiveDate::from_ymd_opt(2026, 10, 6).unwrap();
    let mut field = form::date::Field::time().with_name("at").with_date(date);
    let html = field.render(&mut cx("es-ES", "UTC")).await.into_string();
    assert!(!html.contains("value="));
}

// The format stays visible (and is announced by screen readers) once the placeholder is gone.
#[pagetop::test]
async fn the_format_is_also_a_help_text_linked_to_the_input() {
    let mut field = form::date::Field::date().with_name("expires");
    let html = field.render(&mut cx("es-ES", "UTC")).await.into_string();
    assert!(html.contains(r#"aria-describedby="edit-expires-input-format""#));
    assert!(html.contains(
        r#"<div id="edit-expires-input-format" class="form-text">Formato: dd/mm/aaaa</div>"#
    ));
}

#[pagetop::test]
async fn autocomplete_is_off_unless_set() {
    let mut field = form::date::Field::date().with_name("born");
    let html = field.render(&mut cx("es-ES", "UTC")).await.into_string();
    assert!(html.contains(r#"autocomplete="off""#));

    let mut field = form::date::Field::date()
        .with_name("born")
        .with_autocomplete(form::Autocomplete::token(form::AutofillField::Bday));
    let html = field.render(&mut cx("es-ES", "UTC")).await.into_string();
    assert!(html.contains(r#"autocomplete="bday""#));
}

#[pagetop::test]
async fn required_readonly_and_disabled_are_rendered() {
    let mut field = form::date::Field::date()
        .with_name("expires")
        .with_label(Lc::n("Expires"))
        .with_required(true)
        .with_readonly(true)
        .with_disabled(true);
    let html = field.render(&mut cx("es-ES", "UTC")).await.into_string();
    assert!(html.contains(r#"class="form-required""#));
    assert!(html.contains(" readonly"));
    assert!(html.contains(" required"));
    assert!(html.contains(" disabled"));
}
