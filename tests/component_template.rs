use pagetop::prelude::*;

/// Initializes PageTop (locale, extensions...) once for the whole suite.
///
/// Rendering a `Region`/`Template` looks up localized labels (`aria-label`, etc.), so tests that
/// render them need the localization subsystem loaded.
async fn setup() {
    Application::new().await;
}

// **< A theme that intercepts the `Template` component >*******************************************

/// Replaces the default `Template` composition (`Header` + `Content` + `Footer`) with a fixed
/// marker string, for both `CoreTemplates::Standard` and `CoreTemplates::Admin`. Mirrors how
/// a real theme (e.g. `pagetop-bootsier`) tells its own layout apart from PageTop's default: by
/// intercepting the `Template` component in `handle_component()`, not by swapping which
/// `TemplateRef` gets resolved.
struct MarkerTheme;

#[async_trait]
impl Extension for MarkerTheme {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Self)
    }
}

#[async_trait]
impl Theme for MarkerTheme {
    async fn handle_component(
        &self,
        component: &mut dyn Component,
        _cx: &mut Context,
    ) -> Option<Result<Markup, ComponentError>> {
        let template = (*component).downcast_ref::<layout::Template>()?;
        template.template().downcast_ref::<CoreTemplates>()?;
        Some(Ok(html! { "marker-template-output" }))
    }
}

// **< Default/Admin template identity is independent of the active theme >*************************
//
// `Theme::default_template()`/`admin_template()` were removed: `Context::template()` always
// resolves `Default`/`Admin` to the core `CoreTemplates::Standard`/`Admin` identity, regardless
// of which theme is active. Themes customize the actual rendering by intercepting the `Template`
// component in `handle_component()` instead (see the tests further below).

#[pagetop::test]
async fn default_template_identity_is_independent_of_theme() {
    let cx = Context::default();
    assert_eq!(cx.template().name(), "standard");

    let cx = Context::default().with_theme(&MarkerTheme);
    assert_eq!(cx.template().name(), "standard");
}

#[pagetop::test]
async fn admin_template_identity_is_independent_of_theme() {
    let mut page = Page::admin(web::test::TestRequest::get().to_http_request());
    assert_eq!(page.context().template().name(), "admin");

    let mut page =
        Page::admin(web::test::TestRequest::get().to_http_request()).with_theme(&MarkerTheme);
    assert_eq!(page.context().template().name(), "admin");
}

#[pagetop::test]
async fn explicit_template_is_not_overridden_by_a_later_with_theme() {
    // A template explicitly set with `with_template()` prevails even if `with_theme()` is called
    // afterwards.
    let cx = Context::default()
        .with_template(&CoreTemplates::Admin)
        .with_theme(&pagetop::base::theme::Basic);

    assert_eq!(cx.template().name(), "admin");
}

// **< A theme customizes rendering via `handle_component()` >**************************************

#[pagetop::test]
async fn without_a_matching_theme_the_default_composition_is_used() {
    setup().await;

    // With no theme intercepting it, and no content registered in any region, the default
    // composition (Header + Content + Footer) renders empty.
    let mut template = layout::Template::default();
    let html = template.render(&mut Context::default()).await.into_string();

    assert!(html.is_empty());
}

#[pagetop::test]
async fn theme_replaces_template_rendering_via_handle_component() {
    setup().await;

    let mut template = layout::Template::default();
    let mut cx = Context::default().with_theme(&MarkerTheme);
    let html = template.render(&mut cx).await.into_string();

    assert_eq!(html, "marker-template-output");
}

// **< Page::render() reaches the active theme's `handle_component()` >*****************************

#[pagetop::test]
async fn page_admin_render_reflects_the_active_theme_template() {
    setup().await;

    let request = web::test::TestRequest::get().to_http_request();
    let mut page = Page::admin(request).with_theme(&MarkerTheme);

    let html = page
        .render()
        .await
        .expect("page should render")
        .into_string();

    assert!(html.contains("marker-template-output"));
}
