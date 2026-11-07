// The test email page of the administration panel. `config/common.toml` keeps the emails in
// memory. Nothing in this binary manages permissions, so `grant()` plays the authentication
// extension: it grants every permission to the requests sent with the `x-grant` header.
//
// The scenarios run one after another in a single test: with the `queue` feature, starting the
// application installs the queue migrations, and starting it from parallel tests would install
// them at the same time on the same in-memory database.

use pagetop::prelude::*;

struct TestApp;

#[async_trait]
impl Extension for TestApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_mail::Mail]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![action::auth::CheckPermission::new(grant)]
    }
}

fn grant(request: &HttpRequest, _: PermissionRef, granted: &mut bool) {
    if request.headers().contains_key("x-grant") {
        *granted = true;
    }
}

async fn app() -> Router {
    web::test::init_router(Application::prepare(&TestApp).await.test())
}

async fn send(app: &Router, request: web::test::TestRequest) -> (u16, String, String) {
    let response = web::test::send_request(app, request.to_request()).await;
    let status = response.status().as_u16();
    let header = |name| {
        response
            .headers()
            .get(name)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default()
            .to_owned()
    };
    let (location, cookie) = (header("location"), header("set-cookie"));
    let _ = web::test::read_body_text(response).await;
    (status, location, cookie)
}

fn post(to: &str) -> web::test::TestRequest {
    web::test::TestRequest::post()
        .uri("/admin/config/mail")
        .header("x-grant", "yes")
        .form(format!("to={to}"))
}

#[pagetop::test]
async fn test_email_page() {
    let app = app().await;
    the_page_and_its_permission_are_declared();
    the_page_shows_the_settings_without_credentials_and_the_form(&app).await;
    without_the_permission_the_page_is_denied(&app).await;
    a_test_email_is_sent_and_reported(&app).await;
    an_invalid_address_sends_nothing(&app).await;
}

fn the_page_and_its_permission_are_declared() {
    let page = AdminRegistry::global().page("config/mail").unwrap();
    assert_eq!(page.section(), "config");
    assert_eq!(page.permission().key(), "mail:send_test");
    assert!(PermissionRegistry::global().has_key("mail:send_test"));
}

async fn the_page_shows_the_settings_without_credentials_and_the_form(app: &Router) {
    let request = web::test::TestRequest::get()
        .uri("/admin/config/mail")
        .header("x-grant", "yes")
        .to_request();
    let response = web::test::send_request(app, request).await;
    assert_eq!(response.status().as_u16(), 200);
    let html = web::test::read_body_text(response).await;
    assert!(
        html.contains("PageTop Tests &lt;tests@example.com&gt;"),
        "{html}"
    );
    assert!(html.contains(r#"id="mail-test-form""#), "{html}");
    assert!(html.contains(r#"action="/admin/config/mail""#), "{html}");
    assert!(html.contains(r#"name="to""#), "{html}");
}

async fn without_the_permission_the_page_is_denied(app: &Router) {
    let get = web::test::TestRequest::get().uri("/admin/config/mail");
    assert_eq!(send(app, get).await.0, 403);
    let post = web::test::TestRequest::post()
        .uri("/admin/config/mail")
        .form("to=nobody@example.com");
    assert_eq!(send(app, post).await.0, 403);
    assert!(pagetop_mail::outbox::take_to("nobody@example.com").is_empty());
}

async fn a_test_email_is_sent_and_reported(app: &Router) {
    let (status, location, cookie) = send(app, post("admin@example.com")).await;
    assert_eq!(status, 303);
    assert_eq!(location, "/admin/config/mail");
    // An information message for the next page.
    assert!(
        cookie.starts_with("pagetop_flash=iTest%20email%20sent%20to%20admin%40example.com"),
        "{cookie}"
    );

    let sent = pagetop_mail::outbox::take_to("admin@example.com");
    assert_eq!(sent.len(), 1);
    let subject = sent[0].email().subject();
    assert!(subject.contains("PageTop"), "{subject}");
}

async fn an_invalid_address_sends_nothing(app: &Router) {
    let (status, location, cookie) = send(app, post("not-an-address")).await;
    assert_eq!(status, 303);
    assert_eq!(location, "/admin/config/mail");
    // An error message for the next page.
    assert!(cookie.starts_with("pagetop_flash=e"), "{cookie}");
    assert!(pagetop_mail::outbox::take_to("not-an-address").is_empty());
}
