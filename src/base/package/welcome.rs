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
        .with_assets(AssetsOp::LangId(lang))
        .with_assets(AssetsOp::AddStyleSheet(StyleSheet::from(
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

fn hello_world() -> flex::Container {
    flex::Container::header()
        .with_classes(ClassesOp::Add, "hello-world")
        .with_justify(flex::Justify::Center)
        .add_item(
            flex::Item::new()
                .with_size(flex::Size::Percent90)
                .add_component(
                    flex::Container::new()
                        .with_direction(flex::Direction::Column(BreakPoint::MD))
                        .add_item(
                            flex::Item::new()
                                .with_classes(ClassesOp::Add, "hello-col-text")
                                .with_size(flex::Size::Percent40)
                                .add_component(
                                    Heading::h1(L10n::l("welcome_title"))
                                        .with_size(HeadingSize::Medium),
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
                                .add_component(Paragraph::fluent(
                                    L10n::l("welcome_powered").with_arg(
                                        "pagetop",
                                        format!(
                                            "<a href=\"{}\" target=\"_blank\">{}</a>",
                                            "https://pagetop.cillero.es", "PageTop",
                                        ),
                                    ),
                                ))
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
                            flex::Item::with(Image::with("/base/images/header.svg"))
                                .with_classes(ClassesOp::Add, "hello-col-image")
                                .with_size(flex::Size::Percent60),
                        ),
                ),
        )
}

fn welcome() -> flex::Container {
    flex::Container::section()
        .with_id("welcome-page")
        .with_classes(ClassesOp::Add, "welcome")
        .with_justify(flex::Justify::Center)
        .add_item(
            flex::Item::new()
                .with_size(flex::Size::Percent80)
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
                .add_component(
                    Paragraph::fluent(L10n::l("welcome_text1")).with_font_size(FontSize::Medium),
                )
                .add_component(Paragraph::fluent(L10n::l("welcome_text2"))),
        )
}

fn about_pagetop() -> flex::Container {
    flex::Container::new()
        .with_classes(ClassesOp::Add, "pagetop")
        .with_justify(flex::Justify::Center)
        .add_item(
            flex::Item::new()
                .with_size(flex::Size::Percent90)
                .add_component(
                    flex::Container::new()
                        .with_direction(flex::Direction::Column(BreakPoint::SM))
                        .add_item(
                            flex::Item::with(Image::with("/base/images/about.svg"))
                                .with_classes(ClassesOp::Add, "pagetop-col-image")
                                .with_size(flex::Size::Percent40),
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
                ),
        )
}

fn promo_pagetop() -> flex::Container {
    flex::Container::new()
        .with_classes(ClassesOp::Add, "promo")
        .with_justify(flex::Justify::Center)
        .add_item(
            flex::Item::new()
                .with_size(flex::Size::Percent75)
                .add_component(
                    flex::Container::new()
                        .with_direction(flex::Direction::Column(BreakPoint::MD))
                        .add_item(
                            flex::Item::new()
                                .with_classes(ClassesOp::Add, "promo-col-text")
                                .with_size(flex::Size::Percent50)
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
                            flex::Item::with(Image::with("/base/images/pagetop.png"))
                                .with_classes(ClassesOp::Add, "promo-col-image")
                                .with_size(flex::Size::Percent50),
                        ),
                ),
        )
}

fn reporting_issues() -> flex::Container {
    flex::Container::new()
        .with_classes(ClassesOp::Add, "issues")
        .with_justify(flex::Justify::Center)
        .add_item(
            flex::Item::new()
                .with_size(flex::Size::Percent90)
                .add_component(
                    flex::Container::new()
                        .with_direction(flex::Direction::Column(BreakPoint::MD))
                        .add_item(
                            flex::Item::with(Image::with("/base/images/issues.jpg"))
                                .with_classes(ClassesOp::Add, "issues-col-image"),
                        )
                        .add_item(
                            flex::Item::new()
                                .with_classes(ClassesOp::Add, "issues-col-text")
                                .with_size(flex::Size::Percent50)
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
                ),
        )
}
