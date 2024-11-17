use crate::html::{html, Markup};
use crate::locale::L10n;
use crate::{concat_string, global};

pub async fn homepage() -> Markup {
    html! {
        head {
            meta charset="UTF-8" {}
            meta name="viewport" content="width=device-width, initial-scale=1" {}
            title { (concat_string!(
                &global::SETTINGS.app.name, " | ", L10n::l("welcome_page").to_string()
            )) }
            style { r#"
                body {
                    background-color: #f3d060;
                    font-size: 20px;
                }
                .wrapper {
                    max-width: 1200px;
                    width: 100%;
                    margin: 0 auto;
                    padding: 0;
                }
                .container {
                    padding: 0 16px;
                }
                .title {
                    font-size: clamp(3rem, 10vw, 10rem);
                    letter-spacing: -0.05em;
                    line-height: 1.2;
                    margin: 0;
                }
                .subtitle {
                    font-size: clamp(1.8rem, 2vw, 3rem);
                    letter-spacing: -0.02em;
                    line-height: 1.2;
                    margin: 0;
                }
                .powered {
                    margin: .5em 0 1em;
                }
                .box-container {
                    display: flex;
                    flex-wrap: wrap;
                    justify-content: space-between;
                    align-items: stretch;
                    gap: 1.5em;
                }
                .box {
                    flex: 1 1 280px;
                    border: 3px solid #25282a;
                    box-shadow: 5px 5px 0px #25282a;
                    box-sizing: border-box;
                    padding: 0 16px;
                }
                footer {
                    margin-top: 5em;
                    font-size: 14px;
                    font-weight: 500;
                    color: #a5282c;
                }
            "# }
        }
        body style="font-family: -apple-system, BlinkMacSystemFont, \"Segoe UI\", Roboto, \"Helvetica Neue\", Arial, sans-serif;" {
            div class="wrapper" {
                div class="container" {
                    h1 class="title" { (L10n::l("welcome_title").markup()) }

                    p class="subtitle" {
                        (L10n::l("welcome_intro").with_arg("app", format!(
                            "<span style=\"font-weight: bold;\">{}</span>",
                            &global::SETTINGS.app.name
                        )).markup())
                    }
                    p class="powered" {
                        (L10n::l("welcome_powered").with_arg("pagetop", format!(
                            "<a href=\"{}\" target=\"_blank\">{}</a>",
                            "https://crates.io/crates/pagetop", "PageTop"
                        )).markup())
                    }

                    h2 { (L10n::l("welcome_page").markup()) }

                    div class="box-container" {
                        section class="box" style="background-color: #5eb0e5;" {
                            h3 {
                                (L10n::l("welcome_subtitle")
                                    .with_arg("app", &global::SETTINGS.app.name)
                                    .markup())
                            }
                            p { (L10n::l("welcome_text1").markup()) }
                            p { (L10n::l("welcome_text2").markup()) }
                        }
                        section class="box" style="background-color: #aee1cd;" {
                            h3 {
                                (L10n::l("welcome_pagetop_title").markup())
                            }
                            p { (L10n::l("welcome_pagetop_text1").markup()) }
                            p { (L10n::l("welcome_pagetop_text2").markup()) }
                            p { (L10n::l("welcome_pagetop_text3").markup()) }
                        }
                        section class="box" style="background-color: #ebebe3;" {
                            h3 {
                                (L10n::l("welcome_issues_title").markup())
                            }
                            p { (L10n::l("welcome_issues_text1").markup()) }
                            p {
                                (L10n::l("welcome_issues_text2")
                                    .with_arg("app", &global::SETTINGS.app.name)
                                    .markup())
                            }
                        }
                    }

                    footer { "[" (L10n::l("welcome_have_fun").markup()) "]" }
                }
            }
        }
    }
}
