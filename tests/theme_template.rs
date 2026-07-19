use pagetop::prelude::*;

// **< Theme with its own template >****************************************************************

struct MarkerTemplate;

#[async_trait]
impl Template for MarkerTemplate {
    async fn render(&self, _cx: &mut Context) -> Markup {
        html! { "marker-template-output" }
    }
}

struct MarkerTheme;

#[async_trait]
impl Extension for MarkerTheme {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Self)
    }
}

#[async_trait]
impl Theme for MarkerTheme {
    fn default_template(&self) -> TemplateRef {
        &MarkerTemplate
    }

    fn admin_template(&self) -> TemplateRef {
        &MarkerTemplate
    }
}

async fn render_active_template(cx: &mut Context) -> String {
    let template = cx.template();
    template.render(cx).await.into_string()
}

// **< Context::template() follows the active theme >***********************************************

#[pagetop::test]
async fn with_theme_updates_the_effective_template() {
    // Without changing theme, the active template is not `MarkerTheme`'s.
    let mut cx = Context::new(None);
    assert_ne!(
        render_active_template(&mut cx).await,
        "marker-template-output"
    );

    // After changing theme with `with_theme()`, the active template becomes that theme's, with no
    // need to call `with_template()` explicitly.
    let mut cx = Context::new(None).with_theme(&MarkerTheme);
    assert_eq!(
        render_active_template(&mut cx).await,
        "marker-template-output"
    );
}

#[pagetop::test]
async fn explicit_template_is_not_overridden_by_a_later_with_theme() {
    // A template explicitly set with `with_template()` prevails even if `with_theme()` is called
    // afterwards, regardless of order.
    let mut cx = Context::new(None)
        .with_template(&MarkerTemplate)
        .with_theme(&pagetop::base::theme::Basic);

    assert_eq!(
        render_active_template(&mut cx).await,
        "marker-template-output"
    );
}

// **< Page::admin() follows the active theme >*****************************************************

#[pagetop::test]
async fn page_admin_template_follows_a_later_with_theme() {
    let request = web::test::TestRequest::get().to_http_request();

    let mut page = Page::admin(request).with_theme(&MarkerTheme);
    let markup = page.context().template().render(page.context()).await;

    assert_eq!(markup.into_string(), "marker-template-output");
}
