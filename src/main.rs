use pagetop::core::server;

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    server::run()?.await
}
