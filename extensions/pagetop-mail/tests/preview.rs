// `config/common.toml` enables the previews. The action registry is built once per test binary, so
// the previews declared here live in their own file.

use pagetop::prelude::*;
use pagetop_mail::prelude::*;

struct TestApp;

#[async_trait]
impl Extension for TestApp {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_mail::Mail]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![DeclareMailPreviews::new(declare_previews)]
    }
}

fn declare_previews(previews: &mut MailPreviews) {
    previews.add("test.welcome", Lc::n("Welcome email"), welcome);
}

fn welcome(cx: &Context) -> Email {
    Email::new()
        .with_to("Ana <ana@example.com>")
        .with_subject("Welcome")
        .with_html(html! { p { "Hello " a href="https://example.com/start" { "start" } } })
        .with_attachment(Attachment::new("terms.pdf", b"%PDF".to_vec()))
        .with_langid(cx)
}

async fn get(app: &Router, uri: &str) -> (web::http::StatusCode, String) {
    let req = web::test::TestRequest::get().uri(uri).to_request();
    let resp = web::test::send_request(app, req).await;
    let status = resp.status();
    (status, web::test::read_body_text(resp).await)
}

#[pagetop::test]
async fn previews_are_listed_and_rendered_without_sending() {
    let app = web::test::init_router(Application::prepare(&TestApp).await.test());

    let (status, body) = get(&app, "/mail/preview").await;
    assert_eq!(status, web::http::StatusCode::OK);
    assert!(body.contains(r#"href="/mail/preview/test.welcome""#));
    assert!(body.contains("Welcome email"));

    let (status, body) = get(&app, "/mail/preview/test.welcome").await;
    assert_eq!(status, web::http::StatusCode::OK);
    // Default sender applied, as when sending.
    assert!(body.contains("PageTop Tests &lt;tests@example.com&gt;"));
    assert!(body.contains("Ana &lt;ana@example.com&gt;"));
    // The HTML goes in an isolated iframe, already wrapped in the layout.
    assert!(body.contains("srcdoc=\"&lt;!DOCTYPE html&gt;"));
    assert!(body.contains(r#"sandbox="""#));
    // Generated plain text and attachments.
    assert!(body.contains("Hello start (https://example.com/start)"));
    assert!(body.contains("terms.pdf (application/pdf, 4 B)"));
    assert!(pagetop_mail::outbox::take_to("ana@example.com").is_empty());

    let (status, _) = get(&app, "/mail/preview/missing").await;
    assert_eq!(status, web::http::StatusCode::NOT_FOUND);
}
