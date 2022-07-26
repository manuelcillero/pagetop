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
        .with_context(InContextOp::StyleSheet(AssetsOp::Add(StyleSheet::located(
            "/theme/module/homepage/styles.css",
        ))))
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
                    .with_size(grid::ColumnSize::Is4of12)
                    .with_component(
                        Heading::h1(html! {
                            (l("page_title"))
                        })
                        .with_display(HeadingDisplay::Medium),
                    )
                    .with_component(
                        Paragraph::with(html! {
                            (e("hello_intro", &args![
                                "app" => format!("<strong>{}</strong>", &SETTINGS.app.name)
                            ]))
                        })
                        .with_display(ParagraphDisplay::Small),
                    )
                    .with_component(Paragraph::with(html! {
                        (e("hello_pagetop", &args![
                            "pagetop" => "<a href=\"https://pagetop-rs\">PageTop</a>"
                        ]))
                    }))
                    .with_component(
                        Anchor::button(
                            "#",
                            html! {
                                ("Offered services")
                            },
                        )
                        .with_left_icon(Icon::with("card-checklist"))
                        .with_classes(ClassesOp::Add, "services-link"),
                    )
                    .with_component(
                        Anchor::button(
                            "#",
                            html! {
                                ("Get quote")
                            },
                        )
                        .with_left_icon(Icon::with("envelope-open-heart-fill")),
                    ),
            )
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "hello-col-image")
                    .with_component(Image::new_with_source("/theme/images/demo-header.svg")),
            ),
    )
}

fn welcome() -> Container {
    Container::section()
        .with_id("welcome")
        .with_classes(ClassesOp::Add, "welcome-col-text")
        .with_component(Heading::h2(html! {
            (t("welcome_to", &args!["app" => SETTINGS.app.name.as_str()]))
        }))
        .with_component(
            Heading::h3(html! {
                (l("welcome_subtitle"))
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
                    .with_component(Image::new_with_source("/theme/images/demo-about.svg")),
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
                        (l("pagetop_text3"))
                    })),
            ),
    )
}

fn promo_pagetop() -> Container {
    Container::new().with_id("promo").with_component(
        grid::Row::new()
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "promo-col-image")
                    .with_size(grid::ColumnSize::Is5of12)
                    .with_component(Image::new_with_source("/theme/images/demo-pagetop.svg")),
            )
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "promo-col-text")
                    .with_component(Heading::h2(html! {
                        (l("pagetop_promo_title"))
                    }))
                    .with_component(
                        Paragraph::with(html! {
                            (e("pagetop_promo_text1", &args![
                                "pagetop" => "<a href=\"https://pagetop-rs\">PageTop</a>"
                            ]))
                        })
                        .with_display(ParagraphDisplay::Small),
                    ),
            ),
    )
}

fn reporting_problems() -> Container {
    Container::new().with_id("reporting").with_component(
        grid::Row::new()
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "reporting-col-text")
                    .with_size(grid::ColumnSize::Is7of12)
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
            .with_column(
                grid::Column::new()
                    .with_classes(ClassesOp::Add, "reporting-col-image")
                    .with_component(Image::new_with_source("/theme/images/demo-pagetop.svg")),
            ),
    )
}
