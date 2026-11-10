use pagetop::prelude::*;
use pagetop_htmx::Htmx;

struct TestApp;

#[async_trait]
impl Extension for TestApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&Htmx]
    }

    fn configure_router(&self, routes: Routes) -> Routes {
        routes
            .public_route("/page", web::get(render_page))
            .public_route("/list", web::get(render_list))
    }
}

async fn render_page(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(Html::with(|_| html! { p { "hello" } }))
        .render()
        .await
}

async fn render_list(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(ListView::new().with_table(Table::new().with_column(Lc::n("Name"))))
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

// **< Automatic script injection (BeforeRenderBody) >**********************************************

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

// **< List loading styles >************************************************************************

#[pagetop::test]
async fn pages_with_a_list_embed_the_list_loading_styles_and_others_do_not() {
    let app = web::test::init_router(Application::prepare(&TestApp).await.test());

    let req = web::test::TestRequest::get().uri("/list").to_request();
    let body = web::test::read_body_text(web::test::send_request(&app, req).await).await;
    // HTMX sets `htmx-request` on the element named by `hx-indicator`: the list.
    assert!(
        body.contains(".list-view.htmx-request{opacity:0.5}"),
        "{body}"
    );

    let req = web::test::TestRequest::get().uri("/page").to_request();
    let body = web::test::read_body_text(web::test::send_request(&app, req).await).await;
    assert!(!body.contains("htmx-request"), "{body}");
}
