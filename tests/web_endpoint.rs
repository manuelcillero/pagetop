// Verifies that a route declared with `Endpoint` is protected by its access when it is requested
// and that the links to it are only shown to whoever has that access.

use pagetop::prelude::*;

struct Granted;

impl Permission for Granted {
    fn key(&self) -> CowStr {
        "test:granted".into()
    }
}

struct Denied;

impl Permission for Denied {
    fn key(&self) -> CowStr {
        "test:denied".into()
    }
}

// Grants `test:granted` to everybody, as an authentication extension would.
fn grant(_: &HttpRequest, perm: PermissionRef, granted: &mut bool) {
    if perm.key() == "test:granted" {
        *granted = true;
    }
}

#[derive(Clone, Copy)]
enum TestRoute {
    Open,
    Signed,
    Allowed,
    Forbidden,
    Report(&'static str),
    NoteView,
    NoteSave,
    AllowedItem,
    ForbiddenItem,
}

impl Endpoint for TestRoute {
    fn path(&self) -> CowStr {
        match self {
            Self::Open => "/open".into(),
            Self::Signed => "/signed".into(),
            Self::Allowed => "/allowed".into(),
            Self::Forbidden => "/forbidden".into(),
            Self::Report(slug) => format!("/reports/{slug}").into(),
            Self::NoteView | Self::NoteSave => "/notes".into(),
            Self::AllowedItem => "/items/{id}/edit".into(),
            Self::ForbiddenItem => "/secrets/{id}/edit".into(),
        }
    }

    fn access(&self) -> Access {
        match self {
            Self::Open => Access::Public,
            Self::Signed => Access::Authenticated,
            Self::Allowed | Self::Report(_) | Self::NoteView | Self::AllowedItem => {
                Access::Permission(&Granted)
            }
            Self::Forbidden | Self::NoteSave | Self::ForbiddenItem => Access::Permission(&Denied),
        }
    }
}

// Signs in the requests that carry an `x-user` header.
async fn sign_in(mut request: web::Request, next: web::middleware::Next) -> Response {
    if request.headers().contains_key("x-user") {
        request
            .extensions_mut()
            .insert(CurrentUser::authenticated(1, "Test user"));
    }
    next.run(request).await
}

async fn ok() -> &'static str {
    "ok"
}

async fn home(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(
            Nav::new()
                .with_item(nav::Item::link(Lc::n("nav-open"), TestRoute::Open))
                .with_item(nav::Item::link(Lc::n("nav-signed"), TestRoute::Signed))
                .with_item(nav::Item::link(Lc::n("nav-allowed"), TestRoute::Allowed))
                .with_item(nav::Item::link(
                    Lc::n("nav-forbidden"),
                    TestRoute::Forbidden,
                ))
                .with_item(nav::Item::link(
                    Lc::n("nav-report"),
                    TestRoute::Report("sales"),
                ))
                .with_item(nav::Item::link(Lc::n("nav-literal"), "/literal"))
                .with_item(nav::Item::label(Lc::n("nav-label")))
                .with_item(nav::Item::dropdown(
                    Dropdown::new()
                        .with_title(Lc::n("dropdown-empty"))
                        .with_item(dropdown::Item::link(Lc::n("x"), TestRoute::Forbidden)),
                ))
                .with_item(nav::Item::dropdown(
                    Dropdown::new()
                        .with_title(Lc::n("dropdown-partial"))
                        .with_item(dropdown::Item::link(
                            Lc::n("in-allowed"),
                            TestRoute::Allowed,
                        ))
                        .with_item(dropdown::Item::link(
                            Lc::n("in-forbidden"),
                            TestRoute::Forbidden,
                        ))
                        .with_item(dropdown::Item::button(Lc::n("in-button"))),
                )),
        )
        .with_child(
            Nav::new()
                .with_id("nav-all-hidden")
                .with_item(nav::Item::link(Lc::n("x"), TestRoute::Forbidden)),
        )
        .with_child(
            Dropdown::new()
                .with_title(Lc::n("standalone-empty"))
                .with_item(dropdown::Item::link(Lc::n("x"), TestRoute::Forbidden)),
        )
        .with_child(Button::anchor(Lc::n("button-allowed"), TestRoute::Allowed))
        .with_child(Button::anchor(
            Lc::n("button-forbidden"),
            TestRoute::Forbidden,
        ))
        .with_child(Button::anchor(
            Lc::n("item-allowed"),
            TestRoute::AllowedItem.to(7),
        ))
        .with_child(Button::anchor(
            Lc::n("item-forbidden"),
            TestRoute::ForbiddenItem.to(7),
        ))
        .with_child(Button::anchor(
            Lc::n("item-encoded"),
            TestRoute::AllowedItem.to("a b/c?d#e"),
        ))
        .with_child(Button::submit(Lc::n("button-submit")))
        .render()
        .await
}

struct Ext;

#[async_trait]
impl Extension for Ext {
    fn actions(&self) -> Vec<ActionBox> {
        actions![action::auth::CheckPermission::new(grant)]
    }

