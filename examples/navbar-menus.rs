use pagetop::prelude::*;
use pagetop_bootsier::theme::*;

include_locales!(LOC from "examples/locale");

struct SuperMenu;

#[async_trait]
impl Extension for SuperMenu {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![
            &pagetop_aliner::Aliner,
            &pagetop_bootsier::Bootsier,
            &pagetop::base::extension::Welcome,
        ]
    }

    async fn initialize(&self) {
        let navbar_menu = bs::Navbar::brand_left(bs::navbar::Brand::new())
            .with_expand(BreakPoint::LG)
            .with_item(bs::navbar::Item::nav(
                bs::Nav::new()
                    .with_item(bs::nav::Item::link(Lc::t("menus_item_link", &LOC), "/"))
                    .with_item(bs::nav::Item::link_blank(
                        Lc::t("menus_item_blank", &LOC),
                        "https://docs.rs/pagetop",
                    ))
                    .with_item(bs::nav::Item::dropdown(
                        bs::Dropdown::new()
                            .with_title(Lc::t("menus_test_title", &LOC))
                            .with_item(bs::dropdown::Item::header(Lc::t("menus_dev_header", &LOC)))
                            .with_item(bs::dropdown::Item::link(
                                Lc::t("menus_dev_getting_started", &LOC),
                                "/dev/getting-started",
                            ))
                            .with_item(bs::dropdown::Item::link(
                                Lc::t("menus_dev_guides", &LOC),
                                "/dev/guides",
                            ))
                            .with_item(bs::dropdown::Item::link_blank(
                                Lc::t("menus_dev_forum", &LOC),
                                "https://forum.example.dev",
                            ))
                            .with_item(bs::dropdown::Item::divider())
                            .with_item(bs::dropdown::Item::header(Lc::t("menus_sdk_header", &LOC)))
                            .with_item(bs::dropdown::Item::link(
                                Lc::t("menus_sdk_rust", &LOC),
                                "/dev/sdks/rust",
                            ))
                            .with_item(bs::dropdown::Item::link(
                                Lc::t("menus_sdk_js", &LOC),
                                "/dev/sdks/js",
                            ))
                            .with_item(bs::dropdown::Item::link(
                                Lc::t("menus_sdk_python", &LOC),
                                "/dev/sdks/python",
                            ))
                            .with_item(bs::dropdown::Item::divider())
                            .with_item(bs::dropdown::Item::header(Lc::t(
                                "menus_plugin_header",
                                &LOC,
                            )))
                            .with_item(bs::dropdown::Item::link(
                                Lc::t("menus_plugin_auth", &LOC),
                                "/dev/sdks/rust/plugins/auth",
                            ))
                            .with_item(bs::dropdown::Item::link(
                                Lc::t("menus_plugin_cache", &LOC),
                                "/dev/sdks/rust/plugins/cache",
                            ))
                            .with_item(bs::dropdown::Item::divider())
                            .with_item(bs::dropdown::Item::label(Lc::t("menus_item_label", &LOC)))
                            .with_item(bs::dropdown::Item::link_disabled(
                                Lc::t("menus_item_disabled", &LOC),
                                "#",
                            )),
                    ))
                    .with_item(bs::nav::Item::link_disabled(
                        Lc::t("menus_item_disabled", &LOC),
                        "#",
                    )),
            ))
            .with_item(bs::navbar::Item::nav(
                bs::Nav::new()
                    .with_prop(PropsOp::add_classes(class::Margin::with(
                        BoxSide::Start,
                        ScaleSize::Auto,
                    )))
                    .with_item(bs::nav::Item::link(
                        Lc::t("menus_item_sign_up", &LOC),
                        "/auth/sign-up",
                    ))
                    .with_item(bs::nav::Item::link(
                        Lc::t("menus_item_login", &LOC),
                        "/auth/login",
                    )),
            ));

        InRegion::Global(&CoreRegions::Header).add(
            bs::Container::new()
                .with_width(bs::container::Width::FluidMax(UnitValue::RelRem(75.0)))
                .with_child(navbar_menu),
        );
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&SuperMenu).await.run().await
}
