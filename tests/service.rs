use pagetop::prelude::*;

#[pagetop::test]
async fn homepage_returns_404() {
    let app = web::test::init_router(Application::new().test());

    let req = web::test::TestRequest::get().uri("/").to_request();
    let resp = web::test::send_request(&app, req).await;

    // Comprueba el acceso a la ruta de inicio.
    assert_eq!(resp.status(), web::http::StatusCode::OK);
}
