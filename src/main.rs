#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pagetop::core::server::run(None)?.await
}
