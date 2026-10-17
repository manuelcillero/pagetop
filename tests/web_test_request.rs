use pagetop::prelude::*;

use serde::Deserialize;

// Every method of `web::test::TestRequest` reaches the route registered for it, and `form()` sends
// the fields as a browser does.

#[derive(Deserialize)]
struct Item {
    name: String,
}

async fn echo(web::Form(item): web::Form<Item>) -> String {
    item.name
}

struct Echo;

#[async_trait]
impl Extension for Echo {
    fn configure_router(&self, router: Router) -> Router {
        router.route(
            "/item",
            web::get(|| async { "get" })
                .post(echo)
                .put(echo)
                .patch(echo)
                .delete(|| async { "delete" }),
        )
    }
}

async fn send(request: web::test::TestRequest) -> (web::http::StatusCode, String) {
    let app = web::test::init_router(Application::prepare(&Echo).await.test());
    let response = web::test::send_request(&app, request.uri("/item").to_request()).await;
    let status = response.status();
    (status, web::test::read_body_text(response).await)
}

#[pagetop::test]
async fn get_and_delete_reach_their_routes() {
    let (status, body) = send(web::test::TestRequest::get()).await;
    assert_eq!((status, body.as_str()), (web::http::StatusCode::OK, "get"));

    let (status, body) = send(web::test::TestRequest::delete()).await;
    assert_eq!(
        (status, body.as_str()),
        (web::http::StatusCode::OK, "delete")
    );
}

#[pagetop::test]
async fn post_put_and_patch_send_the_form_fields() {
    for request in [
        web::test::TestRequest::post(),
        web::test::TestRequest::put(),
        web::test::TestRequest::patch(),
    ] {
        let (status, body) = send(request.form("name=Plant+lettuces")).await;
        assert_eq!(
            (status, body.as_str()),
            (web::http::StatusCode::OK, "Plant lettuces")
        );
    }
}

#[pagetop::test]
async fn a_form_request_without_fields_is_rejected() {
    // No `form()`: no body and no form content type, so the form extractor refuses it.
    let (status, _) = send(web::test::TestRequest::post()).await;
    assert!(status.is_client_error());
}
