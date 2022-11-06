use crate::prelude::*;

pub_const_handler!(MODULE_DEFAULT_HOMEPAGE);

localize!("src/base/module/homepage/locales");

pub struct DefaultHomePage;

impl ModuleTrait for DefaultHomePage {
    fn handler(&self) -> Handler {
        MODULE_DEFAULT_HOMEPAGE
    }

    fn name(&self) -> String {
        l("module_name")
    }

    fn description(&self) -> Option<String> {
        Some(l("module_description"))
    }

    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {
        cfg.route("/", app::web::get().to(demo));
    }
}

async fn demo() -> ResultPage<Markup, FatalError> {
    Page::new()
        .with_title(l("page_title").as_str())
        .with_context(PageOp::AddStyleSheet(StyleSheet::located(
            "/theme/module/homepage/styles.css",
        )))
        .with_body_classes(ClassesOp::AddFirst, "default-homepage")
        .add_to("region-content", hello_world())
        .add_to("region-content", welcome())
        .add_to("region-content", about_pagetop())
        .add_to("region-content", promo_pagetop())
        .add_to("region-content", reporting_problems())
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
                        Heading::h1(html! {
                            (l("page_title"))
                        })
                        .with_display(HeadingDisplay::Medium),
                    )
                    .with_component(
                        Paragraph::with(html! {
                            (e("hello_intro", &args![
                                "app" => format!(
                                    "<span class=\"app-name\">{}</span>",
                                    &SETTINGS.app.name,
                                )
                            ]))
                        })
                        .with_display(ParagraphDisplay::Small),
                    )
                    .with_component(Paragraph::with(html! {
                        (e("hello_powered", &args![
                            "pagetop" => format!(
                                "<a href=\"{}\" target=\"_blank\">{}</a>",
                                "https://pagetop.cillero.es",
                                "PageTop",
                            )
                        ]))
                    }))
                    .with_component(
                        Anchor::button(
                            "https://github.com/manuelcillero/pagetop",
                            html! { (l("hello_code")) },
                        )
                        .with_target(AnchorTarget::Blank)
                        .with_left_icon(Icon::with("git"))
                        .with_classes(ClassesOp::Add, "code-link"),
                    )
                    .with_component(
                        Anchor::link(
                            "#welcome",
                            html! { (l("hello_welcome")) },
                        )
                        .with_left_icon(Icon::with("arrow-down-circle-fill"))
                        .with_classes(ClassesOp::Add, "welcome-link"),
                    ),
            )
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "hello-col-image")
                    .with_component(Image::new_with_source("/theme/images/homepage-header.svg")),
            )
    )
}

fn welcome() -> Container {
    Container::section()
        .with_id("welcome")
        .with_classes(ClassesOp::Add, "welcome-col-text")
        .with_component(Heading::h2(html! {
            (l("welcome_page"))
        }))
        .with_component(
            Heading::h3(html! {
                (e("welcome_subtitle", &args![
                    "app" => format!("<span class=\"app-name\">{}</span>", &SETTINGS.app.name)
                ]))
            })
            .with_display(HeadingDisplay::Subtitle),
        )
        .with_component(
            Paragraph::with(html! {
                (l("welcome_text1"))
            })
            .with_display(ParagraphDisplay::Small),
        )
        .with_component(Paragraph::with(html! { (l("welcome_text2")) }))
}

fn about_pagetop() -> Container {
    Container::new().with_id("pagetop").with_component(
        grid::Row::new()
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "pagetop-col-image")
                    .with_size(grid::ColumnSize::Is5of12)
                    .with_component(Image::new_with_source("/theme/images/homepage-about.svg")),
            )
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "pagetop-col-text")
                    .with_component(Heading::h2(html! {
                        (l("pagetop_title"))
                    }))
                    .with_component(
                        Paragraph::with(html! {
                            (l("pagetop_text1"))
                        })
                        .with_display(ParagraphDisplay::Small),
                    )
                    .with_component(Paragraph::with(html! {
                        (l("pagetop_text2"))
                    }))
                    .with_component(Paragraph::with(html! {
                        (e("pagetop_text3", &args![
                            "pagetop_website" => format!(
                                "<a href=\"{}\" target=\"_blank\">{}</a>",
                                "https://docs.rs/pagetop/latest/pagetop",
                                l("pagetop_website"),
                            )
                        ]))
                    })),
            )
    )
}

fn promo_pagetop() -> Container {
    Container::new().with_id("promo").with_component(
        grid::Row::new()
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "promo-col-text")
                    .with_component(Heading::h2(html! {
                        (l("pagetop_promo_title"))
                    }))
                    .with_component(
                        Paragraph::with(html! {
                            (e("pagetop_promo_text1", &args![
                                "pagetop" => format!(
                                    "<a href=\"{}\" target=\"_blank\">{}</a>",
                                    "https://crates.io/crates/pagetop",
                                    "PageTop",
                                )
                            ]))
                        })
                        .with_display(ParagraphDisplay::Small),
                    ),
            )
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "promo-col-image")
                    .with_size(grid::ColumnSize::Is6of12)
                    .with_component(Image::new_with_source("/theme/images/homepage-pagetop.png")),
            )
    )
}

fn reporting_problems() -> Container {
    Container::new().with_id("reporting").with_component(
        grid::Row::new()
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "reporting-col-image")
                    .with_component(Image::new_with_source("/theme/images/homepage-support.jpg")),
            )
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "reporting-col-text")
                    .with_size(grid::ColumnSize::Is6of12)
                    .with_component(Heading::h2(html! {
                        (l("report_problems_title"))
                    }))
                    .with_component(
                        Paragraph::with(html! {
                            (l("report_problems_text1"))
                        })
                        .with_display(ParagraphDisplay::Small),
                    )
                    .with_component(Paragraph::with(html! {
                        (l("report_problems_text2"))
                    })),
            )
    )
}
