use pagetop::prelude::*;

use pagetop_bootsier::prelude::*;

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
                    .with_item(nav::Item::link(L10n::l("sample_menus_item_link"), |cx| {
                        cx.route("/")
                    }))
                    .with_item(nav::Item::link_blank(
                        L10n::l("sample_menus_item_blank"),
                        |_| "https://docs.rs/pagetop".into(),
                    ))
                    .with_item(nav::Item::dropdown(
                        Dropdown::new()
                            .with_title(L10n::l("sample_menus_test_title"))
                            .with_item(dropdown::Item::header(L10n::l("sample_menus_dev_header")))
                            .with_item(dropdown::Item::link(
                                L10n::l("sample_menus_dev_getting_started"),
                                |cx| cx.route("/dev/getting-started"),
                            ))
                            .with_item(dropdown::Item::link(
                                L10n::l("sample_menus_dev_guides"),
                                |cx| cx.route("/dev/guides"),
                            ))
                            .with_item(dropdown::Item::link_blank(
                                L10n::l("sample_menus_dev_forum"),
                                |_| "https://forum.example.dev".into(),
                            ))
                            .with_item(dropdown::Item::divider())
                            .with_item(dropdown::Item::header(L10n::l("sample_menus_sdk_header")))
                            .with_item(dropdown::Item::link(
                                L10n::l("sample_menus_sdk_rust"),
                                |cx| cx.route("/dev/sdks/rust"),
                            ))
                            .with_item(dropdown::Item::link(L10n::l("sample_menus_sdk_js"), |cx| {
                                cx.route("/dev/sdks/js")
                            }))
                            .with_item(dropdown::Item::link(
                                L10n::l("sample_menus_sdk_python"),
                                |cx| cx.route("/dev/sdks/python"),
                            ))
                            .with_item(dropdown::Item::divider())
                            .with_item(dropdown::Item::header(L10n::l(
                                "sample_menus_plugin_header",
                            )))
                            .with_item(dropdown::Item::link(
                                L10n::l("sample_menus_plugin_auth"),
                                |cx| cx.route("/dev/sdks/rust/plugins/auth"),
                            ))
                            .with_item(dropdown::Item::link(
                                L10n::l("sample_menus_plugin_cache"),
                                |cx| cx.route("/dev/sdks/rust/plugins/cache"),
                            ))
                            .with_item(dropdown::Item::divider())
                            .with_item(dropdown::Item::label(L10n::l("sample_menus_item_label")))
                            .with_item(dropdown::Item::link_disabled(
                                L10n::l("sample_menus_item_disabled"),
                                |cx| cx.route("#"),
                            )),
                    ))
                    .with_item(nav::Item::link_disabled(
                        L10n::l("sample_menus_item_disabled"),
                        |cx| cx.route("#"),
                    )),
            ))
            .with_item(navbar::Item::nav(
                Nav::new()
                    .with_classes(
                        ClassesOp::Add,
                        classes::Margin::with(Side::Start, ScaleSize::Auto).to_class(),
                    )
                    .with_item(nav::Item::link(
                        L10n::l("sample_menus_item_sign_up"),
                        |cx| cx.route("/auth/sign-up"),
                    ))
                    .with_item(nav::Item::link(L10n::l("sample_menus_item_login"), |cx| {
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
