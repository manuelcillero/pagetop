use pagetop::prelude::*;

fn bootstrap() {
    register_module(&pagetop_admin::Admin);
    register_module(&pagetop_user::User);
    register_module(&pagetop_node::Node);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(bootstrap).await?.run()?.await
}
