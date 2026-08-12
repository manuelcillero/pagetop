use pagetop::prelude::*;

include_locales!(LOC from "examples/locale");

struct IntroColors;

#[async_trait]
impl Extension for IntroColors {
    fn configure_router(&self, router: Router) -> Router {
        router.route("/", web::get(intro_colors))
    }
}

async fn intro_colors(request: HttpRequest) -> Result<Markup, ErrorPage> {
    Page::new(request)
        .with_child(
            Intro::default()
                .with_opening(IntroOpening::Custom)
                .with_title(Lc::n("PageTop"))
                .with_slogan(Lc::t("colors_slogan", &LOC))
                .with_button(None::<(Lc, Route)>)
                .with_child(
                    Block::new()
                        .with_title(Lc::t("colors_block", &LOC).with_arg("n", "1"))
                        .with_child(Html::with(|cx| {
                            html! {
                                p { (Lc::t("colors_val_1", &LOC).using(cx)) }
                            }
                        })),
                )
                .with_child(
                    Block::new()
                        .with_title(Lc::t("colors_block", &LOC).with_arg("n", "2"))
                        .with_child(Html::with(|cx| {
                            html! {
                                p { (Lc::t("colors_val_2", &LOC).using(cx)) }
                            }
                        })),
                )
                .with_child(
                    Block::new()
                        .with_title(Lc::t("colors_block", &LOC).with_arg("n", "3"))
                        .with_child(Html::with(|cx| {
                            html! {
                                p { (Lc::t("colors_val_3", &LOC).using(cx)) }
                            }
                        })),
                )
                .with_child(
                    Block::new()
                        .with_title(Lc::t("colors_block", &LOC).with_arg("n", "4"))
                        .with_child(Html::with(|cx| {
                            html! {
                                p { (Lc::t("colors_val_4", &LOC).using(cx)) }
                            }
                        })),
                )
                .with_child(
                    Block::new()
                        .with_title(Lc::t("colors_block", &LOC).with_arg("n", "5"))
                        .with_child(Html::with(|cx| {
                            html! {
                                p { (Lc::t("colors_val_5", &LOC).using(cx)) }
                            }
                        })),
                )
                .with_child(
                    Block::new()
                        .with_title(Lc::t("colors_block", &LOC).with_arg("n", "6"))
                        .with_child(Html::with(|cx| {
                            html! {
                                p { (Lc::t("colors_val_6", &LOC).using(cx)) }
                            }
                        })),
                ),
        )
        .render()
        .await
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&IntroColors).await.run().await
}
