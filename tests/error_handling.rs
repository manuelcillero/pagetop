use pagetop::prelude::*;

// **< CatchPanicLayer >****************************************************************************

struct PanicExtension;

#[async_trait]
impl Extension for PanicExtension {
    fn configure_router(&self, router: Router) -> Router {
        router.route("/boom", web::get(boom))
    }
}

async fn boom() -> Result<Markup, ErrorPage> {
    panic!("boom")
}

#[pagetop::test]
async fn panic_in_handler_returns_minimal_500_page_instead_of_crashing() {
    let app = web::test::init_router(Application::prepare(&PanicExtension).await.test());

    let req = web::test::TestRequest::get().uri("/boom").to_request();
    let resp = web::test::send_request(&app, req).await;

    assert_eq!(resp.status(), web::http::StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        resp.headers().get(web::http::header::CONTENT_TYPE).unwrap(),
        "text/html; charset=utf-8"
    );

    let body = web::test::read_body_text(resp).await;
    assert!(body.contains("An unexpected error has occurred"));
}

// **< ErrorPage::NotFound >************************************************************************

// `EXTENSIONS` es un `OnceLock` global (`core/extension/all.rs`): se inicializa una sola vez por
// binario de test. Todos los tests de este fichero comparten la misma extensión raíz
// (`PanicExtension`) para que el orden de ejecución en paralelo no cambie qué rutas quedan
// registradas.
#[pagetop::test]
async fn unknown_route_returns_themed_404_page() {
    let app = web::test::init_router(Application::prepare(&PanicExtension).await.test());

    let req = web::test::TestRequest::get()
        .uri("/does-not-exist")
        .to_request();
    let resp = web::test::send_request(&app, req).await;

    assert_eq!(resp.status(), web::http::StatusCode::NOT_FOUND);

    let body = web::test::read_body_text(resp).await;
    assert!(body.contains("<html"));
}
