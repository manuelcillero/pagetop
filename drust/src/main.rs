use pagetop::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(essence).await?.run()?.await
}
