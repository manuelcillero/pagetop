use pagetop::prelude::*;

use web::http::{StatusCode, header};

struct FlashExtension;

#[async_trait]
impl Extension for FlashExtension {
    fn configure_router(&self, router: Router) -> Router {
        router
            .route("/save", web::post(save))
            .route(
                "/again",
                web::get(|| async { Redirect::see_other("/done") }),
            )
            .route("/done", web::get(done))
            .route("/file", web::get(|| async { "plain text" }))
    }
}

async fn save(request: HttpRequest) -> Response {
    let cx = Context::new(request);
    Redirect::see_other("/done")
        .with_message(&cx, MessageLevel::Info, Lc::n("Saved <b>today</b>"))
        .with_message(&cx, MessageLevel::Error, Lc::n("One; problem"))
}

async fn done(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(Html::with(|_| html! { p { "Done" } }))
        .render()
        .await
}

async fn send(request: web::test::TestRequest) -> Response {
    let app = web::test::init_router(Application::prepare(&FlashExtension).await.test());
    web::test::send_request(&app, request.to_request()).await
}

fn get(uri: &str, cookie: Option<&str>) -> web::test::TestRequest {
    let request = web::test::TestRequest::get().uri(uri);
    match cookie {
        Some(cookie) => request.header("cookie", cookie),
        None => request,
    }
}

fn set_cookie(response: &Response) -> Option<&str> {
    response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
}

// `name=value` of the flash cookie set by `response`.
fn flash_cookie(response: &Response) -> String {
    let cookie = set_cookie(response).expect("the response sets the flash cookie");
    assert!(cookie.starts_with("pagetop_flash="), "{cookie}");
    cookie.split(';').next().unwrap().to_owned()
}

#[pagetop::test]
async fn messages_are_shown_on_the_next_page_and_then_cleared() {
    let saved = send(web::test::TestRequest::post().uri("/save")).await;
    assert_eq!(saved.status(), StatusCode::SEE_OTHER);
    assert!(set_cookie(&saved).unwrap().contains("Max-Age=60"));
    let cookie = flash_cookie(&saved);

    let page = send(get("/done", Some(&cookie))).await;
    assert_eq!(
        set_cookie(&page),
        Some("pagetop_flash=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0")
    );
    let body = web::test::read_body_text(page).await;
    // Shown once, as plain text, before the content.
    let info = body
        .find("Saved &lt;b&gt;today&lt;/b&gt;")
        .expect("info message");
    assert!(body.contains("message message-error"));
    assert!(body.contains("One; problem"));
    assert!(info < body.find("Done").unwrap());
    assert_eq!(body.matches("message-info").count(), 1);

    // Without the cookie there is nothing to show or to clear.
    let page = send(get("/done", None)).await;
    assert_eq!(set_cookie(&page), None);
    assert!(
        !web::test::read_body_text(page)
            .await
            .contains("message-info")
    );
}

#[pagetop::test]
async fn messages_wait_for_a_page() {
    let cookie = flash_cookie(&send(web::test::TestRequest::post().uri("/save")).await);
    // Another redirect, or a response that is not a page, keeps the cookie untouched.
    assert_eq!(set_cookie(&send(get("/again", Some(&cookie))).await), None);
    assert_eq!(set_cookie(&send(get("/file", Some(&cookie))).await), None);
    // An error page shows them.
    let missing = send(get("/missing", Some(&cookie))).await;
    assert_eq!(missing.status(), StatusCode::NOT_FOUND);
    assert!(set_cookie(&missing).unwrap().contains("Max-Age=0"));
    assert!(
        web::test::read_body_text(missing)
            .await
            .contains("One; problem")
    );
}

#[pagetop::test]
async fn forged_cookies_only_show_escaped_text() {
    let cookie = "pagetop_flash=w%3Cscript%3Ealert(1)%3C%2Fscript%3E|xignored";
    let body = web::test::read_body_text(send(get("/done", Some(cookie))).await).await;
    assert!(body.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
    assert!(!body.contains("<script>alert"));
    assert!(!body.contains("ignored"));
}
