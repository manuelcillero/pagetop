use pagetop::prelude::*;

struct Drust;

impl_handle!(APP_DRUST for Drust);

impl ModuleTrait for Drust {
    fn dependencies(&self) -> Vec<ModuleRef> {
        vec![
            // Themes.
            &pagetop_bootsier::Bootsier,
            &pagetop_bulmix::Bulmix,
            // Modules.
            &pagetop_homedemo::HomeDemo,
            &pagetop_admin::Admin,
            &pagetop_user::User,
            &pagetop_node::Node,
        ]
    }

    fn drop_modules(&self) -> Vec<ModuleRef> {
        vec![
        //  &pagetop_node::Node
        ]
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&Drust).unwrap().run()?.await
}