    fn configure_router(&self, routes: Routes) -> Routes {
        routes
            .public_route("/", web::get(home))
            .endpoint(TestRoute::Open, web::get(ok))
            .endpoint(TestRoute::Signed, web::get(ok))
            .endpoint(TestRoute::Allowed, web::get(ok))
            .endpoint(TestRoute::Forbidden, web::get(ok))
            .endpoint(TestRoute::Report("sales"), web::get(ok))
            // Reading is allowed, writing needs a permission that nobody has.
            .endpoint(TestRoute::NoteView, web::get(ok))
            .endpoint(TestRoute::NoteSave, web::post(ok))
            .endpoint(TestRoute::AllowedItem, web::get(ok))
            .endpoint(TestRoute::ForbiddenItem, web::get(ok))
    }

    fn configure_middleware(&self, layers: Layers) -> Layers {
        layers.layer(web::middleware::from_fn(sign_in))
    }
}

async fn send(request: web::test::TestRequest) -> Response {
    let app = Application::prepare(&Ext).await.test();
    web::test::send_request(&app, request.to_request()).await
}

async fn status_of(uri: &str) -> u16 {
    let response = send(web::test::TestRequest::get().uri(uri)).await;
    response.status().as_u16()
}

async fn home_page() -> String {
    let response = send(web::test::TestRequest::get().uri("/")).await;
    assert_eq!(response.status().as_u16(), 200);
    web::test::read_body_text(response).await
}

#[pagetop::test]
async fn the_routes_require_the_access_they_declare() {
    assert_eq!(status_of("/open").await, 200);
    assert_eq!(status_of("/allowed").await, 200);
    assert_eq!(status_of("/reports/sales").await, 200);
    assert_eq!(status_of("/items/7/edit").await, 200);
    assert_eq!(status_of("/signed").await, 403);
    assert_eq!(status_of("/forbidden").await, 403);
    assert_eq!(status_of("/secrets/7/edit").await, 403);
}

#[pagetop::test]
async fn each_method_of_a_route_can_require_its_own_access() {
    assert_eq!(status_of("/notes").await, 200);
    let save = send(web::test::TestRequest::post().uri("/notes")).await;
    assert_eq!(save.status().as_u16(), 403);
}

#[pagetop::test]
async fn links_are_only_shown_with_access_to_their_route() {
    let html = home_page().await;
    for shown in [
        "nav-open",
        "nav-allowed",
        "nav-report",
        r#"href="/reports/sales""#,
        "dropdown-partial",
        "in-allowed",
        "button-allowed",
    ] {
        assert!(html.contains(shown), "`{shown}` is missing in {html}");
    }
    for hidden in [
        "nav-signed",
        "nav-forbidden",
        "in-forbidden",
        "button-forbidden",
        "item-forbidden",
    ] {
        assert!(!html.contains(hidden), "`{hidden}` is shown in {html}");
    }
}

#[pagetop::test]
async fn links_for_signed_in_users_are_shown_after_signing_in() {
    assert_eq!(status_of("/signed").await, 403);
    let signed = send(
        web::test::TestRequest::get()
            .uri("/signed")
            .header("x-user", "1"),
    )
    .await;
    assert_eq!(signed.status().as_u16(), 200);

    let home = send(web::test::TestRequest::get().uri("/").header("x-user", "1")).await;
    let html = web::test::read_body_text(home).await;
    assert!(
        html.contains("nav-signed"),
        "`nav-signed` is missing in {html}"
    );
}

#[pagetop::test]
async fn elements_without_an_endpoint_are_not_affected() {
    let html = home_page().await;
    for shown in ["nav-literal", "nav-label", "in-button", "button-submit"] {
        assert!(html.contains(shown), "`{shown}` is missing in {html}");
    }
}

#[pagetop::test]
async fn menus_with_nothing_to_show_are_not_rendered() {
    let html = home_page().await;
    for hidden in ["dropdown-empty", "nav-all-hidden", "standalone-empty"] {
        assert!(!html.contains(hidden), "`{hidden}` is shown in {html}");
    }
}

#[pagetop::test]
async fn links_fill_the_parameter_of_their_route() {
    let html = home_page().await;
    assert!(html.contains(r#"href="/items/7/edit""#), "{html}");
    assert!(
        html.contains(r#"href="/items/a%20b%2Fc%3Fd%23e/edit""#),
        "{html}"
    );
}

#[pagetop::test]
async fn without_a_request_only_public_routes_are_accessible() {
    let cx = Context::default();
    assert!(Route::from(TestRoute::Open).is_accessible(&cx));
    assert!(!Route::from(TestRoute::Signed).is_accessible(&cx));
    assert!(!Route::from(TestRoute::Allowed).is_accessible(&cx));
    assert!(Route::from("/literal").is_accessible(&cx));
}

#[pagetop::test]
#[should_panic(expected = "needs its parameter")]
async fn linking_to_a_route_with_a_parameter_needs_its_value() {
    let _ = nav::Item::link(Lc::n("x"), TestRoute::AllowedItem);
}

#[pagetop::test]
#[should_panic(expected = "has no parameter")]
async fn filling_a_route_without_a_parameter_is_an_error() {
    let _ = TestRoute::Allowed.to(7);
}
