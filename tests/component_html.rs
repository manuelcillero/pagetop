use pagetop::prelude::*;

#[pagetop::test]
async fn component_html_renders_static_markup() {
    let mut component = Html::with(|_| {
        html! {
            p { "Test" }
        }
    });

    let markup = component.render(&mut Context::default());
    assert_eq!(markup.0, "<p>Test</p>");
}

#[pagetop::test]
async fn component_html_renders_using_context_param() {
    let mut cx = Context::default().with_param("username", "Alice".to_string());

    let mut component = Html::with(|cx| {
        let name = cx.param::<String>("username").cloned().unwrap_or_default();
        html! {
            span { (name) }
        }
    });

    let markup = component.render(&mut cx);
    assert_eq!(markup.0, "<span>Alice</span>");
}

#[pagetop::test]
async fn component_html_allows_replacing_render_function() {
    let mut component = Html::with(|_| html! { div { "Original" } });

    component.alter_fn(|_| html! { div { "Modified" } });

    let markup = component.render(&mut Context::default());
    assert_eq!(markup.0, "<div>Modified</div>");
}

#[pagetop::test]
async fn component_html_default_renders_empty_markup() {
    let mut component = Html::default();

    let markup = component.render(&mut Context::default());
    assert_eq!(markup.0, "");
}

#[pagetop::test]
async fn component_html_can_access_http_method() {
    let req = service::test::TestRequest::with_uri("/").to_http_request();
    let mut cx = Context::new(Some(req));

    let mut component = Html::with(|cx| {
        let method = cx
            .request()
            .map(|r| r.method().to_string())
            .unwrap_or_default();
        html! { span { (method) } }
    });

    let markup = component.render(&mut cx);
    assert_eq!(markup.0, "<span>GET</span>");
}
