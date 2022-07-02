use pagetop::prelude::*;

struct Drust;

impl AppTrait for Drust {
    fn enable_modules(&self) -> Vec<&'static dyn ModuleTrait> {
        vec![
            &pagetop_admin::Admin,
            &pagetop_user::User,
            &pagetop_node::Node,
            &pagetop::base::module::demopage::Demopage,
        ]
    }

    fn themes(&self) -> Vec<&'static dyn ThemeTrait> {
        vec![
            &pagetop::base::theme::bulmix::Bulmix,
        ]
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(Drust).await?.run()?.await
}
