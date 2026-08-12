use pagetop::prelude::*;

async fn setup() {
    Application::new().await;
}

#[pagetop::test]
async fn literal_text() {
    setup().await;

    let lc = Lc::n("© 2025 PageTop");
    assert_eq!(lc.get(), Some("© 2025 PageTop".to_string()));
}

#[pagetop::test]
async fn translation_without_args() {
    setup().await;

    let lc = Lc::l("test_hello_world");
    let translation = lc.lookup(&Locale::resolve("es-ES"));
    assert_eq!(translation, Some("¡Hola mundo!".to_string()));
}

#[pagetop::test]
async fn translation_with_args() {
    setup().await;

    let lc = Lc::l("test_hello_user").with_arg("userName", "Manuel");
    let translation = lc.lookup(&Locale::resolve("es-ES"));
    assert_eq!(translation, Some("¡Hola, Manuel!".to_string()));
}

#[pagetop::test]
async fn translation_with_plural_and_select() {
    setup().await;

    let lc = Lc::l("test_shared_photos").with_args(vec![
        ("userName", "Roberto"),
        ("photoCount", "3"),
        ("userGender", "male"),
    ]);
    let translation = lc.lookup(&Locale::resolve("es-ES")).unwrap();
    assert!(translation.contains("añadido 3 nuevas fotos de él"));
}

#[pagetop::test]
async fn check_fallback_language() {
    setup().await;

    let lc = Lc::l("test_hello_world");
    let translation = lc.lookup(&Locale::resolve("xx-YY")); // Retrocede a "en-US".
    assert_eq!(translation, Some("Hello world!".to_string()));
}

#[pagetop::test]
async fn check_unknown_key() {
    setup().await;

    let lc = Lc::l("non-existent-key");
    let translation = lc.lookup(&Locale::resolve("en-US"));
    assert_eq!(translation, None);
}

// `using()` renders literal text (`Lc::n()`) as HTML-escaped `Markup`, because it may come from
// runtime data (e.g. a menu title or a role label) rather than developer-authored content.
#[pagetop::test]
async fn literal_text_is_escaped_when_rendered_as_markup() {
    setup().await;

    let lc = Lc::n("<script>alert(1)</script>");
    let markup = lc.using(&Locale::default());
    assert_eq!(
        markup.into_string(),
        "&lt;script&gt;alert(1)&lt;/script&gt;"
    );
}

// Translation keys (`Lc::l()`/`Lc::t()`) are developer-authored `.ftl` content that may embed
// HTML on purpose (e.g. `<strong>`), so `using()` must keep rendering them unescaped.
#[pagetop::test]
async fn translated_text_is_not_escaped_when_rendered_as_markup() {
    setup().await;

    let lc = Lc::l("test_hello_world");
    let markup = lc.using(&Locale::resolve("en-US"));
    assert_eq!(markup.into_string(), "Hello world!");
}
