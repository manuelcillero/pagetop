use pagetop::prelude::*;

pub_const_handler!(APP_DRUST);

struct Drust;

impl ModuleTrait for Drust {
    fn handler(&self) -> Handler {
        APP_DRUST
    }

    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![
            &pagetop_admin::Admin,
            &pagetop_user::User,
            &pagetop_node::Node,
            &pagetop::base::module::homepage::DefaultHomePage,
        ]
    }

    fn uninstall_modules(&self) -> Vec<ModuleStaticRef> {
        vec![
        //  &pagetop_node::Node
        ]
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&Drust).await?.run()?.await
}
