use pagetop::prelude::*;

fn bootstrap() {
    register_module(&pagetop_admin::AdminModule);
    register_module(&pagetop_user::UserModule);
    register_module(&pagetop_node::NodeModule);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(bootstrap).await?.run()?.await
}
