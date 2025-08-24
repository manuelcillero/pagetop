use pagetop::prelude::*;

#[pagetop::test]
async fn poweredby_default_shows_only_pagetop_recognition() {
    let _app = service::test::init_service(Application::new().test()).await;

    let p = PoweredBy::default();
    let html = render_component(&p);

    // Debe mostrar el bloque de reconocimiento a PageTop.
    assert!(html.contains("poweredby__pagetop"));

    // Y NO debe mostrar el bloque de copyright.
    assert!(!html.contains("poweredby__copyright"));
}

#[pagetop::test]
async fn poweredby_new_includes_current_year_and_app_name() {
    let _app = service::test::init_service(Application::new().test()).await;

    let p = PoweredBy::new();
    let html = render_component(&p);

    let year = Utc::now().format("%Y").to_string();
    assert!(html.contains(&year), "HTML should include the current year");

    // El nombre de la app proviene de `global::SETTINGS.app.name`.
    let app_name = &global::SETTINGS.app.name;
    assert!(
        html.contains(app_name),
        "HTML should include the application name"
    );

    // Debe existir el span de copyright.
    assert!(html.contains("poweredby__copyright"));
}

#[pagetop::test]
async fn poweredby_with_copyright_overrides_text() {
    let _app = service::test::init_service(Application::new().test()).await;

    let custom = "2001 © FooBar Inc.";
    let p = PoweredBy::default().with_copyright(Some(custom));
    let html = render_component(&p);

    assert!(html.contains(custom));
    assert!(html.contains("poweredby__copyright"));
}

#[pagetop::test]
async fn poweredby_with_copyright_none_hides_text() {
    let _app = service::test::init_service(Application::new().test()).await;

    let p = PoweredBy::new().with_copyright(None::<String>);
    let html = render_component(&p);

    assert!(!html.contains("poweredby__copyright"));
    // El reconocimiento a PageTop siempre debe aparecer.
    assert!(html.contains("poweredby__pagetop"));
}

#[pagetop::test]
async fn poweredby_link_points_to_crates_io() {
    let _app = service::test::init_service(Application::new().test()).await;

    let p = PoweredBy::default();
    let html = render_component(&p);

    assert!(
        html.contains("https://crates.io/crates/pagetop"),
        "Link should point to crates.io/pagetop"
    );
}

#[pagetop::test]
async fn poweredby_getter_reflects_internal_state() {
    let _app = service::test::init_service(Application::new().test()).await;

    // Por defecto no hay copyright.
    let p0 = PoweredBy::default();
    assert_eq!(p0.copyright(), None);

    // Y `new()` lo inicializa con año + nombre de app.
    let p1 = PoweredBy::new();
    let c1 = p1.copyright().expect("Expected copyright to exis");
    assert!(c1.contains(&Utc::now().format("%Y").to_string()));
    assert!(c1.contains(&global::SETTINGS.app.name));
}

// HELPERS *****************************************************************************************

fn render(x: &impl Render) -> String {
    x.render().into_string()
}

fn render_component<C: Component>(c: &C) -> String {
    let mut cx = Context::default();
    let pm = c.prepare_component(&mut cx);
    render(&pm)
}
