use pagetop::prelude::*;
use pagetop_htmx::prelude::*;

struct TestApp;

#[async_trait]
impl Extension for TestApp {
    fn configure_router(&self, router: Router) -> Router {
        router.route("/echo", web::get(echo_request))
    }
}

// Reports every `HtmxRequestExt` value as JSON, so a single route can back every test in this file
// without needing a dedicated handler per header.
async fn echo_request(request: HttpRequest) -> String {
    serde_json::json!({
        "is_htmx": request.is_htmx(),
        "is_boosted": request.is_boosted(),
        "is_history_restore": request.is_history_restore(),
        "current_url": request.hx_current_url(),
        "target": request.hx_target(),
        "trigger_id": request.hx_trigger_id(),
        "trigger_name": request.hx_trigger_name(),
        "prompt": request.hx_prompt(),
    })
    .to_string()
}

async fn echo(app: &Router, headers: &[(&str, &str)]) -> serde_json::Value {
    let mut req = web::test::TestRequest::get().uri("/echo");
    for (name, value) in headers {
        req = req.header(*name, *value);
    }
    let resp = web::test::send_request(app, req.to_request()).await;
    let body = web::test::read_body_text(resp).await;
    serde_json::from_str(&body).unwrap()
}

// All tests in this file share the same root extension (`TestApp`), since `EXTENSIONS` is a global
// `OnceLock` initialized only once per test binary (see `core/extension/all.rs`).

// **< is_htmx() >**********************************************************************************

#[pagetop::test]
async fn is_htmx_is_true_only_when_hx_request_is_exactly_true() {
    let app = web::test::init_router(Application::prepare(&TestApp).await.test());

    let with_header = echo(&app, &[("hx-request", "true")]).await;
    assert_eq!(with_header["is_htmx"], true);

    let without_header = echo(&app, &[]).await;
    assert_eq!(without_header["is_htmx"], false);

    // A stray/incorrect value must not be treated as a truthy HTMX request.
    let wrong_value = echo(&app, &[("hx-request", "false")]).await;
    assert_eq!(wrong_value["is_htmx"], false);
}

// **< is_boosted() >*******************************************************************************

#[pagetop::test]
async fn is_boosted_reflects_the_hx_boosted_header() {
    let app = web::test::init_router(Application::prepare(&TestApp).await.test());

    let boosted = echo(&app, &[("hx-boosted", "true")]).await;
    assert_eq!(boosted["is_boosted"], true);

    let not_boosted = echo(&app, &[]).await;
    assert_eq!(not_boosted["is_boosted"], false);
}

// **< is_history_restore() >***********************************************************************

#[pagetop::test]
async fn is_history_restore_reflects_the_hx_history_restore_request_header() {
    let app = web::test::init_router(Application::prepare(&TestApp).await.test());

    let restoring = echo(&app, &[("hx-history-restore-request", "true")]).await;
    assert_eq!(restoring["is_history_restore"], true);

    let not_restoring = echo(&app, &[]).await;
    assert_eq!(not_restoring["is_history_restore"], false);
}

// **< hx_current_url() / hx_target() / hx_trigger_id() / hx_trigger_name() / hx_prompt() >*********

#[pagetop::test]
async fn hx_current_url_reads_the_hx_current_url_header_when_present() {
    let app = web::test::init_router(Application::prepare(&TestApp).await.test());

    let with_url = echo(&app, &[("hx-current-url", "/admin/users?page=2")]).await;
    assert_eq!(with_url["current_url"], "/admin/users?page=2");

    let without_url = echo(&app, &[]).await;
    assert!(without_url["current_url"].is_null());
}

#[pagetop::test]
async fn hx_target_reads_the_hx_target_header_when_present() {
    let app = web::test::init_router(Application::prepare(&TestApp).await.test());

    let with_target = echo(&app, &[("hx-target", "user-table")]).await;
    assert_eq!(with_target["target"], "user-table");

    let without_target = echo(&app, &[]).await;
    assert!(without_target["target"].is_null());
}

#[pagetop::test]
async fn hx_trigger_id_reads_the_hx_trigger_header_when_present() {
    let app = web::test::init_router(Application::prepare(&TestApp).await.test());

    let with_trigger = echo(&app, &[("hx-trigger", "save-button")]).await;
    assert_eq!(with_trigger["trigger_id"], "save-button");

    let without_trigger = echo(&app, &[]).await;
    assert!(without_trigger["trigger_id"].is_null());
}

#[pagetop::test]
async fn hx_trigger_name_reads_the_hx_trigger_name_header_when_present() {
    let app = web::test::init_router(Application::prepare(&TestApp).await.test());

    let with_name = echo(&app, &[("hx-trigger-name", "email")]).await;
    assert_eq!(with_name["trigger_name"], "email");

    let without_name = echo(&app, &[]).await;
    assert!(without_name["trigger_name"].is_null());
}

#[pagetop::test]
async fn hx_prompt_reads_the_hx_prompt_header_when_present() {
    let app = web::test::init_router(Application::prepare(&TestApp).await.test());

    let with_prompt = echo(&app, &[("hx-prompt", "Are you sure?")]).await;
    assert_eq!(with_prompt["prompt"], "Are you sure?");

    let without_prompt = echo(&app, &[]).await;
    assert!(without_prompt["prompt"].is_null());
}

// **< A realistic combined request >***************************************************************

#[pagetop::test]
async fn a_realistic_htmx_request_reports_all_fields_consistently() {
    // Simulates a table sort click: a boosted-free HTMX request triggered by a link with an `id`,
    // targeting the table wrapper.
    let app = web::test::init_router(Application::prepare(&TestApp).await.test());

    let result = echo(
        &app,
        &[
            ("hx-request", "true"),
            ("hx-target", "user-table"),
            ("hx-trigger", "sort-username"),
            ("hx-current-url", "/admin/users"),
        ],
    )
    .await;

    assert_eq!(result["is_htmx"], true);
    assert_eq!(result["is_boosted"], false);
    assert_eq!(result["is_history_restore"], false);
    assert_eq!(result["target"], "user-table");
    assert_eq!(result["trigger_id"], "sort-username");
    assert_eq!(result["current_url"], "/admin/users");
    assert!(result["trigger_name"].is_null());
    assert!(result["prompt"].is_null());
}
