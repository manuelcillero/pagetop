use crate::prelude::*;

localize!("src/base/module/demopage/locales");

pub struct DemopageModule;

impl ModuleTrait for DemopageModule {
    fn name(&self) -> &'static str {
        "Demopage"
    }

    fn fullname(&self) -> String {
        l("module_fullname")
    }

    fn description(&self) -> Option<String> {
        Some(l("module_description"))
    }

    fn configure_module(&self, cfg: &mut app::web::ServiceConfig) {
        cfg.route("/", app::web::get().to(demo));
    }
}

async fn demo() -> app::Result<Markup> {
    Page::new()
        .using_theme("Bulmix")
        .with_title(l("page_title").as_str())
        .add_to("content", hello_world())
        .add_to("content", hello_world_original())
        .add_to("content", just_visiting())
        .add_to("content", about_pagetop())
        .add_to("content", promo_pagetop())
        .add_to("content", reporting_problems())
        .render()
}

fn hello_world() -> ArcComponent {
    Container::header()
        .add(grid::Row::new()
            .add_column(grid::Column::new()
                .add(Chunck::with(html! {
                    div class="section-title" {
                        (t("welcome_to", &args![
                            "app" => SETTINGS.app.name.as_str()
                        ]))
                    }
                    h1 class="h1-large" {
                        (l("page_title"))
                    }
                    p class="p-large" {
                        (e("welcome_intro", &args![
                            "app" => format!(
                                "<strong>{}</strong>",
                                &SETTINGS.app.name
                            )
                        ]))
                    }
                    p {
                        (e("welcome_pagetop", &args![
                            "pagetop" => "<a href=\"https://pagetop-rs\">PageTop</a>"
                        ]))
                    }
                    a class="btn-solid-lg" href="#services" {
                        "Offered services"
                    }
                    a class="quote" href="#contact" {
                        i class="fas fa-paper-plane" {}
                        "Get quote"
                    }
                }).arc())
            )
            .add_column(grid::Column::new()
                .add(Image::image("/bootsier/images/demo-header.svg").arc())
            )
            .arc()
        )
        .arc()
}

fn hello_world_original() -> ArcComponent {
    Chunck::with(html! {
        header id="header" class="header" {
            div class="container" {
                div class="row" {
                    div class="col-lg-6 col-xl-5" {
                        div class="text-container" {
                            div class="section-title" {
                                (t("welcome_to", &args![
                                    "app" => SETTINGS.app.name.as_str()
                                ]))
                            }
                            h1 class="h1-large" {
                                (l("page_title"))
                            }
                            p class="p-large" {
                                (e("welcome_intro", &args![
                                    "app" => format!(
                                        "<strong>{}</strong>",
                                        &SETTINGS.app.name
                                    )
                                ]))
                            }
                            p {
                                (e("welcome_pagetop", &args![
                                    "pagetop" => "<a href=\"https://pagetop-rs\">PageTop</a>"
                                ]))
                            }
                            a class="btn-solid-lg" href="#services" {
                                "Offered services"
                            }
                            a class="quote" href="#contact" {
                                i class="fas fa-paper-plane" {}
                                "Get quote"
                            }
                        }
                    }
                    div class="col-lg-6 col-xl-7" {
                        div class="image-container" {
                            img class="img-fluid" src="/bootsier/images/demo-header.svg" alt="alternative" {}
                        }
                    }
                }
            }
        }
    }).arc()
}

fn just_visiting() -> ArcComponent {
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
    }).arc()
}

fn about_pagetop() -> ArcComponent {
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
    }).arc()
}

fn promo_pagetop() -> ArcComponent {
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
    }).arc()
}

fn reporting_problems() -> ArcComponent {
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
    }).arc()
}
