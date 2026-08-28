use pagetop::prelude::*;

struct NavDemo;

#[async_trait]
impl Extension for NavDemo {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop::base::extension::Welcome]
    }

    async fn initialize(&self) {
        let main_nav = Nav::new()
            .with_item(nav::Item::link(Lc::n("Home"), "/"))
            .with_item(nav::Item::link_blank(
                Lc::n("Docs"),
                "https://docs.rs/pagetop",
            ))
            .with_item(nav::Item::dropdown(
                Dropdown::new()
                    .with_title(Lc::n("Tools"))
                    .with_item(dropdown::Item::link(Lc::n("Generator"), "/tools/gen"))
                    .with_item(dropdown::Item::link(Lc::n("Reports"), "/tools/reports"))
                    .with_item(dropdown::Item::divider())
                    .with_item(dropdown::Item::header(Lc::n("Danger zone")))
                    .with_item(dropdown::Item::button(Lc::n("Reset settings"))),
            ))
            .with_item(nav::Item::link_disabled(Lc::n("Disabled"), "#"));

        let user_menu = Dropdown::new()
            .with_title(Lc::n("Account"))
            .with_item(dropdown::Item::label(Lc::n("Signed in as demo")))
            .with_item(dropdown::Item::divider())
            .with_item(dropdown::Item::link(Lc::n("Profile"), "/profile"))
            .with_item(dropdown::Item::link(Lc::n("Settings"), "/settings"))
            .with_item(dropdown::Item::divider())
            .with_item(dropdown::Item::button(Lc::n("Sign out")));

        InRegion::Global(&CoreRegions::Header).add(main_nav);
        InRegion::Global(&CoreRegions::Aside).add(user_menu);
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&NavDemo).await.run().await
}
