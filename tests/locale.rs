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

#[pagetop::test]
async fn collation_key_is_case_and_accent_insensitive() {
    setup().await;

    let lower = Lc::n("café").collation_key(&Locale::default());
    let upper = Lc::n("CAFÉ").collation_key(&Locale::default());
    assert_eq!(lower, upper);
}

// `ñ` sorts as its own letter (Spanish/RAE order), between "n..." and "o...", not merged with `n`
// like the other accented letters.
#[pagetop::test]
async fn collation_key_orders_n_tilde_like_spanish() {
    setup().await;

    let mut words = vec![Lc::n("Ñu"), Lc::n("Nube"), Lc::n("Oso"), Lc::n("Café")];
    words.sort_by_key(|w| w.collation_key(&Locale::default()));

    let sorted: Vec<_> = words.iter().map(|w| w.get().unwrap()).collect();
    assert_eq!(sorted, vec!["Café", "Nube", "Ñu", "Oso"]);
}

// Minimal pair sharing the same prefix ("an-"/"añ-"): the comparison is decided by the second
// letter alone (`n` < `ñ`), regardless of how the words continue afterwards ("anual" has more
// letters after the `n` than "año" has after the `ñ`, and still sorts first).
#[pagetop::test]
async fn collation_key_orders_n_tilde_before_further_letters() {
    setup().await;

    let mut words = [Lc::n("Año"), Lc::n("Anzuelo"), Lc::n("Anual")];
    words.sort_by_key(|w| w.collation_key(&Locale::default()));

    let sorted: Vec<_> = words.iter().map(|w| w.get().unwrap()).collect();
    assert_eq!(sorted, vec!["Anual", "Anzuelo", "Año"]);
}

// `collation_key()` sorts the *resolved* translation, not the raw key, so the same `Lc` yields a
// different key depending on the language passed in.
#[pagetop::test]
async fn collation_key_sorts_the_resolved_translation() {
    setup().await;

    let lc = Lc::l("test_hello_world");
    let key_en = lc.collation_key(&Locale::resolve("en-US"));
    let key_es = lc.collation_key(&Locale::resolve("es-ES"));
    assert_ne!(key_en, key_es);
}
