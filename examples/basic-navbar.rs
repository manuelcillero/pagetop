use pagetop::prelude::*;

include_locales!(LOC from "examples/locale");

struct NavbarDemo;

#[async_trait]
impl Extension for NavbarDemo {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![
            &pagetop_bootsier::Bootsier,
            &pagetop::base::extension::Welcome,
        ]
    }

    async fn initialize(&self) {
        let brand = Brand::new()
            .with_image(image::Source::logo(PageTopSvg::Color))
            .with_title(Lc::n("PageTop"))
            .with_route(Route::from("/"));

        let main_nav = Nav::new()
            .with_item(nav::Item::link(Lc::t("navbar_home", &LOC), "/"))
            .with_item(nav::Item::link_blank(
                Lc::t("navbar_docs", &LOC),
                "https://docs.rs/pagetop",
            ))
            .with_item(nav::Item::dropdown(
                Dropdown::new()
                    .with_title(Lc::t("navbar_tools", &LOC))
                    .with_item(dropdown::Item::link(
                        Lc::t("navbar_generator", &LOC),
                        "/tools/gen",
                    ))
                    .with_item(dropdown::Item::link(
                        Lc::t("navbar_reports", &LOC),
                        "/tools/reports",
                    )),
            ))
            .with_item(nav::Item::link(Lc::t("navbar_active", &LOC), "/active").with_active(true))
            .with_item(nav::Item::link_disabled(
                Lc::t("navbar_disabled", &LOC),
                "#",
            ));

        let navbar = Navbar::brand_left(brand).with_item(navbar::Item::nav(main_nav));

        InRegion::Global(&CoreRegions::Header).add(navbar);
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&NavbarDemo).await.run().await
}
