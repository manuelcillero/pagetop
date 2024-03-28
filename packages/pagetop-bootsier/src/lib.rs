use pagetop::prelude::*;

static_locales!(LOCALES_BOOTSIER);

static_files!(bootsier);

pub struct Bootsier;

impl PackageTrait for Bootsier {
    fn theme(&self) -> Option<ThemeRef> {
        Some(&Bootsier)
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![
            action::theme::BeforePrepare::<Icon>::new(&Self, before_prepare_icon),
            action::theme::BeforePrepare::<Button>::new(&Self, before_prepare_button),
            action::theme::BeforePrepare::<Heading>::new(&Self, before_prepare_heading),
            action::theme::BeforePrepare::<Paragraph>::new(&Self, before_prepare_paragraph),
            action::theme::RenderComponent::<Error404>::new(&Self, render_error404),
        ]
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
        Body::with(match page.context().layout() {
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
        })
        .render(page.context())
    }

    fn after_prepare_body(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/base/favicon.ico")))
            .alter_assets(AssetsOp::AddStyleSheet(
                StyleSheet::at("/bootsier/css/bootstrap.min.css")
                    .with_version("5.1.3")
                    .with_weight(-99),
            ))
            .alter_assets(AssetsOp::AddJavaScript(
                JavaScript::at("/bootsier/js/bootstrap.bundle.min.js")
                    .with_version("5.1.3")
                    .with_weight(-99),
            ))
            .alter_assets(AssetsOp::AddBaseAssets)
            .alter_assets(AssetsOp::AddStyleSheet(
                StyleSheet::at("/bootsier/css/styles.css").with_version("0.0.1"),
            ));
    }
}

fn before_prepare_icon(i: &mut Icon, _cx: &mut Context) {
    i.alter_classes(
        ClassesOp::Replace(i.font_size().to_string()),
        with_font(i.font_size()),
    );
}

#[rustfmt::skip]
fn before_prepare_button(b: &mut Button, _cx: &mut Context) {
    b.alter_classes(ClassesOp::Replace("button__tap".to_owned()), "btn");
    b.alter_classes(
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
    b.alter_classes(
        ClassesOp::Replace(b.font_size().to_string()),
        with_font(b.font_size()),
    );
}

#[rustfmt::skip]
fn before_prepare_heading(h: &mut Heading, _cx: &mut Context) {
    h.alter_classes(
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
    p.alter_classes(
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
}

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
