use pagetop::prelude::*;

// **< Tema con plantilla propia >******************************************************************

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

// **< Context::template() sigue al tema activo >***************************************************

#[pagetop::test]
async fn with_theme_updates_the_effective_template() {
    // Sin cambiar de tema, la plantilla activa no es la de `MarkerTheme`.
    let mut cx = Context::new(None);
    assert_ne!(
        render_active_template(&mut cx).await,
        "marker-template-output"
    );

    // Tras cambiar de tema con `with_theme()`, la plantilla activa pasa a ser la de ese tema, sin
    // necesidad de llamar a `with_template()` explícitamente.
    let mut cx = Context::new(None).with_theme(&MarkerTheme);
    assert_eq!(
        render_active_template(&mut cx).await,
        "marker-template-output"
    );
}

#[pagetop::test]
async fn explicit_template_is_not_overridden_by_a_later_with_theme() {
    // Una plantilla fijada explícitamente con `with_template()` prevalece aunque `with_theme()` se
    // llame después, en cualquier orden.
    let mut cx = Context::new(None)
        .with_template(&MarkerTemplate)
        .with_theme(&pagetop::base::theme::Basic);

    assert_eq!(
        render_active_template(&mut cx).await,
        "marker-template-output"
    );
}

// **< Page::admin() sigue al tema activo >*********************************************************

#[pagetop::test]
async fn page_admin_template_follows_a_later_with_theme() {
    let request = web::test::TestRequest::get().to_http_request();

    let mut page = Page::admin(request).with_theme(&MarkerTheme);
    let markup = page.context().template().render(page.context()).await;

    assert_eq!(markup.into_string(), "marker-template-output");
}
