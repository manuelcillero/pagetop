use pagetop::prelude::*;

struct Drust;

impl PackageTrait for Drust {
    fn dependencies(&self) -> Vec<PackageRef> {
        vec![
            // Packages.
            &pagetop_admin::Admin,
            &pagetop_user::User,
            &pagetop_node::Node,
            // Themes.
            &pagetop_bootsier::Bootsier,
            &pagetop_bulmix::Bulmix,
        ]
    }

    fn drop_packages(&self) -> Vec<PackageRef> {
        vec![
        //  &pagetop_node::Node
        ]
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&Drust).run()?.await
}
