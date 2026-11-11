use pagetop::prelude::*;

use web::http::{StatusCode, header};

struct ListExtension;

#[async_trait]
impl Extension for ListExtension {
    fn configure_router(&self, routes: Routes) -> Routes {
        routes
            .public_route(
                "/items",
                web::remember_query(web::get(list).post(|| async { "posted" })),
            )
            .public_route("/failing", web::remember_query(web::get(failing)))
            .public_route("/picky", web::remember_query(web::get(picky)))
            .public_route("/{*rest}", web::remember_query(web::get(list)))
            .route(
                "/private",
                Access::Authenticated,
                web::remember_query(web::get(list)),
            )
            .nest(
                "/nested",
                Access::Public,
                Router::new().route("/items", web::remember_query(web::get(list))),
            )
    }
}

async fn list(request: HttpRequest) -> String {
    format!("query: {}", request.query_string())
}

async fn failing(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Err(ErrorPage::NotFound(request))
}

// Accepts any query but `q=stale`, as a list does with parameters from an older release.
async fn picky(request: HttpRequest) -> Result<String, ErrorPage> {
    match request.query_string() {
        "q=stale" => Err(ErrorPage::BadRequest(request)),
        query => Ok(format!("query: {query}")),
    }
}

async fn send(request: web::test::TestRequest) -> Response {
    send_edited(request, |_| {}).await
}

// Sends `request` after `edit` changes its headers, for what `TestRequest` cannot express.
async fn send_edited(
    request: web::test::TestRequest,
    edit: impl FnOnce(&mut web::http::HeaderMap),
) -> Response {
    let mut request = request.to_request();
    edit(request.headers_mut());
    let app = web::test::init_router(Application::prepare(&ListExtension).await.test());
    web::test::send_request(&app, request).await
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

// `name=value` of the cookie set by `response`.
fn list_cookie(response: &Response) -> String {
    let cookie = set_cookie(response).expect("the response sets the list cookie");
    assert!(cookie.starts_with("pagetop_list_"), "{cookie}");
    cookie.split(';').next().unwrap().to_owned()
}

fn location(response: &Response) -> &str {
    response
        .headers()
        .get(header::LOCATION)
        .and_then(|value| value.to_str().ok())
        .expect("redirect location")
}

#[pagetop::test]
async fn a_list_remembers_its_state_and_restores_it_when_requested_without_it() {
    let filtered = send(get("/items?q=acer&page=2&lang=es", None)).await;
    assert_eq!(filtered.status(), StatusCode::OK);
    let cookie = set_cookie(&filtered).unwrap();
    assert!(
        cookie.contains("=q=acer&page=2;"),
        "lang is not saved: {cookie}"
    );
    assert!(cookie.contains("Path=/items;"), "{cookie}");
    assert!(cookie.contains("HttpOnly"), "{cookie}");
    assert!(cookie.contains("; Secure"), "{cookie}");
    assert!(!cookie.contains("Max-Age"), "a session cookie: {cookie}");
    let cookie = list_cookie(&filtered);

    let back = send(get("/items", Some(&cookie))).await;
    assert_eq!(back.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&back), "/items?q=acer&page=2");

    // The language of the current request is kept.
    let back = send(get("/items?lang=en", Some(&cookie))).await;
    assert_eq!(location(&back), "/items?q=acer&page=2&lang=en");
}

#[pagetop::test]
async fn without_a_saved_state_the_list_is_served_as_is() {
    let response = send(get("/items", None)).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(set_cookie(&response).is_none());
    assert_eq!(web::test::read_body_text(response).await, "query: ");
}

#[pagetop::test]
async fn the_same_state_is_not_saved_again() {
    let cookie = list_cookie(&send(get("/items?q=acer", None)).await);
    let again = send(get("/items?q=acer", Some(&cookie))).await;
    assert_eq!(again.status(), StatusCode::OK);
    assert!(set_cookie(&again).is_none());
}

#[pagetop::test]
async fn reset_forgets_the_state() {
    let cookie = list_cookie(&send(get("/items?q=acer", None)).await);

    let reset = send(get("/items?reset&lang=es", Some(&cookie))).await;
    assert_eq!(reset.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&reset), "/items?lang=es");
    let expired = set_cookie(&reset).unwrap();
    assert!(expired.starts_with(&cookie[..cookie.find('=').unwrap() + 1]));
    assert!(expired.contains("Max-Age=0"), "{expired}");
}

