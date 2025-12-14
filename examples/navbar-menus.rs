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
            .add_item(navbar::Item::nav(
                Nav::new()
                    .add_item(nav::Item::link(L10n::l("sample_menus_item_link"), |cx| {
                        cx.route("/")
                    }))
                    .add_item(nav::Item::link_blank(
                        L10n::l("sample_menus_item_blank"),
                        |_| "https://docs.rs/pagetop".into(),
                    ))
                    .add_item(nav::Item::dropdown(
                        Dropdown::new()
                            .with_title(L10n::l("sample_menus_test_title"))
                            .add_item(dropdown::Item::header(L10n::l("sample_menus_dev_header")))
                            .add_item(dropdown::Item::link(
                                L10n::l("sample_menus_dev_getting_started"),
                                |cx| cx.route("/dev/getting-started"),
                            ))
                            .add_item(dropdown::Item::link(
                                L10n::l("sample_menus_dev_guides"),
                                |cx| cx.route("/dev/guides"),
                            ))
                            .add_item(dropdown::Item::link_blank(
                                L10n::l("sample_menus_dev_forum"),
                                |_| "https://forum.example.dev".into(),
                            ))
                            .add_item(dropdown::Item::divider())
                            .add_item(dropdown::Item::header(L10n::l("sample_menus_sdk_header")))
                            .add_item(dropdown::Item::link(
                                L10n::l("sample_menus_sdk_rust"),
                                |cx| cx.route("/dev/sdks/rust"),
                            ))
                            .add_item(dropdown::Item::link(L10n::l("sample_menus_sdk_js"), |cx| {
                                cx.route("/dev/sdks/js")
                            }))
                            .add_item(dropdown::Item::link(
                                L10n::l("sample_menus_sdk_python"),
                                |cx| cx.route("/dev/sdks/python"),
                            ))
                            .add_item(dropdown::Item::divider())
                            .add_item(dropdown::Item::header(L10n::l(
                                "sample_menus_plugin_header",
                            )))
                            .add_item(dropdown::Item::link(
                                L10n::l("sample_menus_plugin_auth"),
                                |cx| cx.route("/dev/sdks/rust/plugins/auth"),
                            ))
                            .add_item(dropdown::Item::link(
                                L10n::l("sample_menus_plugin_cache"),
                                |cx| cx.route("/dev/sdks/rust/plugins/cache"),
                            ))
                            .add_item(dropdown::Item::divider())
                            .add_item(dropdown::Item::label(L10n::l("sample_menus_item_label")))
                            .add_item(dropdown::Item::link_disabled(
                                L10n::l("sample_menus_item_disabled"),
                                |cx| cx.route("#"),
                            )),
                    ))
                    .add_item(nav::Item::link_disabled(
                        L10n::l("sample_menus_item_disabled"),
                        |cx| cx.route("#"),
                    )),
            ))
            .add_item(navbar::Item::nav(
                Nav::new()
                    .with_classes(
                        ClassesOp::Add,
                        classes::Margin::with(Side::Start, ScaleSize::Auto).to_class(),
                    )
                    .add_item(nav::Item::link(
                        L10n::l("sample_menus_item_sign_up"),
                        |cx| cx.route("/auth/sign-up"),
                    ))
                    .add_item(nav::Item::link(L10n::l("sample_menus_item_login"), |cx| {
                        cx.route("/auth/login")
                    })),
            ));

        InRegion::Global(&DefaultRegion::Header).add(Child::with(
            Container::new()
                .with_width(container::Width::FluidMax(UnitValue::RelRem(75.0)))
                .add_child(navbar_menu),
        ));
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&SuperMenu).run()?.await
}
