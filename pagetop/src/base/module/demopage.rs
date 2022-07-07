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
        .add(grid::Row::new()
            .add_column(grid::Column::new()
                .add(Heading::h1(html! {
                    (l("page_title"))
                }).with_display(HeadingDisplay::Large))
                .add(Paragraph::with(html! {
                    (t("welcome_to", &args![
                        "app" => SETTINGS.app.name.as_str()
                    ]))
                }))
                .add(Paragraph::with(html! {
                    (e("welcome_intro", &args![
                        "app" => format!("<strong>{}</strong>", &SETTINGS.app.name)
                    ]))
                }).with_display(ParagraphDisplay::Small))
                .add(Paragraph::with(html! {
                    (e("welcome_pagetop", &args![
                        "pagetop" => "<a href=\"https://pagetop-rs\">PageTop</a>"
                    ]))
                }))
                .add(Anchor::button("#",
                    html! {
                        ("Offered services")
                    }).with_left_icon(
                        Icon::with("card-checklist")
                    )
                )
                .add(Anchor::button("#",
                    html! {
                        ("Get quote")
                    }).with_left_icon(
                        Icon::with("envelope-open-heart-fill")
                    )
                )
            )
            .add_column(grid::Column::new()
                .add(Image::image("/theme/images/demo-header.svg"))
            )
            .with_spaces(&[SpaceSet::PaddingBoth(SpaceValue::RelEm(2.0), SpaceValue::RelPct(5.0))])
        )
}

fn just_visiting() -> Chunck {
    Chunck::with(html! {
        div id="details" class="basic-1" {
            div class="container" {
                div class="row" {
                    div class="col-lg-6 col-xl-7" {
                        div class="image-container" {
                            img class="img-fluid" src="/bootsier/images/demo-visiting.svg" alt="alternative" {}
                        }
                    }
                    div class="col-lg-6 col-xl-5" {
                        div class="text-container" {
                            h2 {
                                span {
                                    (l("visiting_title"))
                                }
                                br;
                                (l("visiting_subtitle"))
                            }
                            p { (l("visiting_text1")) }
                            p { (l("visiting_text2")) }
                            a class="btn-solid-reg" data-bs-toggle="modal" data-bs-target="#staticBackdrop" { "Modal" }
                        }
                    }
                }
            }
        }
    })
}

fn about_pagetop() -> Chunck {
    Chunck::with(html! {
        div id="pagetop" class="basic-2" {
            div class="container" {
                div class="row" {
                    div class="col-lg-6 col-xl-5" {
                        div class="text-container" {
                            h2 { (l("pagetop_title")) }
                            p { (l("pagetop_text1")) }
                            p { (l("pagetop_text2")) }
                            p { (l("pagetop_text3")) }
                        }
                    }
                    div class="col-lg-6 col-xl-7" {
                        div class="image-container" {
                            img class="img-fluid" src="/bootsier/images/demo-pagetop.svg" alt="alternative" {}
                        }
                    }
                }
            }
        }
    })
}

fn promo_pagetop() -> Chunck {
    Chunck::with(html! {
        div id="promo" class="basic-3" {
            div class="container" {
                div class="row" {
                    div class="col-lg-6 col-xl-5" {
                        div class="text-container" {
                            h2 { (l("pagetop_promo_title")) }
                            p  { (e("pagetop_promo_text1", &args![
                                "pagetop" =>
                                    "<a href=\"https://pagetop-rs\">PageTop</a>"
                            ])) }
                        }
                    }
                    div class="col-lg-6 col-xl-7" {
                        div class="image-container" {
                            img class="img-fluid" src="/bootsier/images/demo-pagetop.svg" alt="alternative" {}
                        }
                    }
                }
            }
        }
    })
}

fn reporting_problems() -> Chunck {
    Chunck::with(html! {
        div id="reporting" class="basic-4" {
            div class="container" {
                div class="row" {
                    div class="col-lg-6 col-xl-5" {
                        div class="text-container" {
                            h2 { (l("report_problems_title")) }
                            p { (l("report_problems_text1")) }
                            p { (l("report_problems_text2")) }
                        }
                    }
                    div class="col-lg-6 col-xl-7" {
                        div class="image-container" {
                            img class="img-fluid" src="/bootsier/images/demo-pagetop.svg" alt="alternative" {}
                        }
                    }
                }
            }
        }
    })
}