#[pagetop::test]
async fn an_empty_filter_saves_nothing_and_forgets_the_state() {
    // Submitting the filter with every field empty is the same as not filtering.
    let blank = send(get("/items?q=&status=&lang=es", None)).await;
    assert_eq!(blank.status(), StatusCode::OK);
    assert!(set_cookie(&blank).is_none());

    let cookie = list_cookie(&send(get("/items?q=acer", None)).await);
    let blank = send(get("/items?q=&status=", Some(&cookie))).await;
    assert_eq!(blank.status(), StatusCode::OK);
    let expired = set_cookie(&blank).expect("the saved state is forgotten");
    assert!(expired.starts_with(&cookie[..cookie.find('=').unwrap() + 1]));
    assert!(expired.contains("Max-Age=0"), "{expired}");
}

#[pagetop::test]
async fn htmx_requests_without_parameters_are_redirected_too() {
    // HTMX follows the redirection, so a boosted back link also shows the remembered state.
    let cookie = list_cookie(&send(get("/items?q=acer", None)).await);
    let request = get("/items", Some(&cookie)).header("hx-request", "true");
    let response = send(request).await;
    assert_eq!(response.status(), StatusCode::SEE_OTHER);
    assert_eq!(location(&response), "/items?q=acer");
}

#[pagetop::test]
async fn failed_responses_and_other_methods_save_nothing() {
    let failed = send(get("/failing?q=acer", None)).await;
    assert_eq!(failed.status(), StatusCode::NOT_FOUND);
    assert!(set_cookie(&failed).is_none());

    let posted = send(web::test::TestRequest::post().uri("/items?q=acer")).await;
    assert_eq!(posted.status(), StatusCode::OK);
    assert!(set_cookie(&posted).is_none());
}

#[pagetop::test]
async fn a_saved_state_that_no_longer_gives_a_page_is_forgotten() {
    let cookie = list_cookie(&send(get("/picky?q=fine", None)).await);
    let name = &cookie[..cookie.find('=').unwrap()];
    let stale = format!("{name}=q=stale");

    // Coming back restores it, and the list now refuses it: the state is dropped, so the next visit
    // does not fail again.
    let back = send(get("/picky", Some(&stale))).await;
    assert_eq!(location(&back), "/picky?q=stale");
    let refused = send(get("/picky?q=stale", Some(&stale))).await;
    assert_eq!(refused.status(), StatusCode::BAD_REQUEST);
    let expired = set_cookie(&refused).expect("the stale state is forgotten");
    assert!(expired.contains("Max-Age=0"), "{expired}");

    // A failing request with other parameters keeps the saved state.
    let refused = send(get("/picky?q=stale", Some(&cookie))).await;
    assert_eq!(refused.status(), StatusCode::BAD_REQUEST);
    assert!(set_cookie(&refused).is_none());
}

#[pagetop::test]
async fn states_that_cannot_go_in_a_cookie_are_not_saved() {
    let long = format!("/items?q={}", "a".repeat(1100));
    assert!(set_cookie(&send(get(&long, None)).await).is_none());
    assert!(set_cookie(&send(get("/items?q=a,b", None)).await).is_none());
}

#[pagetop::test]
async fn a_tampered_cookie_is_ignored() {
    let cookie = list_cookie(&send(get("/items?q=acer", None)).await);
    let name = &cookie[..cookie.find('=').unwrap()];
    let tampered = format!("{name}=q=\"x\"");
    let response = send(get("/items", Some(&tampered))).await;
    assert_eq!(response.status(), StatusCode::OK);
    let expired = set_cookie(&response).expect("the invalid cookie is removed");
    assert!(expired.contains("Max-Age=0"), "{expired}");
}

