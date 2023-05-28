use pagetop::prelude::*;
use pagetop_minimal::component::*;

define_handle!(MODULE_DEMOHOME);

define_locale!(LOCALE_DEMOHOME, "src/locales");

include!(concat!(env!("OUT_DIR"), "/homedemo.rs"));

pub struct HomeDemo;

impl ModuleTrait for HomeDemo {
    fn handle(&self) -> Handle {
        MODULE_DEMOHOME
    }

    fn name(&self) -> String {
        _t("module_name", Locale::From(&LOCALE_DEMOHOME))
    }

    fn description(&self) -> Option<String> {
        Some(_t("module_description", Locale::From(&LOCALE_DEMOHOME)))
    }

    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![&pagetop_minimal::Minimal]
    }

    fn configure_service(&self, cfg: &mut server::web::ServiceConfig) {
        serve_static_files!(cfg, "/homedemo", bundle_homedemo);
        cfg.route("/", server::web::get().to(demo));
    }
}

async fn demo(request: server::HttpRequest) -> ResultPage<Markup, FatalError> {
    Page::new(request)
        .with_title(L10n::t("page_title", &LOCALE_DEMOHOME))
        .with_context(ContextOp::AddStyleSheet(StyleSheet::located(
            "/homedemo/css/styles.css",
        )))
        .with_body_classes(ClassesOp::AddFirst, "default-homepage")
        .with_this_in("region-content", hello_world())
        .with_this_in("region-content", welcome())
        .with_this_in("region-content", about_pagetop())
        .with_this_in("region-content", promo_pagetop())
        .with_this_in("region-content", reporting_issues())
        .render()
}

fn hello_world() -> Container {
    Container::header().with_id("hello-world").with_component(
        grid::Row::new()
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "hello-col-text")
                    .with_size(grid::ColumnSize::Is5of12)
                    .with_component(
                        Heading::h1(L10n::t("page_title", &LOCALE_DEMOHOME))
                            .with_display(HeadingDisplay::Medium),
                    )
                    .with_component(
                        Paragraph::with(L10n::e("hello_intro", &LOCALE_DEMOHOME).with_arg(
                            "app",
                            format!(
                                "<span class=\"app-name\">{}</span>",
                                &config::SETTINGS.app.name,
                            ),
                        ))
                        .with_display(ParagraphDisplay::Small),
                    )
                    .with_component(Paragraph::with(
                        L10n::e("hello_powered", &LOCALE_DEMOHOME).with_arg(
                            "pagetop",
                            format!(
                                "<a href=\"{}\" target=\"_blank\">{}</a>",
                                "https://pagetop.cillero.es", "PageTop",
                            ),
                        ),
                    ))
                    .with_component(
                        Anchor::button(
                            "https://github.com/manuelcillero/pagetop",
                            L10n::t("hello_code", &LOCALE_DEMOHOME),
                        )
                        .with_target(AnchorTarget::Blank)
                        .with_left_icon(Icon::with("git"))
                        .with_classes(ClassesOp::Add, "code-link"),
                    )
                    .with_component(
                        Anchor::link("#welcome", L10n::t("hello_welcome", &LOCALE_DEMOHOME))
                            .with_left_icon(Icon::with("arrow-down-circle-fill"))
                            .with_classes(ClassesOp::Add, "welcome-link"),
                    ),
            )
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "hello-col-image")
                    .with_component(Image::with("/homedemo/images/header.svg")),
            ),
    )
}

fn welcome() -> Container {
    Container::section()
        .with_id("welcome")
        .with_classes(ClassesOp::Add, "welcome-col-text")
        .with_component(Heading::h2(L10n::t("welcome_page", &LOCALE_DEMOHOME)))
        .with_component(
            Heading::h3(L10n::e("welcome_subtitle", &LOCALE_DEMOHOME).with_arg(
                "app",
                format!(
                    "<span class=\"app-name\">{}</span>",
                    &config::SETTINGS.app.name
                ),
            ))
            .with_display(HeadingDisplay::Subtitle),
        )
        .with_component(
            Paragraph::with(L10n::t("welcome_text1", &LOCALE_DEMOHOME))
                .with_display(ParagraphDisplay::Small),
        )
        .with_component(Paragraph::with(L10n::t("welcome_text2", &LOCALE_DEMOHOME)))
}

fn about_pagetop() -> Container {
    Container::new().with_id("pagetop").with_component(
        grid::Row::new()
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "pagetop-col-image")
                    .with_size(grid::ColumnSize::Is5of12)
                    .with_component(Image::with("/homedemo/images/about.svg")),
            )
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "pagetop-col-text")
                    .with_component(Heading::h2(L10n::t("pagetop_title", &LOCALE_DEMOHOME)))
                    .with_component(
                        Paragraph::with(L10n::t("pagetop_text1", &LOCALE_DEMOHOME))
                            .with_display(ParagraphDisplay::Small),
                    )
                    .with_component(Paragraph::with(L10n::t("pagetop_text2", &LOCALE_DEMOHOME)))
                    .with_component(Paragraph::with(
                        L10n::e("pagetop_text3", &LOCALE_DEMOHOME).with_arg(
                            "pagetop_website",
                            format!(
                                "<a href=\"{}\" target=\"_blank\">{}</a>",
                                "https://docs.rs/pagetop/latest/pagetop",
                                _t("pagetop_website", Locale::From(&LOCALE_DEMOHOME)),
                            ),
                        ),
                    )),
            ),
    )
}

fn promo_pagetop() -> Container {
    Container::new().with_id("promo").with_component(
        grid::Row::new()
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "promo-col-text")
                    .with_component(Heading::h2(L10n::t(
                        "pagetop_promo_title",
                        &LOCALE_DEMOHOME,
                    )))
                    .with_component(
                        Paragraph::with(L10n::e("pagetop_promo_text1", &LOCALE_DEMOHOME).with_arg(
                            "pagetop",
                            format!(
                                "<a href=\"{}\" target=\"_blank\">{}</a>",
                                "https://crates.io/crates/pagetop", "PageTop",
                            ),
                        ))
                        .with_display(ParagraphDisplay::Small),
                    ),
            )
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "promo-col-image")
                    .with_size(grid::ColumnSize::Is6of12)
                    .with_component(Image::with("/homedemo/images/pagetop.png")),
            ),
    )
}

fn reporting_issues() -> Container {
    Container::new().with_id("reporting").with_component(
        grid::Row::new()
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "reporting-col-image")
                    .with_component(Image::with("/homedemo/images/support.jpg")),
            )
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "reporting-col-text")
                    .with_size(grid::ColumnSize::Is6of12)
                    .with_component(Heading::h2(L10n::t(
                        "report_problems_title",
                        &LOCALE_DEMOHOME,
                    )))
                    .with_component(
                        Paragraph::with(L10n::t("report_problems_text1", &LOCALE_DEMOHOME))
                            .with_display(ParagraphDisplay::Small),
                    )
                    .with_component(Paragraph::with(L10n::t(
                        "report_problems_text2",
                        &LOCALE_DEMOHOME,
                    ))),
            ),
    )
}
