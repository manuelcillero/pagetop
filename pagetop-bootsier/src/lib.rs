use pagetop::prelude::*;
use pagetop_jquery::JQuery;

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

    fn dependencies(&self) -> Vec<ModuleRef> {
        vec![&pagetop_jquery::JQuery]
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
                            #(region) { (content) }
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

    fn after_prepare_body(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/bootsier/css/bootstrap.min.css")
                    .with_version("5.1.3")
                    .with_weight(-99),
            ))
            .alter_context(ContextOp::AddJavaScript(
                JavaScript::at("/bootsier/js/bootstrap.bundle.min.js")
                    .with_version("5.1.3")
                    .with_weight(-99),
            ));

        if let Some(true) = page.context().get_param::<bool>(PARAM_INCLUDE_FLEX) {
            page.alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/theme/css/flex.css").with_version("0.0.0"),
            ));
        }
        if let Some(true) = page.context().get_param::<bool>(PARAM_INCLUDE_ICONS) {
            page.alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/theme/icons/bootstrap-icons.css").with_version("1.8.2"),
            ));
        }

        JQuery.enable_jquery(page.context());
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
                                (L10n::t("e404-description", &LOCALES_BOOTSIER).prepare(cx))
                            }
                            hr class="my-4";
                            p {
                                (L10n::t("e404-description", &LOCALES_BOOTSIER).prepare(cx))
                            }
                            a
                                class="btn btn-primary btn-lg"
                                href="/"
                                role="button"
                            {
                                (L10n::t("back-homepage", &LOCALES_BOOTSIER).prepare(cx))
                            }
                        }
                    }
                }
            }),
            _ => None,
        }
    }
}
