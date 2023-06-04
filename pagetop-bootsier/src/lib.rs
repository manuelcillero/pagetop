use pagetop::prelude::*;

define_handle!(THEME_BOOTSIER);

define_locale!(LOCALE_BOOTSIER, "static/locales");

include!(concat!(env!("OUT_DIR"), "/bootsier.rs"));

pub struct Bootsier;

impl ModuleTrait for Bootsier {
    fn handle(&self) -> Handle {
        THEME_BOOTSIER
    }

    fn theme(&self) -> Option<ThemeStaticRef> {
        Some(&Bootsier)
    }

    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![&pagetop_jquery::JQuery]
    }

    fn configure_service(&self, cfg: &mut server::web::ServiceConfig) {
        serve_static_files!(cfg, "/bootsier", bundle_bootsier);
    }
}

impl ThemeTrait for Bootsier {
    #[rustfmt::skip]
    fn regions(&self) -> Vec<(&'static str, L10n)> {
        vec![
            ("header",         L10n::t("header",         &LOCALE_BOOTSIER)),
            ("nav_branding",   L10n::t("nav_branding",   &LOCALE_BOOTSIER)),
            ("nav_main",       L10n::t("nav_main",       &LOCALE_BOOTSIER)),
            ("nav_additional", L10n::t("nav_additional", &LOCALE_BOOTSIER)),
            ("breadcrumb",     L10n::t("breadcrumb",     &LOCALE_BOOTSIER)),
            ("content",        L10n::t("breadcrumb",     &LOCALE_BOOTSIER)),
            ("sidebar_first",  L10n::t("sidebar_first",  &LOCALE_BOOTSIER)),
            ("sidebar_second", L10n::t("sidebar_second", &LOCALE_BOOTSIER)),
            ("footer",         L10n::t("footer",         &LOCALE_BOOTSIER)),
        ]
    }

    fn before_render_page(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::located("/bootsier/css/bootstrap.min.css")
                    .with_version("5.1.3")
                    .with_weight(-99),
            ))
            .alter_context(ContextOp::AddJavaScript(
                JavaScript::located("/bootsier/js/bootstrap.bundle.min.js")
                    .with_version("5.1.3")
                    .with_weight(-99),
            ));
        pagetop_jquery::JQuery::add_jquery(page.context());
    }

    fn render_page_body(&self, page: &mut Page) -> Markup {
        match page.template() {
            "admin" => html! {
                body class=[page.body_classes().get()] {
                    @for region in &[
                        "top-menu",
                        "side-menu",
                        "content"
                    ] {
                        @if let Some(content) = page.render_region(region) {
                            #(region) { (content) }
                        }
                    }
                }
            },
            _ => {
                let header         = page.render_region("header");
                let nav_branding   = page.render_region("nav_branding");
                let nav_main       = page.render_region("nav_main");
                let nav_additional = page.render_region("nav_additional");
                let breadcrumb     = page.render_region("breadcrumb");
                let content        = page.render_region("content");
                let sidebar_first  = page.render_region("sidebar_first");
                let sidebar_second = page.render_region("sidebar_second");
                let footer         = page.render_region("footer");
                html! {
                    body class=[page.body_classes().get()] {
                        @if header.is_some() {
                            #header { (header.unwrap()) }
                        }
                        @if nav_branding.is_some() {
                            #nav_branding { (nav_branding.unwrap()) }
                        }
                        @if nav_main.is_some() {
                            #nav_main { (nav_main.unwrap()) }
                        }
                        @if nav_additional.is_some() {
                            #nav_additional { (nav_additional.unwrap()) }
                        }
                        @if breadcrumb.is_some() {
                            #breadcrumb { (breadcrumb.unwrap()) }
                        }
                        @if content.is_some() {
                            #content { (content.unwrap()) }
                        }
                        @if sidebar_first.is_some() {
                            #sidebar_first { (sidebar_first.unwrap()) }
                        }
                        @if sidebar_second.is_some() {
                            #sidebar_second { (sidebar_second.unwrap()) }
                        }
                        @if footer.is_some() {
                            #footer { (footer.unwrap()) }
                        }
                    }
                }
            }
        }
    }

    fn render_component(
        &self,
        component: &dyn ComponentTrait,
        rcx: &mut RenderContext,
    ) -> Option<Markup> {
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
                                (L10n::t("e404-description", &LOCALE_BOOTSIER).render(rcx))
                            }
                            hr class="my-4";
                            p {
                                (L10n::t("e404-description", &LOCALE_BOOTSIER).render(rcx))
                            }
                            a
                                class="btn btn-primary btn-lg"
                                href="/"
                                role="button"
                            {
                                (L10n::t("back-homepage", &LOCALE_BOOTSIER).render(rcx))
                            }
                        }
                    }
                }
            }),
            _ => None,
        }
    }
}
