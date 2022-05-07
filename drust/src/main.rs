use pagetop::prelude::*;

fn bootstrap() {
    include_module(&pagetop_admin::Admin);
    include_module(&pagetop_user::User);
    include_module(&pagetop_node::Node);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(UsingBootstrap::Fn(bootstrap)).await?.run()?.await
}
