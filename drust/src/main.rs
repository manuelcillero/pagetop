use pagetop::prelude::*;

struct Drust;

impl PackageTrait for Drust {
    fn dependencies(&self) -> Vec<PackageRef> {
        vec![
            // Layouts.
            //&pagetop_bootsier::Bootsier,

            // Packages.
            //&pagetop_admin::Admin,
            //&pagetop_user::User,
            //&pagetop_node::Node,
        ]
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&Drust).run()?.await
}