#[pagetop::test]
async fn a_tampered_cookie_with_no_usable_state_is_forgotten_without_redirecting() {
    let cookie = list_cookie(&send(get("/items?q=acer", None)).await);
    let name = &cookie[..cookie.find('=').unwrap()];
    // Redirecting with any of these would land on the same list again, forever.
    for value in ["&", "lang=es", "reset", "q=&status="] {
        let tampered = format!("{name}={value}");
        let response = send(get("/items", Some(&tampered))).await;
        assert_eq!(response.status(), StatusCode::OK, "{value}");
        let expired = set_cookie(&response).expect("the useless cookie is removed");
        assert!(expired.contains("Max-Age=0"), "{value}: {expired}");
    }
    // Saved `lang` is never replayed: the one of the current request wins.
    let tampered = format!("{name}=q=a&lang=fr");
    let response = send(get("/items?lang=es", Some(&tampered))).await;
    assert_eq!(location(&response), "/items?q=a&lang=es");
}

#[pagetop::test]
async fn links_from_other_sites_cannot_change_the_saved_state() {
    // Without this, a page elsewhere could link to `/items?q=...` and fix what the list shows to
    // whoever follows the link, every time they come back to it.
    for site in ["cross-site", "same-site"] {
        let foreign = get("/items?q=planted", None).header("sec-fetch-site", site);
        let response = send(foreign).await;
        assert_eq!(response.status(), StatusCode::OK, "{site}");
        assert!(set_cookie(&response).is_none(), "{site}");

        let cookie = list_cookie(&send(get("/items?q=acer", None)).await);
        for uri in ["/items?reset", "/items?q=", "/items?q=other"] {
            let foreign = get(uri, Some(&cookie)).header("sec-fetch-site", site);
            assert!(set_cookie(&send(foreign).await).is_none(), "{site} {uri}");
        }
        // Restoring only reads the state, so it works whatever the origin of the link.
        let foreign = get("/items", Some(&cookie)).header("sec-fetch-site", site);
        assert_eq!(location(&send(foreign).await), "/items?q=acer", "{site}");
    }
    // Requests from the application itself, or typed by hand, do change it.
    for site in ["same-origin", "none"] {
        let own = get("/items?q=acer", None).header("sec-fetch-site", site);
        assert!(set_cookie(&send(own).await).is_some(), "{site}");
    }
}

#[pagetop::test]
async fn the_access_of_the_route_is_checked_before_the_saved_state() {
    // An anonymous visitor with a cookie for a protected list gets neither a redirection that
    // reveals the saved state nor a new cookie: access is checked first.
    let cookie = "pagetop_list_c36367b4cdbc13dd=q=secret"; // The cookie name of `/private`.
    for uri in ["/private", "/private?q=other", "/private?reset"] {
        let response = send(get(uri, Some(cookie))).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN, "{uri}");
        assert!(set_cookie(&response).is_none(), "{uri}");
        assert!(response.headers().get(header::LOCATION).is_none(), "{uri}");
    }
}

#[pagetop::test]
async fn a_path_that_a_browser_would_read_as_another_site_is_left_alone() {
    // A list mounted on a catch-all route also matches `//other.example`; redirecting there would
    // send the browser to another site.
    let response = send(get("//other.example/?q=a", None)).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(set_cookie(&response).is_none());
}

#[pagetop::test]
async fn each_list_has_its_own_cookie_with_its_full_path() {
    let top = list_cookie(&send(get("/items?q=a", None)).await);
    let nested = send(get("/nested/items?q=b", None)).await;
    assert!(set_cookie(&nested).unwrap().contains("Path=/nested/items;"));
    let nested = list_cookie(&nested);
    assert_ne!(
        top.split('=').next(),
        nested.split('=').next(),
        "different lists, different cookies"
    );

    let back = send(get("/nested/items", Some(&nested))).await;
    assert_eq!(location(&back), "/nested/items?q=b");

    // The state of one list is never restored in another.
    let other = send(get("/items", Some(&nested))).await;
    assert_eq!(other.status(), StatusCode::OK);
    assert!(other.headers().get(header::LOCATION).is_none());
}

#[pagetop::test]
async fn the_cookie_is_found_in_any_cookie_header() {
    // With HTTP/2 the browser may split the cookies into several `cookie` headers.
    let cookie = list_cookie(&send(get("/items?q=acer", None)).await);
    let response = send_edited(get("/items", Some("theme=dark")), |headers| {
        headers.append(header::COOKIE, cookie.parse().unwrap());
    })
    .await;
    assert_eq!(location(&response), "/items?q=acer");
}
