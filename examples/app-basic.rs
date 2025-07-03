use pagetop::prelude::*;

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::new().run()?.await
}
