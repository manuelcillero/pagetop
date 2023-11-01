use pagetop::prelude::*;

new_handle!(THEME_BOOTSIER);

new_static_locales!(LOCALES_BOOTSIER);

new_static_files!(bootsier);

pub struct Bootsier;

impl ModuleTrait for Bootsier {
    fn handle(&self) -> Handle {
        THEME_BOOTSIER
    }

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
        match component.handle() {
            COMPONENT_BASE_ICON => {
                let i = component_as_mut::<Icon>(component);
                match i.font_size() {
                    FontSize::ExtraLarge => {
                        i.alter_classes(ClassesOp::Replace(i.font_size().to_string()), "fs-1");
                    }
                    FontSize::XxLarge => {
                        i.alter_classes(ClassesOp::Replace(i.font_size().to_string()), "fs-2");
                    }
                    FontSize::XLarge => {
                        i.alter_classes(ClassesOp::Replace(i.font_size().to_string()), "fs-3");
                    }
                    FontSize::Large => {
                        i.alter_classes(ClassesOp::Replace(i.font_size().to_string()), "fs-4");
                    }
                    FontSize::Medium => {
                        i.alter_classes(ClassesOp::Replace(i.font_size().to_string()), "fs-5");
                    }
                    _ => {}
                };
            }
            COMPONENT_BASE_ANCHOR => {
                let a = component_as_mut::<Anchor>(component);
                match a.font_size() {
                    FontSize::ExtraLarge => {
                        a.alter_classes(ClassesOp::Replace(a.font_size().to_string()), "fs-1");
                    }
                    FontSize::XxLarge => {
                        a.alter_classes(ClassesOp::Replace(a.font_size().to_string()), "fs-2");
                    }
                    FontSize::XLarge => {
                        a.alter_classes(ClassesOp::Replace(a.font_size().to_string()), "fs-3");
                    }
                    FontSize::Large => {
                        a.alter_classes(ClassesOp::Replace(a.font_size().to_string()), "fs-4");
                    }
                    FontSize::Medium => {
                        a.alter_classes(ClassesOp::Replace(a.font_size().to_string()), "fs-5");
                    }
                    _ => {}
                };
            }
            COMPONENT_BASE_HEADING => {
                let h = component_as_mut::<Heading>(component);
                let original = h.display().to_string();
                h.alter_classes(
                    ClassesOp::SetDefault,
                    match h.display() {
                        HeadingDisplay::ExtraLarge => "display-1",
                        HeadingDisplay::XxLarge => "display-2",
                        HeadingDisplay::XLarge => "display-3",
                        HeadingDisplay::Large => "display-4",
                        HeadingDisplay::Medium => "display-5",
                        _ => original.as_str(),
                    },
                );
            }
            COMPONENT_BASE_PARAGRAPH => {
                let p = component_as_mut::<Paragraph>(component);
                match p.font_size() {
                    FontSize::ExtraLarge => {
                        p.alter_classes(ClassesOp::Replace(p.font_size().to_string()), "fs-1");
                    }
                    FontSize::XxLarge => {
                        p.alter_classes(ClassesOp::Replace(p.font_size().to_string()), "fs-2");
                    }
                    FontSize::XLarge => {
                        p.alter_classes(ClassesOp::Replace(p.font_size().to_string()), "fs-3");
                    }
                    FontSize::Large => {
                        p.alter_classes(ClassesOp::Replace(p.font_size().to_string()), "fs-4");
                    }
                    FontSize::Medium => {
                        p.alter_classes(ClassesOp::Replace(p.font_size().to_string()), "fs-5");
                    }
                    _ => {}
                };
            }
            _ => {}
        }
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
