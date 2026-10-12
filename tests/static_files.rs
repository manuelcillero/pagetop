use pagetop::prelude::*;

// Static assets of PageTop itself (`/pagetop/...`), always registered.
const ASSET: &str = "/pagetop/css/basic.min.css";

// Header added by the global middleware of the test extension.
const MARK: &str = "x-global-middleware";

// Paths seen by the global middleware, to check whether it ran independently of what ends up in
// the response headers.
static SEEN_PATHS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

// Whether the global middleware has seen `path`. The lock is released before the caller asserts,
// so that a failing test does not poison it for the rest of the tests running in parallel.
fn middleware_saw(path: &str) -> bool {
    SEEN_PATHS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .iter()
        .any(|seen| seen == path)
}

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
    SEEN_PATHS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .push(request.uri().path().to_owned());
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert(MARK, web::http::HeaderValue::from_static("1"));
    headers.append(
        web::http::header::SET_COOKIE,
        web::http::HeaderValue::from_static("a=1"),
    );
    headers.append(
        web::http::header::SET_COOKIE,
        web::http::HeaderValue::from_static("b=2"),
    );
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
async fn unknown_routes_go_through_global_middleware() {
    // The 404 page must see what the middleware adds to the request (e.g. the current user).
    let resp = get("/does-not-exist", None).await;
    assert_eq!(resp.status(), web::http::StatusCode::NOT_FOUND);
    assert!(middleware_saw("/does-not-exist"));
}

#[pagetop::test]
async fn error_pages_keep_headers_added_by_middleware() {
    // The rendered 404 page replaces the response, but keeps the middleware headers.
    let resp = get("/does-not-exist", None).await;
    let headers = resp.headers();
    assert_eq!(resp.status(), web::http::StatusCode::NOT_FOUND);
    assert!(headers.contains_key(MARK));
    assert_eq!(
        headers
            .get_all(web::http::header::SET_COOKIE)
            .iter()
            .count(),
        2
    );
    let content_type = headers.get(web::http::header::CONTENT_TYPE).unwrap();
    assert!(content_type.to_str().unwrap().starts_with("text/html"));
}

#[pagetop::test]
async fn static_files_skip_global_middleware() {
    let resp = get(ASSET, None).await;
    assert_eq!(resp.status(), web::http::StatusCode::OK);
    assert!(!resp.headers().contains_key(MARK));
    assert!(!middleware_saw(ASSET));
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
