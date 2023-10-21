use pagetop::prelude::*;

new_handle!(THEME_BOOTSIER);

static_locales!(LOCALES_BOOTSIER);

static_files!(bootsier);

pub struct Bootsier;

impl ModuleTrait for Bootsier {
    fn handle(&self) -> Handle {
        THEME_BOOTSIER
    }

    fn theme(&self) -> Option<ThemeRef> {
        Some(&Bootsier)
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        static_files_service!(scfg, "/bootsier", bootsier);
    }
}

impl ThemeTrait for Bootsier {
    #[rustfmt::skip]
    fn regions(&self) -> Vec<(&'static str, L10n)> {
        vec![
            ("header",         L10n::t("header",         &LOCALES_BOOTSIER)),
            ("nav_branding",   L10n::t("nav_branding",   &LOCALES_BOOTSIER)),
            ("nav_main",       L10n::t("nav_main",       &LOCALES_BOOTSIER)),
            ("nav_additional", L10n::t("nav_additional", &LOCALES_BOOTSIER)),
            ("breadcrumb",     L10n::t("breadcrumb",     &LOCALES_BOOTSIER)),
            ("content",        L10n::t("breadcrumb",     &LOCALES_BOOTSIER)),
            ("sidebar_first",  L10n::t("sidebar_first",  &LOCALES_BOOTSIER)),
            ("sidebar_second", L10n::t("sidebar_second", &LOCALES_BOOTSIER)),
            ("footer",         L10n::t("footer",         &LOCALES_BOOTSIER)),
        ]
    }

    fn prepare_body(&self, page: &mut Page) -> Markup {
        match page.template() {
            "admin" => html! {
                body class=[page.body_classes().get()] {
                    @for region in &[
                        "top-menu",
                        "side-menu",
                        "content"
                    ] {
                        @if let Some(content) = page.prepare_region(region) {
                            #(region) class="region" { (content) }
                        }
                    }
                }
            },
            _ => {
                let header = page.prepare_region("header");
                let nav_branding = page.prepare_region("nav_branding");
                let nav_main = page.prepare_region("nav_main");
                let nav_additional = page.prepare_region("nav_additional");
                let breadcrumb = page.prepare_region("breadcrumb");
                let content = page.prepare_region("content");
                let sidebar_first = page.prepare_region("sidebar_first");
                let sidebar_second = page.prepare_region("sidebar_second");
                let footer = page.prepare_region("footer");
                html! {
                    body class=[page.body_classes().get()] {
                        (header.unwrap_or_default())
                        (nav_branding.unwrap_or_default())
                        (nav_main.unwrap_or_default())
                        (nav_additional.unwrap_or_default())
                        (breadcrumb.unwrap_or_default())
                        (content.unwrap_or_default())
                        (sidebar_first.unwrap_or_default())
                        (sidebar_second.unwrap_or_default())
                        (footer.unwrap_or_default())
                    }
                }
            }
        }
    }

    fn after_prepare_body(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/base/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/bootsier/css/bootstrap.min.css")
                    .with_version("5.1.3")
                    .with_weight(-99),
            ))
            .alter_context(ContextOp::AddJavaScript(
                JavaScript::at("/bootsier/js/bootstrap.bundle.min.js")
                    .with_version("5.1.3")
                    .with_weight(-99),
            ))
            .alter_context(ContextOp::AddBaseAssets);
    }

    fn render_component(&self, component: &dyn ComponentTrait, cx: &mut Context) -> Option<Markup> {
        match component.handle() {
            ERROR_404 => Some(html! {
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
            }),
            _ => None,
        }
    }
}
