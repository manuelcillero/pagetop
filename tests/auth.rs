use pagetop::prelude::*;

// **< CurrentUser >********************************************************************************

#[pagetop::test]
async fn anonymous_reports_itself_correctly() {
    let user = CurrentUser::anonymous();
    assert!(user.is_anonymous());
    assert!(!user.is_authenticated());
    assert_eq!(user.id(), None);
    assert_eq!(user.display_name(), None);
}

#[pagetop::test]
async fn authenticated_reports_itself_correctly() {
    let madrid: Tz = "Europe/Madrid".parse().unwrap();
    let user = CurrentUser::authenticated(42, "Alice").with_timezone("Europe/Madrid");
    assert!(!user.is_anonymous());
    assert!(user.is_authenticated());
    assert_eq!(user.id(), Some(42));
    assert_eq!(user.display_name(), Some("Alice"));
    assert_eq!(user.timezone(), madrid);
}

#[pagetop::test]
async fn authenticated_falls_back_to_default_timezone_when_none() {
    let user = CurrentUser::authenticated(42, "Alice");
    assert_eq!(user.timezone(), Timezone::default_tz());
}

#[pagetop::test]
async fn authenticated_discards_invalid_preferences() {
    let user = CurrentUser::authenticated(42, "Alice")
        .with_language("xx-XX")
        .with_timezone("Mars/Olympus")
        .with_theme("NotRegistered");
    assert_eq!(user.language(), None);
    assert_eq!(user.timezone(), Timezone::default_tz());
    assert!(user.theme().is_none());
}

#[pagetop::test]
async fn anonymous_can_have_preferences() {
    let madrid: Tz = "Europe/Madrid".parse().unwrap();
    let user = CurrentUser::anonymous()
        .with_language("es-ES")
        .with_timezone("Europe/Madrid");
    assert!(user.is_anonymous());
    assert_eq!(user.language(), Locale::resolve("es-ES").as_option());
    assert_eq!(user.timezone(), madrid);
}

#[pagetop::test]
async fn authenticated_resolves_language_to_a_supported_one() {
    let user = CurrentUser::authenticated(42, "Alice").with_language("es");
    assert_eq!(user.language(), Locale::resolve("es").as_option());
    assert!(user.language().is_some());
}

// **< Context::current_user() >********************************************************************

#[pagetop::test]
async fn current_user_defaults_to_anonymous() {
    let cx = Context::default();
    assert!(cx.current_user().is_anonymous());
}

#[pagetop::test]
async fn current_user_propagates_from_request_extensions() {
    let req = web::test::TestRequest::get()
        .with_extension(CurrentUser::authenticated(7, "Bob"))
        .to_http_request();
    let cx = Context::new(req);
    let user = cx.current_user();
    assert!(user.is_authenticated());
    assert_eq!(user.id(), Some(7));
    assert_eq!(user.display_name(), Some("Bob"));
}

#[pagetop::test]
async fn user_language_sets_the_request_language() {
    let req = web::test::TestRequest::get()
        .header("Accept-Language", "en-US")
        .with_extension(CurrentUser::authenticated(7, "Bob").with_language("es-ES"))
        .to_http_request();
    let cx = Context::new(req);
    assert_eq!(cx.langid().to_string(), "es-ES");
}

#[pagetop::test]
async fn lang_query_overrides_user_language() {
    let req = web::test::TestRequest::get()
        .uri("/?lang=en-US")
        .with_extension(CurrentUser::authenticated(7, "Bob").with_language("es-ES"))
        .to_http_request();
    let cx = Context::new(req);
    assert_eq!(cx.langid().to_string(), "en-US");
}

struct UserTheme;

impl Extension for UserTheme {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&UserTheme)
    }
}

impl Theme for UserTheme {}

#[pagetop::test]
async fn user_theme_sets_the_context_theme() {
    let _ = Application::prepare(&UserTheme).await;
    let req = web::test::TestRequest::get()
        .with_extension(CurrentUser::authenticated(7, "Bob").with_theme("UserTheme"))
        .to_http_request();
    let cx = Context::new(req);
    assert_eq!(cx.theme().short_name(), "UserTheme");
}

#[pagetop::test]
async fn context_uses_the_default_theme_without_user_theme() {
    let cx = Context::default();
    assert_eq!(cx.theme().short_name(), "Basic");
}

// **< HttpRequest::extension() >*******************************************************************

#[pagetop::test]
async fn request_extension_returns_none_for_unknown_type() {
    let req = web::test::TestRequest::get().to_http_request();
    assert!(req.extension::<CurrentUser>().is_none());
}

#[pagetop::test]
async fn request_extension_returns_injected_value() {
    let req = web::test::TestRequest::get()
        .with_extension(CurrentUser::authenticated(1, "Carol"))
        .to_http_request();

    let user = req
        .extension::<CurrentUser>()
        .expect("extension should exist");
    assert!(user.is_authenticated());
    assert_eq!(user.id(), Some(1));
    assert_eq!(user.display_name(), Some("Carol"));
}

// **< Page::new() >********************************************************************************

#[pagetop::test]
async fn page_new_propagates_current_user_from_request_extensions() {
    let req = web::test::TestRequest::get()
        .with_extension(CurrentUser::authenticated(5, "Dave"))
        .to_http_request();
    let page = Page::new(req);
    let user = page.current_user();
    assert!(user.is_authenticated());
    assert_eq!(user.id(), Some(5));
    assert_eq!(user.display_name(), Some("Dave"));
}

#[pagetop::test]
async fn page_new_uses_anonymous_when_no_extension() {
    let req = web::test::TestRequest::get().to_http_request();
    let page = Page::new(req);
    assert!(page.current_user().is_anonymous());
}
