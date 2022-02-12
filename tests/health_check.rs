use pagetop::core::server;

fn spawn_app() {
    let server = server::run(None).expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    spawn_app();
}
