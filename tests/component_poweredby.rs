use pagetop::prelude::*;

/// Initializes PageTop (locale, extensions...) once for the whole suite.
///
/// The tests in this module render components directly with `Context::default()`, so they only need
/// the localization subsystem and the registered extensions, not a router.
async fn setup() {
    Application::new().await;
}

#[pagetop::test]
async fn poweredby_default_shows_only_pagetop_recognition() {
    setup().await;

    let mut p = PoweredBy::default();
    let html = p.render(&mut Context::default()).await.into_string();

    // Should show the PageTop acknowledgment block.
    assert!(html.contains("poweredby__pagetop"));

    // And should NOT show the copyright block.
    assert!(!html.contains("poweredby__copyright"));
}

#[pagetop::test]
async fn poweredby_new_includes_current_year_and_app_name() {
    setup().await;

    let mut p = PoweredBy::new();
    let html = p.render(&mut Context::default()).await.into_string();

    let year = Utc::now().format("%Y").to_string();
    assert!(html.contains(&year), "HTML should include the current year");

    // The app name comes from `global::SETTINGS.app.name`.
    let app_name = &global::SETTINGS.app.name;
    assert!(
        html.contains(app_name),
        "HTML should include the application name"
    );

    // The copyright span must exist.
    assert!(html.contains("poweredby__copyright"));
}

#[pagetop::test]
async fn poweredby_with_copyright_overrides_text() {
    setup().await;

    let custom = "2001 © FooBar Inc.";
    let mut p = PoweredBy::default().with_copyright(Some(custom));
    let html = p.render(&mut Context::default()).await.into_string();

    assert!(html.contains(custom));
    assert!(html.contains("poweredby__copyright"));
}

#[pagetop::test]
async fn poweredby_with_copyright_none_hides_text() {
    setup().await;

    let mut p = PoweredBy::new().with_copyright(None::<String>);
    let html = p.render(&mut Context::default()).await.into_string();

    assert!(!html.contains("poweredby__copyright"));
    // The PageTop acknowledgment must always appear.
    assert!(html.contains("poweredby__pagetop"));
}

#[pagetop::test]
async fn poweredby_link_points_to_crates_io() {
    setup().await;

    let mut p = PoweredBy::default();
    let html = p.render(&mut Context::default()).await.into_string();

    assert!(
        html.contains("https://pagetop.cillero.es"),
        "Link should point to pagetop.cillero.es"
    );
}

#[pagetop::test]
async fn poweredby_getter_reflects_internal_state() {
    setup().await;

    // There is no copyright by default.
    let p0 = PoweredBy::default();
    assert_eq!(p0.copyright(), None);

    // And `new()` initializes it with year + app name.
    let p1 = PoweredBy::new();
    let c1 = p1.copyright().expect("Expected copyright to exist");
    assert!(c1.contains(&Utc::now().format("%Y").to_string()));
    assert!(c1.contains(&global::SETTINGS.app.name));
}
