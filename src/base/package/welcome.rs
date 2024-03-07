use crate::prelude::*;

pub struct Welcome;

impl PackageTrait for Welcome {
    fn name(&self) -> L10n {
        L10n::l("welcome_package_name")
    }

    fn description(&self) -> L10n {
        L10n::l("welcome_package_description")
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/", service::web::get().to(home_page))
            .route("/{lang}", service::web::get().to(home_lang));
    }
}

async fn home_page(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    home(request, &LANGID_DEFAULT)
}

async fn home_lang(
    request: HttpRequest,
    path: service::web::Path<String>,
) -> ResultPage<Markup, ErrorPage> {
    match langid_for(path.into_inner()) {
        Ok(lang) => home(request, lang),
        _ => Err(ErrorPage::NotFound(request)),
    }
}

fn home(request: HttpRequest, lang: &'static LanguageIdentifier) -> ResultPage<Markup, ErrorPage> {
    Page::new(request)
        .with_title(L10n::l("welcome_title"))
        .with_context(ContextOp::LangId(lang))
        .with_context(ContextOp::AddStyleSheet(StyleSheet::at(
            "/base/css/welcome.css",
        )))
        .with_body_id("welcome")
        .with_component(hello_world())
        .with_component(welcome())
        .with_component(about_pagetop())
        .with_component(promo_pagetop())
        .with_component(reporting_issues())
        .render()
}

fn hello_world() -> Wrapper {
    Wrapper::header()
        .with_classes(ClassesOp::Add, "hello-world")
        .add_component(
            flex::Container::new()
                .with_direction(flex::Direction::Column(BreakPoint::MD))
                .add_item(
                    flex::Item::new()
                        .with_classes(ClassesOp::Add, "hello-col-text")
                        .with_size(flex::ItemSize::Percent40)
                        .add_component(
                            Heading::h1(L10n::l("welcome_title")).with_size(HeadingSize::Medium),
                        )
                        .add_component(
                            Paragraph::fluent(L10n::l("welcome_intro").with_arg(
                                "app",
                                format!(
                                    "<span class=\"app-name\">{}</span>",
                                    &config::SETTINGS.app.name,
                                ),
                            ))
                            .with_font_size(FontSize::Medium),
                        )
                        .add_component(Paragraph::fluent(L10n::l("welcome_powered").with_arg(
                            "pagetop",
                            format!(
                                "<a href=\"{}\" target=\"_blank\">{}</a>",
                                "https://pagetop.cillero.es", "PageTop",
                            ),
                        )))
                        .add_component(
                            Button::anchor(
                                "https://github.com/manuelcillero/pagetop",
                                L10n::l("welcome_code"),
                            )
                            .with_target(ButtonTarget::Blank)
                            .with_left_icon(Some(Icon::with("git")))
                            .with_classes(ClassesOp::Add, "code-link")
                            .with_font_size(FontSize::Medium),
                        )
                        .add_component(
                            Button::anchor("#welcome-page", L10n::l("welcome"))
                                .with_style(StyleBase::Link)
                                .with_left_icon(Some(Icon::with("arrow-down-circle-fill")))
                                .with_classes(ClassesOp::Add, "welcome-link")
                                .with_font_size(FontSize::Medium),
                        ),
                )
                .add_item(
                    flex::Item::new()
                        .with_classes(ClassesOp::Add, "hello-col-image")
                        .with_size(flex::ItemSize::Percent60)
                        .add_component(Image::with("/base/images/header.svg")),
                ),
        )
}

fn welcome() -> Wrapper {
    Wrapper::section()
        .with_id("welcome-page")
        .with_classes(ClassesOp::Add, "welcome")
        .add_component(Heading::h2(L10n::l("welcome_page")))
        .add_component(
            Heading::h3(L10n::l("welcome_subtitle").with_arg(
                "app",
                format!(
                    "<span class=\"app-name\">{}</span>",
                    &config::SETTINGS.app.name
                ),
            ))
            .with_size(HeadingSize::Subtitle),
        )
        .add_component(Paragraph::fluent(L10n::l("welcome_text1")).with_font_size(FontSize::Medium))
        .add_component(Paragraph::fluent(L10n::l("welcome_text2")))
}

fn about_pagetop() -> Wrapper {
    Wrapper::new()
        .with_classes(ClassesOp::Add, "pagetop")
        .add_component(
            flex::Container::new()
                .with_direction(flex::Direction::Column(BreakPoint::SM))
                .add_item(
                    flex::Item::new()
                        .with_classes(ClassesOp::Add, "pagetop-col-image")
                        .with_size(flex::ItemSize::Percent40)
                        .add_component(Image::with("/base/images/about.svg")),
                )
                .add_item(
                    flex::Item::new()
                        .with_classes(ClassesOp::Add, "pagetop-col-text")
                        .add_component(Heading::h2(L10n::l("welcome_pagetop_title")))
                        .add_component(
                            Paragraph::fluent(L10n::l("welcome_pagetop_text1"))
                                .with_font_size(FontSize::Medium),
                        )
                        .add_component(Paragraph::fluent(L10n::l("welcome_pagetop_text2")))
                        .add_component(Paragraph::fluent(L10n::l("welcome_pagetop_text3"))),
                ),
        )
}

fn promo_pagetop() -> Wrapper {
    Wrapper::new()
        .with_classes(ClassesOp::Add, "promo")
        .add_component(
            flex::Container::new()
                .with_direction(flex::Direction::Column(BreakPoint::MD))
                .add_item(
                    flex::Item::new()
                        .with_classes(ClassesOp::Add, "promo-col-text")
                        .with_size(flex::ItemSize::Percent50)
                        .add_component(Heading::h2(L10n::l("welcome_promo_title")))
                        .add_component(
                            Paragraph::fluent(L10n::l("welcome_promo_text1").with_arg(
                                "pagetop",
                                format!(
                                    "<a href=\"{}\" target=\"_blank\">{}</a>",
                                    "https://crates.io/crates/pagetop", "PageTop",
                                ),
                            ))
                            .with_font_size(FontSize::Medium),
                        ),
                )
                .add_item(
                    flex::Item::new()
                        .with_classes(ClassesOp::Add, "promo-col-image")
                        .with_size(flex::ItemSize::Percent50)
                        .add_component(Image::with("/base/images/pagetop.png")),
                ),
        )
}

fn reporting_issues() -> Wrapper {
    Wrapper::new()
        .with_classes(ClassesOp::Add, "issues")
        .add_component(
            flex::Container::new()
                .with_direction(flex::Direction::Column(BreakPoint::MD))
                .add_item(
                    flex::Item::new()
                        .with_classes(ClassesOp::Add, "issues-col-image")
                        .add_component(Image::with("/base/images/issues.jpg")),
                )
                .add_item(
                    flex::Item::new()
                        .with_classes(ClassesOp::Add, "issues-col-text")
                        .with_size(flex::ItemSize::Percent50)
                        .add_component(Heading::h2(L10n::l("welcome_issues_title")))
                        .add_component(
                            Paragraph::fluent(L10n::l("welcome_issues_text1"))
                                .with_font_size(FontSize::Medium),
                        )
                        .add_component(Paragraph::fluent(
                            L10n::l("welcome_issues_text2").with_arg(
                                "app",
                                format!(
                                    "<span class=\"app-name\">{}</span>",
                                    &config::SETTINGS.app.name,
                                ),
                            ),
                        )),
                ),
        )
}
