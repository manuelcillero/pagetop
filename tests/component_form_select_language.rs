use pagetop::prelude::*;

#[pagetop::test]
async fn offers_every_supported_language_and_marks_the_selected_one() {
    let mut field = form::SelectLanguage::new()
        .with_name("language")
        .with_selected("es-ES");
    let html = field.render(&mut Context::default()).await.into_string();

    for (langid, _) in Locale::supported_languages() {
        assert!(html.contains(&format!(r#"value="{langid}""#)));
    }
    assert!(html.contains(r#"<option value="es-ES" selected>"#));
}

#[pagetop::test]
async fn optional_field_offers_the_site_language_first() {
    let mut field = form::SelectLanguage::new().with_selected("es-ES");
    let html = field.render(&mut Context::default()).await.into_string();

    let site = html.find(r#"<option value="">Use the site language: "#);
    let first_language = html.find(r#"<option value="en"#);
    assert!(site.is_some());
    assert!(site < first_language);
}

#[pagetop::test]
async fn optional_field_selects_the_site_language_when_nothing_is_selected() {
    let mut field = form::SelectLanguage::new();
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"<option value="" selected>Use the site language: "#));
}

#[pagetop::test]
async fn required_field_asks_to_choose_when_nothing_is_selected() {
    let mut field = form::SelectLanguage::new().with_required(true);
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(html.contains(r#"<option value="" selected>Choose a language...</option>"#));
    assert!(!html.contains("Use the site language"));
}

#[pagetop::test]
async fn required_field_has_no_empty_option_when_a_language_is_selected() {
    let mut field = form::SelectLanguage::new()
        .with_required(true)
        .with_selected("es-ES");
    let html = field.render(&mut Context::default()).await.into_string();

    assert!(!html.contains(r#"value="""#));
}

#[pagetop::test]
async fn language_aliases_select_their_canonical_language() {
    for alias in ["es", "es-es", "ES-ES"] {
        let mut field = form::SelectLanguage::new().with_selected(alias);
        let html = field.render(&mut Context::default()).await.into_string();

        assert!(html.contains(r#"<option value="es-ES" selected>"#));
        assert!(html.contains(r#"<option value="">Use the site language: "#));
    }
}

#[pagetop::test]
async fn unknown_selected_value_falls_back_to_the_empty_option() {
    let mut optional = form::SelectLanguage::new().with_selected("xx-XX");
    let html = optional.render(&mut Context::default()).await.into_string();
    assert!(html.contains(r#"<option value="" selected>Use the site language: "#));

    let mut required = form::SelectLanguage::new()
        .with_required(true)
        .with_selected("xx-XX");
    let html = required.render(&mut Context::default()).await.into_string();
    assert!(html.contains(r#"<option value="" selected>Choose a language...</option>"#));
}

#[pagetop::test]
async fn languages_are_sorted_by_their_translated_name() {
    let mut field = form::SelectLanguage::new();

    let mut cx = Context::default().with_langid(&Locale::resolve("es-ES"));
    let html = field.render(&mut cx).await.into_string();
    let spanish = html
        .find(">Español (España)</option>")
        .expect("Spanish name");
    let english = html
        .find(">Inglés (Estados Unidos)</option>")
        .expect("English name");
    assert!(spanish < english);

    let mut cx = Context::default().with_langid(&Locale::resolve("en-US"));
    let html = field.render(&mut cx).await.into_string();
    let spanish = html
        .find(">Spanish (Spain)</option>")
        .expect("Spanish name");
    let english = html
        .find(">English (United States)</option>")
        .expect("English name");
    assert!(english < spanish);
}
