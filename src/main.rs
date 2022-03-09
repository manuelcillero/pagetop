#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ```
    // let app = pagetop::Application::build(None).await?;
    // app.run()?.await
    // ```
    pagetop::Application::build(None).await?.run()?.await
}
