use crate::prelude::*;

localize!("src/module/homepage/locales");

pub struct HomepageModule;

impl ModuleTrait for HomepageModule {
    fn fullname(&self) -> String {
        l("module_fullname")
    }

    fn description(&self) -> Option<String> {
        Some(l("module_description"))
    }

    fn configure_module(&self, cfg: &mut app::web::ServiceConfig) {
        cfg.route("/", app::web::get().to(home));
    }
}

async fn home() -> app::Result<Markup> {
    Page::prepare()
        .with_title(
            l("page_title").as_str()
        )
        .add_to("content", Container::prepare()
            .with_id("welcome")
            .add(Chunck::markup(html! {
                h1 { (l("page_title")) }
                p  { (e("text_welcome", &args![
                    "app" => format!("<strong>{}</strong>", &SETTINGS.app.name),
                    "pagetop" => "<a href=\"https://pagetop-rs\">PageTop</a>"
                ])) }
            }))
        )
        .add_to("content", Container::prepare()
            .add(Container::row()
                .add(Container::column()
                    .with_id("visitors")
                    .add(Chunck::markup(html! {
                        h2 { (l("title_normal_user")) }
                        p  { (l("text1_normal_user")) }
                        p  { (l("text2_normal_user")) }
                    })))
                .add(Container::column()
                    .with_id("pagetop")
                    .add(Chunck::markup(html! {
                        h2 { (l("title_about_pagetop")) }
                        p  { (l("text1_about_pagetop")) }
                        p  { (l("text2_about_pagetop")) }

                        h2 { (l("title_promo_pagetop")) }
                        p  { (e("text1_promo_pagetop", &args![
                            "pagetop" =>
                                "<a href=\"https://pagetop-rs\">PageTop</a>"
                        ])) }
                    }))
                )
            )
        )
        .add_to("content", Container::prepare()
            .with_id("reporting")
            .add(Chunck::markup(html! {
                h2 { (l("title_report_problems")) }
                p  { (l("text1_report_problems")) }
                p  { (l("text2_report_problems")) }
            }))
        )
        .render()
}
