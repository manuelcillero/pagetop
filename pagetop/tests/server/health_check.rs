use pagetop::prelude::*;

struct HealthCheck;

impl ExtensionTrait for HealthCheck {}

#[pagetop::test]
async fn health_check_works() {
    let app = service::test::init_service(Application::prepare(&HealthCheck).test()).await;
    let req = service::test::TestRequest::get().uri("/").to_request();
    let _resp = service::test::call_service(&app, req).await;
}
