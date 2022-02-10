fn spawn_app() {
    let server = pagetop::core::server::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    spawn_app();
}
