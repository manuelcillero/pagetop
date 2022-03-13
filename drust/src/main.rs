#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pagetop::Application::build(None).await?.run()?.await
}
