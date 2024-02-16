use pagetop::prelude::*;

static_locales!(LOCALES_BOOTSIER);

static_files!(bootsier);

pub struct Bootsier;

impl PackageTrait for Bootsier {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Bootsier)
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        service_for_static_files!(scfg, bootsier => "/bootsier");
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
                        (self.prepare_region(page, region))
                    }
                }
            },
            _ => html! {
                body class=[page.body_classes().get()] {
                    (self.prepare_region(page, "header"))
                    (self.prepare_region(page, "nav_branding"))
                    (self.prepare_region(page, "nav_main"))
                    (self.prepare_region(page, "nav_additional"))
                    (self.prepare_region(page, "breadcrumb"))
                    (self.prepare_region(page, "content"))
                    (self.prepare_region(page, "sidebar_first"))
                    (self.prepare_region(page, "sidebar_second"))
                    (self.prepare_region(page, "footer"))
                }
            },
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
            .alter_context(ContextOp::AddBaseAssets)
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::at("/bootsier/css/styles.css").with_version("0.0.1"),
            ));
    }

    fn before_prepare_component(&self, component: &mut dyn ComponentTrait, _cx: &mut Context) {
        match component.type_id() {
            t if t == TypeId::of::<Icon>() => {
                if let Some(i) = component_as_mut::<Icon>(component) {
                    match i.font_size() {
                        FontSize::ExtraLarge => {
                            i.replace_classes(i.font_size().to_string(), "fs-1");
                        }
                        FontSize::XxLarge => {
                            i.replace_classes(i.font_size().to_string(), "fs-2");
                        }
                        FontSize::XLarge => {
                            i.replace_classes(i.font_size().to_string(), "fs-3");
                        }
                        FontSize::Large => {
                            i.replace_classes(i.font_size().to_string(), "fs-4");
                        }
                        FontSize::Medium => {
                            i.replace_classes(i.font_size().to_string(), "fs-5");
                        }
                        _ => {}
                    };
                }
            }
            t if t == TypeId::of::<Button>() => {
                if let Some(b) = component_as_mut::<Button>(component) {
                    match b.style() {
                        ButtonStyle::Default => {
                            b.replace_classes(b.style().to_string(), "btn btn-primary");
                        }
                        ButtonStyle::Info => {
                            b.replace_classes(b.style().to_string(), "btn btn-info");
                        }
                        ButtonStyle::Success => {
                            b.replace_classes(b.style().to_string(), "btn btn-success");
                        }
                        ButtonStyle::Warning => {
                            b.replace_classes(b.style().to_string(), "btn btn-warning");
                        }
                        ButtonStyle::Danger => {
                            b.replace_classes(b.style().to_string(), "btn btn-danger");
                        }
                        ButtonStyle::Light => {
                            b.replace_classes(b.style().to_string(), "btn btn-light");
                        }
                        ButtonStyle::Dark => {
                            b.replace_classes(b.style().to_string(), "btn btn-dark");
                        }
                        ButtonStyle::Link => {
                            b.replace_classes(b.style().to_string(), "btn btn-link");
                        }
                    };
                    match b.font_size() {
                        FontSize::ExtraLarge => {
                            b.replace_classes(b.font_size().to_string(), "fs-1");
                        }
                        FontSize::XxLarge => {
                            b.replace_classes(b.font_size().to_string(), "fs-2");
                        }
                        FontSize::XLarge => {
                            b.replace_classes(b.font_size().to_string(), "fs-3");
                        }
                        FontSize::Large => {
                            b.replace_classes(b.font_size().to_string(), "fs-4");
                        }
                        FontSize::Medium => {
                            b.replace_classes(b.font_size().to_string(), "fs-5");
                        }
                        _ => {}
                    };
                }
            }
            t if t == TypeId::of::<Heading>() => {
                if let Some(h) = component_as_mut::<Heading>(component) {
                    match h.size() {
                        HeadingSize::ExtraLarge => {
                            h.replace_classes(h.size().to_string(), "display-1");
                        }
                        HeadingSize::XxLarge => {
                            h.replace_classes(h.size().to_string(), "display-2");
                        }
                        HeadingSize::XLarge => {
                            h.replace_classes(h.size().to_string(), "display-3");
                        }
                        HeadingSize::Large => {
                            h.replace_classes(h.size().to_string(), "display-4");
                        }
                        HeadingSize::Medium => {
                            h.replace_classes(h.size().to_string(), "display-5");
                        }
                        _ => {}
                    };
                }
            }
            t if t == TypeId::of::<Paragraph>() => {
                if let Some(p) = component_as_mut::<Paragraph>(component) {
                    match p.font_size() {
                        FontSize::ExtraLarge => {
                            p.replace_classes(p.font_size().to_string(), "fs-1");
                        }
                        FontSize::XxLarge => {
                            p.replace_classes(p.font_size().to_string(), "fs-2");
                        }
                        FontSize::XLarge => {
                            p.replace_classes(p.font_size().to_string(), "fs-3");
                        }
                        FontSize::Large => {
                            p.replace_classes(p.font_size().to_string(), "fs-4");
                        }
                        FontSize::Medium => {
                            p.replace_classes(p.font_size().to_string(), "fs-5");
                        }
                        _ => {}
                    };
                }
            }
            _ => {}
        }
    }

    fn render_component(&self, component: &dyn ComponentTrait, cx: &mut Context) -> Option<Markup> {
        match component.type_id() {
            t if t == TypeId::of::<Error404>() => Some(html! {
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
