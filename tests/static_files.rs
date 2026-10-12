use pagetop::prelude::*;

// Static assets of PageTop itself (`/pagetop/...`), always registered.
const ASSET: &str = "/pagetop/css/basic.min.css";

// Header added by the global middleware of the test extension.
const MARK: &str = "x-global-middleware";

// `EXTENSIONS` is a global `OnceLock`: all tests in this file share the same root extension.
struct MarkingExtension;

#[async_trait]
impl Extension for MarkingExtension {
    fn configure_router(&self, router: Router) -> Router {
        router.route("/page", web::get(|| async { "page" }))
    }

    fn configure_middleware(&self, router: Router) -> Router {
        router.layer(web::middleware::from_fn(mark_response))
    }
}

async fn mark_response(request: web::Request, next: web::middleware::Next) -> Response {
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(MARK, web::http::HeaderValue::from_static("1"));
    response
}

async fn get(uri: &str, if_none_match: Option<&str>) -> Response {
    let app = web::test::init_router(Application::prepare(&MarkingExtension).await.test());
    let mut req = web::test::TestRequest::get().uri(uri);
    if let Some(etag) = if_none_match {
        req = req.header("if-none-match", etag);
    }
    web::test::send_request(&app, req.to_request()).await
}

#[pagetop::test]
async fn routes_go_through_global_middleware() {
    let resp = get("/page", None).await;
    assert_eq!(resp.status(), web::http::StatusCode::OK);
    assert!(resp.headers().contains_key(MARK));
}

#[pagetop::test]
async fn static_files_skip_global_middleware() {
    let resp = get(ASSET, None).await;
    assert_eq!(resp.status(), web::http::StatusCode::OK);
    assert!(!resp.headers().contains_key(MARK));
}

#[pagetop::test]
async fn embedded_static_files_are_revalidated_with_etag() {
    let resp = get(ASSET, None).await;
    let headers = resp.headers();
    assert_eq!(
        headers.get(web::http::header::CACHE_CONTROL).unwrap(),
        "no-cache"
    );
    let etag = headers
        .get(web::http::header::ETAG)
        .expect("embedded asset without ETag")
        .to_str()
        .unwrap()
        .to_owned();

    // Same content: 304 without body, with the same validators.
    let resp = get(ASSET, Some(&etag)).await;
    assert_eq!(resp.status(), web::http::StatusCode::NOT_MODIFIED);
    assert_eq!(
        resp.headers().get(web::http::header::ETAG).unwrap(),
        etag.as_str()
    );
    assert!(web::test::read_body_text(resp).await.is_empty());

    // Weak comparison inside a list of tags.
    let resp = get(ASSET, Some(&format!("\"other\", W/{etag}"))).await;
    assert_eq!(resp.status(), web::http::StatusCode::NOT_MODIFIED);

    // A different tag gets the full content.
    let resp = get(ASSET, Some("\"other\"")).await;
    assert_eq!(resp.status(), web::http::StatusCode::OK);
    assert!(!web::test::read_body_text(resp).await.is_empty());
}
