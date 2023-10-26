use pagetop::prelude::*;

new_handle!(MODULE_TEST_SERVER_HEALTH_CHECK);

struct HealthCheck;

impl ModuleTrait for HealthCheck {
    fn handle(&self) -> Handle {
        MODULE_TEST_SERVER_HEALTH_CHECK
    }
}

#[pagetop::test]
async fn health_check_works() {
    let app = service::test::init_service(Application::prepare(&HealthCheck).unwrap().test()).await;
    let req = service::test::TestRequest::get().uri("/").to_request();
    let _resp = service::test::call_service(&app, req).await;

    //  assert_eq!("OK", "OK");
}
