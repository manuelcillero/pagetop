async fn spawn_app() {
    let server = pagetop::Application::prepare(None)
        .await?
        .run()?
        .expect("Failed to prepare server");
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    spawn_app();
}
