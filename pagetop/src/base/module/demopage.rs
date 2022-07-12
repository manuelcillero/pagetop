use crate::prelude::*;

pub const DEMOPAGE_MODULE: &str = "pagetop::module::demopage";

localize!("src/base/module/demopage/locales");

pub struct Demopage;

impl ModuleTrait for Demopage {
    fn handler(&self) -> &'static str {
        DEMOPAGE_MODULE
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

async fn demo() -> app::Result<Markup> {
    Page::new()
        .with_title(l("page_title").as_str())
        .add_to("content", hello_world())
        .add_to("content", just_visiting())
        .add_to("content", about_pagetop())
        .add_to("content", promo_pagetop())
        .add_to("content", reporting_problems())
        .render()
}

fn hello_world() -> Container {
    Container::header()
        .with_id("hello-world")
        .with_component(grid::Row::new()
            .with_layout(
                &[LayoutSet::PaddingSide(UnitValue::RelEm(2.0), UnitValue::RelPct(5.0))]
            )
            .with_column(grid::Column::new()
                .with_size(grid::ColumnSize::Is4of12)
                .with_component(Heading::h1(html! {
                        (l("page_title"))
                    })
                    .with_display(HeadingDisplay::Medium)
                )
                .with_component(Paragraph::with(html! {
                        (t("welcome_to", &args!["app" => SETTINGS.app.name.as_str()]))
                    })
                )
                .with_component(Paragraph::with(html! {
                        (e("welcome_intro", &args![
                            "app" => format!("<strong>{}</strong>", &SETTINGS.app.name)
                        ]))
                    })
                    .with_display(ParagraphDisplay::Small)
                )
                .with_component(Paragraph::with(html! {
                        (e("welcome_pagetop", &args![
                            "pagetop" => "<a href=\"https://pagetop-rs\">PageTop</a>"
                        ]))
                    })
                )
                .with_component(Anchor::button("#", html! {
                        ("Offered services")
                    })
                    .with_left_icon(Icon::with("card-checklist"))
                    .with_layout(&[
                        LayoutSet::PaddingSide(UnitValue::UnSet, UnitValue::RelEm(1.5)),
                        LayoutSet::RadiusAll(UnitValue::RelEm(1.5)),
                    ])
                )
                .with_component(Anchor::button("#", html! {
                        ("Get quote")
                    })
                    .with_left_icon(Icon::with("envelope-open-heart-fill"))
                )
            )
            .with_column(grid::Column::new()
                .with_component(Image::image("/theme/images/demo-header.svg"))
            )
        )
}

fn just_visiting() -> Container {
    Container::new()
        .with_id("visiting")
        .with_component(grid::Row::new()
            .with_layout(
                &[LayoutSet::PaddingSide(UnitValue::RelEm(1.0), UnitValue::RelPct(5.0))]
            )
            .with_column(grid::Column::new()
                .with_layout(&[LayoutSet::PaddingAll(UnitValue::RelPct(2.0))])
                .with_size(grid::ColumnSize::Is5of12)
                .with_component(Image::image("/theme/images/demo-visiting.svg"))
            )
            .with_column(grid::Column::new()
                .with_layout(&[
                    LayoutSet::PaddingTop(UnitValue::RelPct(2.5)),
                    LayoutSet::PaddingLeft(UnitValue::RelPct(5.0)),
                ])
                .with_component(Heading::h2(html! {
                        (l("visiting_title"))
                    })
                )
                .with_component(Heading::h3(html! {
                        (l("visiting_subtitle"))
                    })
                    .with_display(HeadingDisplay::Subtitle)
                )
                .with_component(Paragraph::with(html! {
                        (l("visiting_text1"))
                    })
                    .with_display(ParagraphDisplay::Small)
                )
                .with_component(Paragraph::with(html! { (l("visiting_text2")) }))
            )
        )
}

fn about_pagetop() -> Container {
    Container::new()
        .with_id("pagetop")
        .with_component(grid::Row::new()
            .with_layout(
                &[LayoutSet::PaddingSide(UnitValue::RelEm(1.0), UnitValue::RelPct(5.0))]
            )
            .with_column(grid::Column::new()
                .with_layout(&[
                    LayoutSet::PaddingTop(UnitValue::RelPct(2.5)),
                    LayoutSet::PaddingLeft(UnitValue::RelPct(5.0)),
                ])
                .with_size(grid::ColumnSize::Is7of12)
                .with_component(Heading::h2(html! {
                        (l("pagetop_title"))
                    })
                )
                .with_component(Paragraph::with(html! {
                        (l("pagetop_text1"))
                    })
                    .with_display(ParagraphDisplay::Small)
                )
                .with_component(Paragraph::with(html! {
                        (l("pagetop_text2"))
                    })
                )
                .with_component(Paragraph::with(html! {
                        (l("pagetop_text3"))
                    })
                )
            )
            .with_column(grid::Column::new()
                .with_layout(&[LayoutSet::PaddingAll(UnitValue::RelPct(2.0))])
                .with_component(Image::image("/theme/images/demo-pagetop.svg"))
            )
        )
}

fn promo_pagetop() -> Container {
    Container::new()
        .with_id("promo")
        .with_component(grid::Row::new()
            .with_layout(
                &[LayoutSet::PaddingSide(UnitValue::RelEm(1.0), UnitValue::RelPct(5.0))]
            )
            .with_column(grid::Column::new()
                .with_layout(&[LayoutSet::PaddingAll(UnitValue::RelPct(2.0))])
                .with_size(grid::ColumnSize::Is5of12)
                .with_component(Image::image("/theme/images/demo-pagetop.svg"))
            )
            .with_column(grid::Column::new()
                .with_layout(&[
                    LayoutSet::PaddingTop(UnitValue::RelPct(2.5)),
                    LayoutSet::PaddingLeft(UnitValue::RelPct(5.0)),
                ])
                .with_component(Heading::h2(html! {
                        (l("pagetop_promo_title"))
                    })
                )
                .with_component(Paragraph::with(html! {
                        (e("pagetop_promo_text1", &args![
                            "pagetop" => "<a href=\"https://pagetop-rs\">PageTop</a>"
                        ]))
                    })
                    .with_display(ParagraphDisplay::Small)
                )
            )
        )
}

fn reporting_problems() -> Container {
    Container::new()
        .with_id("reporting")
        .with_component(grid::Row::new()
            .with_layout(
                &[LayoutSet::PaddingSide(UnitValue::RelEm(1.0), UnitValue::RelPct(5.0))]
            )
            .with_column(grid::Column::new()
                .with_layout(&[
                    LayoutSet::PaddingTop(UnitValue::RelPct(2.5)),
                    LayoutSet::PaddingLeft(UnitValue::RelPct(5.0)),
                ])
                .with_size(grid::ColumnSize::Is7of12)
                .with_component(Heading::h2(html! {
                        (l("report_problems_title"))
                    })
                )
                .with_component(Paragraph::with(html! {
                        (l("report_problems_text1"))
                    })
                    .with_display(ParagraphDisplay::Small)
                )
                .with_component(Paragraph::with(html! {
                        (l("report_problems_text2"))
                    })
                )
            )
            .with_column(grid::Column::new()
                .with_layout(&[LayoutSet::PaddingAll(UnitValue::RelPct(2.0))])
                .with_component(Image::image("/theme/images/demo-pagetop.svg"))
            )
        )
}
