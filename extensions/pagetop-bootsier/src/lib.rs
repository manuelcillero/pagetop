use pagetop::prelude::*;

// GLOBAL ******************************************************************************************

include_files!(bootsier_bs);
include_files!(bootsier_js);

include_locales!(LOCALES_BOOTSIER);

const BOOTSTRAP_VERSION: &str = "5.3.3"; // Versión de la librería Bootstrap.

// API *********************************************************************************************

pub mod config;

pub mod bs;

pub struct Bootsier;

impl ExtensionTrait for Bootsier {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Bootsier)
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![
            //action::theme::BeforeRender::<Region>::new(&Self, before_render_region),
            //action::theme::BeforePrepare::<Button>::new(&Self, before_prepare_button),
            //action::theme::BeforePrepare::<Heading>::new(&Self, before_prepare_heading),
            //action::theme::BeforePrepare::<Paragraph>::new(&Self, before_prepare_paragraph),
            //action::theme::RenderComponent::<Error404>::new(&Self, render_error404),
        ]
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        include_files_service!(scfg, bootsier_bs => "/bootsier/bs");
        include_files_service!(scfg, bootsier_js => "/bootsier/js");
    }
}

impl ThemeTrait for Bootsier {
    #[rustfmt::skip]
    fn regions(&self) -> Vec<(&'static str, L10n)> {
        vec![
            ("region-header",         L10n::t("header",         &LOCALES_BOOTSIER)),
            ("region-nav_branding",   L10n::t("nav_branding",   &LOCALES_BOOTSIER)),
            ("region-nav_main",       L10n::t("nav_main",       &LOCALES_BOOTSIER)),
            ("region-nav_additional", L10n::t("nav_additional", &LOCALES_BOOTSIER)),
            ("region-breadcrumb",     L10n::t("breadcrumb",     &LOCALES_BOOTSIER)),
            ("region-content",        L10n::t("content",        &LOCALES_BOOTSIER)),
            ("region-sidebar_first",  L10n::t("sidebar_first",  &LOCALES_BOOTSIER)),
            ("region-sidebar_second", L10n::t("sidebar_second", &LOCALES_BOOTSIER)),
            ("region-footer",         L10n::t("footer",         &LOCALES_BOOTSIER)),
        ]
    }

    fn render_page_body(&self, page: &mut Page) -> Markup {
        html! {
            body id=[page.body_id().get()] class=[page.body_classes().get()] {
                //@if let Some(skip) = L10n::l("skip_to_content").using(page.context().langid()) {
                //    div class="skip__to_content" {
                //        a href=(concat_string!("#", skip_to_id)) { (skip) }
                //    }
                //}
                (bs::Container::new()
                    .with_id("container-wrapper")
                    .with_breakpoint(bs::BreakPoint::FluidMax(config::SETTINGS.bootsier.max_width))
                    .with_child(Region::of("region-content"))
                    .render(page.context()))
            }
        }
    }

    fn after_render_page_body(&self, page: &mut Page) {
        page.alter_assets(AssetsOp::AddStyleSheet(
            StyleSheet::from("/bootsier/bs/bootstrap.min.css")
                .with_version(BOOTSTRAP_VERSION)
                .with_weight(-99),
        ))
        .alter_assets(AssetsOp::AddJavaScript(
            JavaScript::defer("/bootsier/js/bootstrap.min.js")
                .with_version(BOOTSTRAP_VERSION)
                .with_weight(-99),
        ));
    }

