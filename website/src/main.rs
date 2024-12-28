use pagetop::prelude::*;

include_files!(BUNDLE_DOC => doc);

include_locales!(LOCALES_WEBSITE);

struct PageTopWebSite;

impl PackageTrait for PageTopWebSite {
    fn name(&self) -> L10n {
        L10n::t("app_name", &LOCALES_WEBSITE)
    }

    fn description(&self) -> L10n {
        L10n::t("app_description", &LOCALES_WEBSITE)
    }

    fn dependencies(&self) -> Vec<PackageRef> {
        vec![
            // Paquetes.
            &pagetop_mdbook::MdBook,
            // Temas.
            &pagetop_bootsier::Bootsier,
        ]
    }

    fn init(&self) {
        /*
                let branding = Branding::new()
                    .with_logo(Some(Image::pagetop()))
                    .with_slogan(L10n::t("app_slogan", &LOCALES_WEBSITE))
                    .with_frontpage(|cx| match cx.langid().language.as_str() {
                        "es" => "/es",
                        _ => "/",
                    });
                let menu = Menu::new()
                    .add_item(menu::Item::link(
                        L10n::t("menu_home", &LOCALES_WEBSITE),
                        |cx| match cx.langid().language.as_str() {
                            "es" => "/es",
                            _ => "/",
                        },
                    ))
                    .add_item(menu::Item::link(
                        L10n::t("menu_documentation", &LOCALES_WEBSITE),
                        |cx| match cx.langid().language.as_str() {
                            "es" => "/doc/latest/es",
                            _ => "/doc/latest/en",
                        },
                    ))
                    .add_item(menu::Item::link_blank(
                        L10n::t("menu_api", &LOCALES_WEBSITE),
                        |_| "https://docs.rs/pagetop",
                    ))
                    .add_item(menu::Item::link_blank(
                        L10n::t("menu_code", &LOCALES_WEBSITE),
                        |_| "https://github.com/manuelcillero/pagetop",
                    ))
                    .add_item(menu::Item::html(Html::with(html! {
                        select id="select-lang" {
                            option value="en" { "EN" }
                            option value="es" { "ES" }
                        }
                        script {
                            r###"
        var selectLang=document.getElementById('select-lang');
        selectLang.value=document.documentElement.lang;
        selectLang.addEventListener('change',function(){window.location.href='/'+selectLang.value;});
                        "###
                        }
                    })));

                InRegion::Named("header").add(AnyComponent::with(
                    flex::Container::new()
                        .with_direction(flex::Direction::Row(BreakPoint::None))
                        .with_justify(flex::Justify::SpaceBetween)
                        .with_align(flex::Align::End)
                        .add_item(flex::Item::with(branding))
                        .add_item(flex::Item::with(menu)),
                ));
                InRegion::Named("pagetop").add(AnyComponent::with(
                    Block::new()
                        .with_style(StyleBase::Info)
                        .add_component(Paragraph::fluent(L10n::t(
                            "under_construction",
                            &LOCALES_WEBSITE,
                        ))),
                ));
                InRegion::Named("footer").add(AnyComponent::with(PoweredBy::new()));
            */
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/doc/latest/{lang}", service::web::get().to(doc_latest));
        pagetop_mdbook::MdBook::mdbook_service(scfg, "/doc", &BUNDLE_DOC);
    }
}

async fn doc_latest(path: service::web::Path<String>) -> service::HttpResponse {
    match path.into_inner().as_str() {
        "es" => Redirect::see_other("/doc/v0.0/es/index.html"),
        _ => Redirect::see_other("/doc/v0.0/en/index.html"),
    }
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&PageTopWebSite).run()?.await
}
