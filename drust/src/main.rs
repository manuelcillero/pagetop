use pagetop::prelude::*;

struct Drust;

impl AppTrait for Drust {
    fn enable_modules(&self) -> Vec<ModuleStaticRef> {
        vec![
            &pagetop_admin::Admin,
            &pagetop_user::User,
            &pagetop_node::Node,
        ]
    }

    fn disable_modules(&self) -> Vec<ModuleStaticRef> {
        vec![
        //  &pagetop_node::Node,
        ]
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(Drust).await?.run()?.await
}
