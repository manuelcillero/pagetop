use pagetop::{prelude::*, core::app::AppTrait};

struct Drust;

impl AppTrait for Drust {
    fn enabled_modules(&self) -> Vec<&'static dyn ModuleTrait> {
        vec![
            &pagetop_admin::Admin,
            &pagetop_user::User,
            &pagetop_node::Node,
        ]
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(Drust).await?.run()?.await
}
