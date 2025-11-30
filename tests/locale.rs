use pagetop::prelude::*;

#[pagetop::test]
async fn literal_text() {
    let _app = service::test::init_service(Application::new().test()).await;

    let l10n = L10n::n("© 2025 PageTop");
    assert_eq!(l10n.get(), Some("© 2025 PageTop".to_string()));
}

#[pagetop::test]
async fn translation_without_args() {
    let _app = service::test::init_service(Application::new().test()).await;

    let l10n = L10n::l("test_hello_world");
    let translation = l10n.lookup(&LangMatch::resolve("es-ES"));
    assert_eq!(translation, Some("¡Hola mundo!".to_string()));
}

#[pagetop::test]
async fn translation_with_args() {
    let _app = service::test::init_service(Application::new().test()).await;

    let l10n = L10n::l("test_hello_user").with_arg("userName", "Manuel");
    let translation = l10n.lookup(&LangMatch::resolve("es-ES"));
    assert_eq!(translation, Some("¡Hola, Manuel!".to_string()));
}

#[pagetop::test]
async fn translation_with_plural_and_select() {
    let _app = service::test::init_service(Application::new().test()).await;

    let l10n = L10n::l("test_shared_photos").with_args(vec![
        ("userName", "Roberto"),
        ("photoCount", "3"),
        ("userGender", "male"),
    ]);
    let translation = l10n.lookup(&LangMatch::resolve("es-ES")).unwrap();
    assert!(translation.contains("añadido 3 nuevas fotos de él"));
}

#[pagetop::test]
async fn check_fallback_language() {
    let _app = service::test::init_service(Application::new().test()).await;

    let l10n = L10n::l("test_hello_world");
    let translation = l10n.lookup(&LangMatch::resolve("xx-YY")); // Retrocede a "en-US".
    assert_eq!(translation, Some("Hello world!".to_string()));
}

#[pagetop::test]
async fn check_unknown_key() {
    let _app = service::test::init_service(Application::new().test()).await;

    let l10n = L10n::l("non-existent-key");
    let translation = l10n.lookup(&LangMatch::resolve("en-US"));
    assert_eq!(translation, None);
}
