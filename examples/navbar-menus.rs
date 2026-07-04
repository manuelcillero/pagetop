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
            .with_expand(token::BreakPoint::LG)
            .with_item(bs::navbar::Item::nav(
                bs::Nav::new()
                    .with_item(bs::nav::Item::link(
                        L10n::t("menus_item_link", &LOC),
                        |cx| cx.route("/"),
                    ))
                    .with_item(bs::nav::Item::link_blank(
                        L10n::t("menus_item_blank", &LOC),
                        |_| "https://docs.rs/pagetop".into(),
                    ))
                    .with_item(bs::nav::Item::dropdown(
                        bs::Dropdown::new()
                            .with_title(L10n::t("menus_test_title", &LOC))
                            .with_item(bs::dropdown::Item::header(L10n::t(
                                "menus_dev_header",
                                &LOC,
                            )))
                            .with_item(bs::dropdown::Item::link(
                                L10n::t("menus_dev_getting_started", &LOC),
                                |cx| cx.route("/dev/getting-started"),
                            ))
                            .with_item(bs::dropdown::Item::link(
                                L10n::t("menus_dev_guides", &LOC),
                                |cx| cx.route("/dev/guides"),
                            ))
                            .with_item(bs::dropdown::Item::link_blank(
                                L10n::t("menus_dev_forum", &LOC),
                                |_| "https://forum.example.dev".into(),
                            ))
                            .with_item(bs::dropdown::Item::divider())
                            .with_item(bs::dropdown::Item::header(L10n::t(
                                "menus_sdk_header",
                                &LOC,
                            )))
                            .with_item(bs::dropdown::Item::link(
                                L10n::t("menus_sdk_rust", &LOC),
                                |cx| cx.route("/dev/sdks/rust"),
                            ))
                            .with_item(bs::dropdown::Item::link(
                                L10n::t("menus_sdk_js", &LOC),
                                |cx| cx.route("/dev/sdks/js"),
                            ))
                            .with_item(bs::dropdown::Item::link(
                                L10n::t("menus_sdk_python", &LOC),
                                |cx| cx.route("/dev/sdks/python"),
                            ))
                            .with_item(bs::dropdown::Item::divider())
                            .with_item(bs::dropdown::Item::header(L10n::t(
                                "menus_plugin_header",
                                &LOC,
                            )))
                            .with_item(bs::dropdown::Item::link(
                                L10n::t("menus_plugin_auth", &LOC),
                                |cx| cx.route("/dev/sdks/rust/plugins/auth"),
                            ))
                            .with_item(bs::dropdown::Item::link(
                                L10n::t("menus_plugin_cache", &LOC),
                                |cx| cx.route("/dev/sdks/rust/plugins/cache"),
                            ))
                            .with_item(bs::dropdown::Item::divider())
                            .with_item(bs::dropdown::Item::label(L10n::t("menus_item_label", &LOC)))
                            .with_item(bs::dropdown::Item::link_disabled(
                                L10n::t("menus_item_disabled", &LOC),
                                |cx| cx.route("#"),
                            )),
                    ))
                    .with_item(bs::nav::Item::link_disabled(
                        L10n::t("menus_item_disabled", &LOC),
                        |cx| cx.route("#"),
                    )),
            ))
            .with_item(bs::navbar::Item::nav(
                bs::Nav::new()
                    .with_prop(PropsOp::add_classes(class::Margin::with(
                        token::Side::Start,
                        token::ScaleSize::Auto,
                    )))
                    .with_item(bs::nav::Item::link(
                        L10n::t("menus_item_sign_up", &LOC),
                        |cx| cx.route("/auth/sign-up"),
                    ))
                    .with_item(bs::nav::Item::link(
                        L10n::t("menus_item_login", &LOC),
                        |cx| cx.route("/auth/login"),
                    )),
            ));

        InRegion::Global(&DefaultRegion::Header).add(
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
