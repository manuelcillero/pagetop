use pagetop::prelude::*;

use pagetop_bootsier::prelude::*;

struct SuperMenu;

impl Extension for SuperMenu {
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![&pagetop_aliner::Aliner, &pagetop_bootsier::Bootsier]
    }

    fn initialize(&self) {
        let home_path = |cx: &Context| util::join!("/lang/", cx.langid().language.as_str()).into();

        let navbar_menu = Navbar::brand_left(navbar::Brand::new().with_path(Some(home_path)))
            .with_expand(BreakPoint::LG)
            .add_item(navbar::Item::nav(
                Nav::new()
                    .add_item(nav::Item::link(
                        L10n::l("sample_menus_item_link"),
                        home_path,
                    ))
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
                                |_| "/dev/getting-started".into(),
                            ))
                            .add_item(dropdown::Item::link(
                                L10n::l("sample_menus_dev_guides"),
                                |_| "/dev/guides".into(),
                            ))
                            .add_item(dropdown::Item::link_blank(
                                L10n::l("sample_menus_dev_forum"),
                                |_| "https://forum.example.dev".into(),
                            ))
                            .add_item(dropdown::Item::divider())
                            .add_item(dropdown::Item::header(L10n::l("sample_menus_sdk_header")))
                            .add_item(dropdown::Item::link(
                                L10n::l("sample_menus_sdk_rust"),
                                |_| "/dev/sdks/rust".into(),
                            ))
                            .add_item(dropdown::Item::link(L10n::l("sample_menus_sdk_js"), |_| {
                                "/dev/sdks/js".into()
                            }))
                            .add_item(dropdown::Item::link(
                                L10n::l("sample_menus_sdk_python"),
                                |_| "/dev/sdks/python".into(),
                            ))
                            .add_item(dropdown::Item::divider())
                            .add_item(dropdown::Item::header(L10n::l(
                                "sample_menus_plugin_header",
                            )))
                            .add_item(dropdown::Item::link(
                                L10n::l("sample_menus_plugin_auth"),
                                |_| "/dev/sdks/rust/plugins/auth".into(),
                            ))
                            .add_item(dropdown::Item::link(
                                L10n::l("sample_menus_plugin_cache"),
                                |_| "/dev/sdks/rust/plugins/cache".into(),
                            ))
                            .add_item(dropdown::Item::divider())
                            .add_item(dropdown::Item::label(L10n::l("sample_menus_item_label")))
                            .add_item(dropdown::Item::link_disabled(
                                L10n::l("sample_menus_item_disabled"),
                                |_| "#".into(),
                            )),
                    ))
                    .add_item(nav::Item::link_disabled(
                        L10n::l("sample_menus_item_disabled"),
                        |_| "#".into(),
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
                        |_| "/auth/sign-up".into(),
                    ))
                    .add_item(nav::Item::link(L10n::l("sample_menus_item_login"), |_| {
                        "/auth/login".into()
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
