use pagetop::prelude::*;
use pagetop_htmx::Htmx;

struct TestApp;

#[async_trait]
impl Extension for TestApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&Htmx]
    }

    fn configure_router(&self, router: Router) -> Router {
        router.route("/page", web::get(render_page))
    }
}

async fn render_page(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(Html::with(|_| html! { p { "hello" } }))
        .render()
        .await
}

// All tests in this file share the same root extension (`TestApp`), since `EXTENSIONS` is a global
// `OnceLock` initialized only once per test binary (see `core/extension/all.rs`).

// **< Static assets >******************************************************************************

#[pagetop::test]
async fn htmx_script_is_served_at_the_expected_static_path() {
    let app = web::test::init_router(Application::prepare(&TestApp).await.test());

    let req = web::test::TestRequest::get()
        .uri("/htmx/js/htmx.min.js")
        .to_request();
    let resp = web::test::send_request(&app, req).await;

    assert_eq!(resp.status(), web::http::StatusCode::OK);

    let body = web::test::read_body_text(resp).await;
    assert!(!body.is_empty());
    assert!(body.contains("htmx"));
}

// **< Automatic script injection (BeforeRenderBody) >***********************************************

#[pagetop::test]
async fn rendered_pages_automatically_include_the_pinned_htmx_script_tag() {
    let app = web::test::init_router(Application::prepare(&TestApp).await.test());

    let req = web::test::TestRequest::get().uri("/page").to_request();
    let resp = web::test::send_request(&app, req).await;

    assert_eq!(resp.status(), web::http::StatusCode::OK);

    let body = web::test::read_body_text(resp).await;
    // The version must stay in sync with the bundled `assets/js/htmx.min.js`; a mismatch here
    // would mean the browser caches a stale script under a version tag that no longer matches it.
    assert!(body.contains(r#"src="/htmx/js/htmx.min.js?v=2.0.10""#));
    assert!(body.contains("defer"));
    assert!(body.contains("<p>hello</p>"));
}
