use pagetop::prelude::*;

new_handle!(MODULE_HOMEDEMO);

static_locales!(LOCALES_HOMEDEMO);

static_files!(homedemo);

pub struct HomeDemo;

impl ModuleTrait for HomeDemo {
    fn handle(&self) -> Handle {
        MODULE_HOMEDEMO
    }

    fn name(&self) -> L10n {
        L10n::t("module_name", &LOCALES_HOMEDEMO)
    }

    fn description(&self) -> L10n {
        L10n::t("module_description", &LOCALES_HOMEDEMO)
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        static_files_service!(scfg, "/homedemo", homedemo);
        scfg.route("/", service::web::get().to(demo));
    }
}

async fn demo(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    Page::new(request)
        .with_title(L10n::t("page_title", &LOCALES_HOMEDEMO))
        .with_context(ContextOp::AddStyleSheet(StyleSheet::at(
            "/homedemo/css/styles.css",
        )))
        .with_body_classes(ClassesOp::Add, "default-homepage")
        .with_in("content", hello_world())
        .with_in("content", welcome())
        .with_in("content", about_pagetop())
        .with_in("content", promo_pagetop())
        .with_in("content", reporting_issues())
        .render()
}

fn hello_world() -> Wrapper {
    Wrapper::header().with_id("hello-world").add_component(
        flex::Container::new()
            .with_direction(flex::Direction::Column(BreakPoint::MD))
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "hello-col-text")
                    .with_size(flex::ItemSize::Percent40)
                    .add_component(
                        Heading::h1(L10n::t("page_title", &LOCALES_HOMEDEMO))
                            .with_display(HeadingDisplay::Medium),
                    )
                    .add_component(
                        Paragraph::translated(L10n::t("hello_intro", &LOCALES_HOMEDEMO).with_arg(
                            "app",
                            format!(
                                "<span class=\"app-name\">{}</span>",
                                &config::SETTINGS.app.name,
                            ),
                        ))
                        .with_display(ParagraphDisplay::Small),
                    )
                    .add_component(Paragraph::translated(
                        L10n::t("hello_powered", &LOCALES_HOMEDEMO).with_arg(
                            "pagetop",
                            format!(
                                "<a href=\"{}\" target=\"_blank\">{}</a>",
                                "https://pagetop.cillero.es", "PageTop",
                            ),
                        ),
                    ))
                    .add_component(
                        Anchor::button(
                            "https://github.com/manuelcillero/pagetop",
                            L10n::t("hello_code", &LOCALES_HOMEDEMO),
                        )
                        .with_target(AnchorTarget::Blank)
                        .with_left_icon(Icon::with("git"))
                        .with_classes(ClassesOp::Add, "code-link"),
                    )
                    .add_component(
                        Anchor::link("#welcome", L10n::t("hello_welcome", &LOCALES_HOMEDEMO))
                            .with_left_icon(Icon::with("arrow-down-circle-fill"))
                            .with_classes(ClassesOp::Add, "welcome-link"),
                    ),
            )
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "hello-col-image")
                    .with_size(flex::ItemSize::Percent60)
                    .add_component(Image::with("/homedemo/images/header.svg")),
            ),
    )
}

fn welcome() -> Wrapper {
    Wrapper::section()
        .with_id("welcome")
        .with_classes(ClassesOp::Add, "welcome-col-text")
        .add_component(Heading::h2(L10n::t("welcome_page", &LOCALES_HOMEDEMO)))
        .add_component(
            Heading::h3(L10n::t("welcome_subtitle", &LOCALES_HOMEDEMO).with_arg(
                "app",
                format!(
                    "<span class=\"app-name\">{}</span>",
                    &config::SETTINGS.app.name
                ),
            ))
            .with_display(HeadingDisplay::Subtitle),
        )
        .add_component(
            Paragraph::translated(L10n::t("welcome_text1", &LOCALES_HOMEDEMO))
                .with_display(ParagraphDisplay::Small),
        )
        .add_component(Paragraph::translated(L10n::t(
            "welcome_text2",
            &LOCALES_HOMEDEMO,
        )))
}

fn about_pagetop() -> Wrapper {
    Wrapper::new().with_id("pagetop").add_component(
        flex::Container::new()
            .with_direction(flex::Direction::Column(BreakPoint::SM))
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "pagetop-col-image")
                    .with_size(flex::ItemSize::Percent40)
                    .add_component(Image::with("/homedemo/images/about.svg")),
            )
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "pagetop-col-text")
                    .add_component(Heading::h2(L10n::t("pagetop_title", &LOCALES_HOMEDEMO)))
                    .add_component(
                        Paragraph::translated(L10n::t("pagetop_text1", &LOCALES_HOMEDEMO))
                            .with_display(ParagraphDisplay::Small),
                    )
                    .add_component(Paragraph::translated(L10n::t(
                        "pagetop_text2",
                        &LOCALES_HOMEDEMO,
                    )))
                    .add_component(Paragraph::translated(
                        L10n::t("pagetop_text3", &LOCALES_HOMEDEMO)
                            .with_arg("href", "https://docs.rs/pagetop/latest/pagetop"),
                    )),
            ),
    )
}

fn promo_pagetop() -> Wrapper {
    Wrapper::new().with_id("promo").add_component(
        flex::Container::new()
            .with_direction(flex::Direction::Column(BreakPoint::MD))
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "promo-col-text")
                    .with_size(flex::ItemSize::Percent60)
                    .add_component(Heading::h2(L10n::t(
                        "pagetop_promo_title",
                        &LOCALES_HOMEDEMO,
                    )))
                    .add_component(
                        Paragraph::translated(
                            L10n::t("pagetop_promo_text1", &LOCALES_HOMEDEMO).with_arg(
                                "pagetop",
                                format!(
                                    "<a href=\"{}\" target=\"_blank\">{}</a>",
                                    "https://crates.io/crates/pagetop", "PageTop",
                                ),
                            ),
                        )
                        .with_display(ParagraphDisplay::Small),
                    ),
            )
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "promo-col-image")
                    .with_size(flex::ItemSize::Percent40)
                    .add_component(Image::with("/homedemo/images/pagetop.png")),
            ),
    )
}

fn reporting_issues() -> Wrapper {
    Wrapper::new().with_id("reporting").add_component(
        flex::Container::new()
            .with_direction(flex::Direction::Column(BreakPoint::MD))
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "reporting-col-image")
                    .add_component(Image::with("/homedemo/images/support.jpg")),
            )
            .add_item(
                flex::Item::new()
                    .with_inner_classes(ClassesOp::Add, "reporting-col-text")
                    .with_size(flex::ItemSize::Percent50)
                    .add_component(Heading::h2(L10n::t(
                        "report_problems_title",
                        &LOCALES_HOMEDEMO,
                    )))
                    .add_component(
                        Paragraph::translated(L10n::t("report_problems_text1", &LOCALES_HOMEDEMO))
                            .with_display(ParagraphDisplay::Small),
                    )
                    .add_component(Paragraph::translated(L10n::t(
                        "report_problems_text2",
                        &LOCALES_HOMEDEMO,
                    ))),
            ),
    )
}
