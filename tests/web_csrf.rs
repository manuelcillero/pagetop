use pagetop::prelude::*;

use web::http::StatusCode;

// Paths seen by the extension middleware: a rejected request must not reach it (it would load
// the session).
static SEEN_PATHS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

fn middleware_saw(path: &str) -> bool {
    SEEN_PATHS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .iter()
        .any(|seen| seen == path)
}

struct ActionExtension;

#[async_trait]
impl Extension for ActionExtension {
    fn configure_router(&self, routes: Routes) -> Routes {
        routes.public_route(
            "/action/{name}",
            web::get(|| async { "read" }).post(|| async { "done" }),
        )
    }

    fn configure_middleware(&self, layers: Layers) -> Layers {
        layers.layer(web::middleware::from_fn(record_path))
    }
}

async fn record_path(request: web::Request, next: web::middleware::Next) -> Response {
    SEEN_PATHS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .push(request.uri().path().to_owned());
    next.run(request).await
}

async fn send(request: web::test::TestRequest) -> Response {
    let app = web::test::init_router(Application::prepare(&ActionExtension).await.test());
    web::test::send_request(&app, request.to_request()).await
}

fn post(path: &str, headers: &[(&str, &str)]) -> web::test::TestRequest {
    headers.iter().fold(
        web::test::TestRequest::post().uri(path),
        |req, (name, value)| req.header(name, value),
    )
}

#[pagetop::test]
async fn same_origin_requests_are_allowed() {
    let resp = send(post("/action/same", &[("sec-fetch-site", "same-origin")])).await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(web::test::read_body_text(resp).await, "done");

    let origin = [("origin", "http://example.com"), ("host", "example.com")];
    assert_eq!(
        send(post("/action/old", &origin)).await.status(),
        StatusCode::OK
    );
}

#[pagetop::test]
async fn cross_origin_requests_are_rejected_before_the_middleware() {
    let resp = send(post("/action/cross", &[("sec-fetch-site", "cross-site")])).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    // The theme renders the 403 page.
    assert!(web::test::read_body_text(resp).await.contains("<html"));
    assert!(!middleware_saw("/action/cross"));

    let resp = send(post(
        "/action/subdomain",
        &[("sec-fetch-site", "same-site")],
    ))
    .await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    let origin = [("origin", "https://evil.example"), ("host", "example.com")];
    assert_eq!(
        send(post("/action/old-cross", &origin)).await.status(),
        StatusCode::FORBIDDEN
    );
}

#[pagetop::test]
async fn safe_methods_and_non_browser_clients_are_not_checked() {
    let req = web::test::TestRequest::get()
        .uri("/action/read")
        .header("sec-fetch-site", "cross-site");
    assert_eq!(send(req).await.status(), StatusCode::OK);
    // No Sec-Fetch-Site nor Origin: not a browser (curl, another server, a webhook).
    assert_eq!(
        send(post("/action/curl", &[])).await.status(),
        StatusCode::OK
    );
}

#[pagetop::test]
async fn trusted_origins_are_allowed() {
    // `config/test.toml` trusts "https://Trusted.example/", normalized when loaded.
    let headers = [
        ("sec-fetch-site", "cross-site"),
        ("origin", "https://trusted.example"),
    ];
    assert_eq!(
        send(post("/action/trusted", &headers)).await.status(),
        StatusCode::OK
    );
    let headers = [
        ("origin", "https://trusted.example"),
        ("host", "example.com"),
    ];
    assert_eq!(
        send(post("/action/trusted-old", &headers)).await.status(),
        StatusCode::OK
    );
}
