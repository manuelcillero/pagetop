use pagetop::prelude::*;

#[pagetop::test]
async fn homepage_returns_404() {
    let app = service::test::init_service(Application::new().test()).await;

    let req = service::test::TestRequest::get().uri("/").to_request();
    let resp = service::test::call_service(&app, req).await;

    // Comprueba el acceso a la ruta de inicio.
    // assert_eq!(resp.status(), service::http::StatusCode::OK);

    // Sin ruta de inicio se obtiene error 404, pero el test funciona.
    assert_eq!(resp.status(), service::http::StatusCode::NOT_FOUND);
}
