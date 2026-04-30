use pagetop::prelude::*;

use pagetop_bootsier::prelude::*;

include_locales!(LOC from "examples/locale");

struct SuperMenu;

impl Extension for SuperMenu {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_aliner::Aliner, &pagetop_bootsier::Bootsier]
    }

    fn initialize(&self) {
        let navbar_menu = Navbar::brand_left(navbar::Brand::new())
            .with_expand(BreakPoint::LG)
            .with_item(navbar::Item::nav(
                Nav::new()
                    .with_item(nav::Item::link(L10n::t("menus_item_link", &LOC), |cx| {
                        cx.route("/")
                    }))
                    .with_item(nav::Item::link_blank(
                        L10n::t("menus_item_blank", &LOC),
                        |_| "https://docs.rs/pagetop".into(),
                    ))
                    .with_item(nav::Item::dropdown(
                        Dropdown::new()
                            .with_title(L10n::t("menus_test_title", &LOC))
                            .with_item(dropdown::Item::header(L10n::t("menus_dev_header", &LOC)))
                            .with_item(dropdown::Item::link(
                                L10n::t("menus_dev_getting_started", &LOC),
                                |cx| cx.route("/dev/getting-started"),
                            ))
                            .with_item(dropdown::Item::link(
                                L10n::t("menus_dev_guides", &LOC),
                                |cx| cx.route("/dev/guides"),
                            ))
                            .with_item(dropdown::Item::link_blank(
                                L10n::t("menus_dev_forum", &LOC),
                                |_| "https://forum.example.dev".into(),
                            ))
                            .with_item(dropdown::Item::divider())
                            .with_item(dropdown::Item::header(L10n::t("menus_sdk_header", &LOC)))
                            .with_item(dropdown::Item::link(
                                L10n::t("menus_sdk_rust", &LOC),
                                |cx| cx.route("/dev/sdks/rust"),
                            ))
                            .with_item(dropdown::Item::link(L10n::t("menus_sdk_js", &LOC), |cx| {
                                cx.route("/dev/sdks/js")
                            }))
                            .with_item(dropdown::Item::link(
                                L10n::t("menus_sdk_python", &LOC),
                                |cx| cx.route("/dev/sdks/python"),
                            ))
                            .with_item(dropdown::Item::divider())
                            .with_item(dropdown::Item::header(L10n::t("menus_plugin_header", &LOC)))
                            .with_item(dropdown::Item::link(
                                L10n::t("menus_plugin_auth", &LOC),
                                |cx| cx.route("/dev/sdks/rust/plugins/auth"),
                            ))
                            .with_item(dropdown::Item::link(
                                L10n::t("menus_plugin_cache", &LOC),
                                |cx| cx.route("/dev/sdks/rust/plugins/cache"),
                            ))
                            .with_item(dropdown::Item::divider())
                            .with_item(dropdown::Item::label(L10n::t("menus_item_label", &LOC)))
                            .with_item(dropdown::Item::link_disabled(
                                L10n::t("menus_item_disabled", &LOC),
                                |cx| cx.route("#"),
                            )),
                    ))
                    .with_item(nav::Item::link_disabled(
                        L10n::t("menus_item_disabled", &LOC),
                        |cx| cx.route("#"),
                    )),
            ))
            .with_item(navbar::Item::nav(
                Nav::new()
                    .with_classes(
                        ClassesOp::Add,
                        classes::Margin::with(Side::Start, ScaleSize::Auto).to_class(),
                    )
                    .with_item(nav::Item::link(L10n::t("menus_item_sign_up", &LOC), |cx| {
                        cx.route("/auth/sign-up")
                    }))
                    .with_item(nav::Item::link(L10n::t("menus_item_login", &LOC), |cx| {
                        cx.route("/auth/login")
                    })),
            ));

        InRegion::Global(&DefaultRegion::Header).add(
            Container::new()
                .with_width(container::Width::FluidMax(UnitValue::RelRem(75.0)))
                .with_child(navbar_menu),
        );
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&SuperMenu).run()?.await
}
