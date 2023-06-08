use pagetop::prelude::*;

struct HealthCheck;

impl ModuleTrait for HealthCheck {}

async fn spawn_app() {
    let server = Application::prepare(&HealthCheck).unwrap().server();
    let _ = actix_web::rt::spawn(server);
}

#[actix_web::test]
async fn health_check_works() {
    spawn_app();
}
