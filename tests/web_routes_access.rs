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

// Offers the login form to an anonymous user without access, as an authentication extension would.
fn offer_login(_: &HttpRequest) -> Option<Response> {
    Some(Redirect::see_other("/login"))
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

struct Ext;

#[async_trait]
impl Extension for Ext {
    fn actions(&self) -> Vec<ActionBox> {
        actions![
            action::auth::CheckPermission::new(grant),
            action::auth::AnonymousDenied::new(offer_login),
        ]
    }

    fn configure_router(&self, routes: Routes) -> Routes {
        routes
            .public_route("/public", web::get(ok))
            .route("/account", Access::Authenticated, web::get(ok))
            .route("/granted", Access::Permission(&Granted), web::get(ok))
            .route("/denied", Access::Permission(&Denied), web::get(ok))
            // Reading is public, writing needs a permission that nobody has.
            .public_route("/notes", web::get(ok))
            .route("/notes", Access::Permission(&Denied), web::post(ok))
            // Their own fallbacks require the access too.
            .route(
                "/hooks",
                Access::Permission(&Denied),
                web::get(ok).fallback(ok),
            )
            .nest(
                "/api",
                Access::Permission(&Denied),
                Router::new().route("/data", web::get(ok)).fallback(ok),
            )
            .nest_service("/files", Access::Authenticated, web::ServeDir::new("tests"))
    }

    fn configure_middleware(&self, layers: Layers) -> Layers {
        layers.layer(web::middleware::from_fn(sign_in))
    }
}

async fn send(method: &str, uri: &str, user: bool) -> (u16, Option<String>) {
    let app = Application::prepare(&Ext).await.test();
    let request = if method == "POST" {
        web::test::TestRequest::post()
    } else {
        web::test::TestRequest::get()
    };
    let request = if user {
        request.header("x-user", "1")
    } else {
        request
    };
    let response = web::test::send_request(&app, request.uri(uri).to_request()).await;
    let location = response
        .headers()
        .get("location")
        .map(|value| value.to_str().unwrap().to_owned());
    (response.status().as_u16(), location)
}

#[pagetop::test]
async fn a_public_route_is_open_to_everybody() {
    assert_eq!(send("GET", "/public", false).await.0, 200);
    assert_eq!(send("GET", "/public", true).await.0, 200);
}

#[pagetop::test]
async fn an_anonymous_user_without_access_gets_the_response_of_the_action() {
    let login = (303, Some("/login".to_owned()));
    assert_eq!(send("GET", "/account", false).await, login);
    assert_eq!(send("GET", "/denied", false).await, login);
    // A granted permission does not need a session.
    assert_eq!(send("GET", "/granted", false).await.0, 200);
}

#[pagetop::test]
async fn an_authenticated_user_without_access_gets_access_denied() {
    assert_eq!(send("GET", "/account", true).await.0, 200);
    assert_eq!(send("GET", "/granted", true).await.0, 200);
    assert_eq!(send("GET", "/denied", true).await.0, 403);
}

#[pagetop::test]
async fn each_method_of_a_route_has_its_own_access() {
    assert_eq!(send("GET", "/notes", true).await.0, 200);
    assert_eq!(send("POST", "/notes", true).await.0, 403);
}

#[pagetop::test]
async fn the_fallback_of_a_route_requires_its_access() {
    assert_eq!(send("POST", "/hooks", true).await.0, 403);
    assert_eq!(send("GET", "/api/other", true).await.0, 403);
}

#[pagetop::test]
async fn nested_routers_and_services_share_the_access_of_the_prefix() {
    assert_eq!(send("GET", "/api/data", true).await.0, 403);
    assert_eq!(
        send("GET", "/files/web_routes_access.rs", true).await.0,
        200
    );
    assert_eq!(
        send("GET", "/files/web_routes_access.rs", false).await.0,
        303
    );
    // Routes that are not mounted are not affected.
    assert_eq!(send("GET", "/missing", true).await.0, 404);
}
