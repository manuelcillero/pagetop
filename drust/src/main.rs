use pagetop::prelude::*;

pub_handle!(APP_DRUST);

struct Drust;

impl ModuleTrait for Drust {
    fn handle(&self) -> Handle {
        APP_DRUST
    }

    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![
            // Modules.
            &pagetop_homedemo::HomeDemo,
            &pagetop_admin::Admin,
            &pagetop_user::User,
            &pagetop_node::Node,
            // Themes.
            &pagetop_minimal::Minimal,
            &pagetop_aliner::Aliner,
            &pagetop_bootsier::Bootsier,
            &pagetop_bulmix::Bulmix,
        ]
    }

    fn drop_modules(&self) -> Vec<ModuleStaticRef> {
        vec![
        //  &pagetop_node::Node
        ]
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&Drust).unwrap().run()?.await
}