    /*

            fn prepare_body(&self, page: &mut Page) -> PrepareMarkup {
                let skip_to_id = page.body_skip_to().get().unwrap_or("content".to_owned());

                PrepareMarkup::With(html! {
                    body id=[page.body_id().get()] class=[page.body_classes().get()] {
                        @if let Some(skip) = L10n::l("skip_to_content").using(page.context().langid()) {
                            div class="skip__to_content" {
                                a href=(concat_string!("#", skip_to_id)) { (skip) }
                            }
                        }
                        (match page.context().layout() {
                            "admin" => flex::Container::new()
                                .add_item(flex::Item::region().with_id("top-menu"))
                                .add_item(flex::Item::region().with_id("side-menu"))
                                .add_item(flex::Item::region().with_id("content")),
                            _ => flex::Container::new()
                                .add_item(flex::Item::region().with_id("header"))
                                .add_item(flex::Item::region().with_id("nav_branding"))
                                .add_item(flex::Item::region().with_id("nav_main"))
                                .add_item(flex::Item::region().with_id("nav_additional"))
                                .add_item(flex::Item::region().with_id("breadcrumb"))
                                .add_item(flex::Item::region().with_id("content"))
                                .add_item(flex::Item::region().with_id("sidebar_first"))
                                .add_item(flex::Item::region().with_id("sidebar_second"))
                                .add_item(flex::Item::region().with_id("footer")),
                        }.render(page.context()))
                    }
                })
            }
    */

    /*
    }

    fn before_prepare_icon(i: &mut Icon, _cx: &mut Context) {
        i.set_classes(
            ClassesOp::Replace(i.font_size().to_string()),
            with_font(i.font_size()),
        );
    }

    #[rustfmt::skip]
    fn before_prepare_button(b: &mut Button, _cx: &mut Context) {
        b.set_classes(ClassesOp::Replace("button__tap".to_owned()), "btn");
        b.set_classes(
            ClassesOp::Replace(b.style().to_string()),
            match b.style() {
                StyleBase::Default => "btn-primary",
                StyleBase::Info    => "btn-info",
                StyleBase::Success => "btn-success",
                StyleBase::Warning => "btn-warning",
                StyleBase::Danger  => "btn-danger",
                StyleBase::Light   => "btn-light",
                StyleBase::Dark    => "btn-dark",
                StyleBase::Link    => "btn-link",
            },
        );
        b.set_classes(
            ClassesOp::Replace(b.font_size().to_string()),
            with_font(b.font_size()),
        );
    }

    #[rustfmt::skip]
    fn before_prepare_heading(h: &mut Heading, _cx: &mut Context) {
        h.set_classes(
            ClassesOp::Replace(h.size().to_string()),
            match h.size() {
                HeadingSize::ExtraLarge => "display-1",
                HeadingSize::XxLarge    => "display-2",
                HeadingSize::XLarge     => "display-3",
                HeadingSize::Large      => "display-4",
                HeadingSize::Medium     => "display-5",
                _ => "",
            },
        );
    }

    fn before_prepare_paragraph(p: &mut Paragraph, _cx: &mut Context) {
        p.set_classes(
            ClassesOp::Replace(p.font_size().to_string()),
            with_font(p.font_size()),
        );
    }

    fn render_error404(_: &Error404, cx: &mut Context) -> Option<Markup> {
        Some(html! {
            div class="jumbotron" {
                div class="media" {
                    img
                        src="/bootsier/images/caution.png"
                        class="mr-4"
                        style="width: 20%; max-width: 188px"
                        alt="Caution!";
                    div class="media-body" {
                        h1 class="display-4" { ("RESOURCE NOT FOUND") }
                        p class="lead" {
                            (L10n::t("e404-description", &LOCALES_BOOTSIER)
                                .escaped(cx.langid()))
                        }
                        hr class="my-4";
                        p {
                            (L10n::t("e404-description", &LOCALES_BOOTSIER)
                                .escaped(cx.langid()))
                        }
                        a
                            class="btn btn-primary btn-lg"
                            href="/"
                            role="button"
                        {
                            (L10n::t("back-homepage", &LOCALES_BOOTSIER)
                                .escaped(cx.langid()))
                        }
                    }
                }
            }
        })
    */
}

/*
#[rustfmt::skip]
fn with_font(font_size: &FontSize) -> String {
    String::from(match font_size {
        FontSize::ExtraLarge => "fs-1",
        FontSize::XxLarge    => "fs-2",
        FontSize::XLarge     => "fs-3",
        FontSize::Large      => "fs-4",
        FontSize::Medium     => "fs-5",
        _ => "",
    })
}
*/
