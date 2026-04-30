use pagetop::prelude::*;

include_locales!(LOC from "examples/locale");

struct IntroColors;

impl Extension for IntroColors {
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/", service::web::get().to(intro_colors));
    }
}

async fn intro_colors(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Page::new(request)
        .with_child(
            Intro::default()
                .with_opening(IntroOpening::Custom)
                .with_title(L10n::n("PageTop"))
                .with_slogan(L10n::t("colors_slogan", &LOC))
                .with_button(None::<(L10n, FnPathByContext)>)
                .with_child(
                    Block::new()
                        .with_title(L10n::t("colors_block", &LOC).with_arg("n", "1"))
                        .with_child(Html::with(|cx| {
                            html! {
                                p { (L10n::t("colors_val_1", &LOC).using(cx)) }
                            }
                        })),
                )
                .with_child(
                    Block::new()
                        .with_title(L10n::t("colors_block", &LOC).with_arg("n", "2"))
                        .with_child(Html::with(|cx| {
                            html! {
                                p { (L10n::t("colors_val_2", &LOC).using(cx)) }
                            }
                        })),
                )
                .with_child(
                    Block::new()
                        .with_title(L10n::t("colors_block", &LOC).with_arg("n", "3"))
                        .with_child(Html::with(|cx| {
                            html! {
                                p { (L10n::t("colors_val_3", &LOC).using(cx)) }
                            }
                        })),
                )
                .with_child(
                    Block::new()
                        .with_title(L10n::t("colors_block", &LOC).with_arg("n", "4"))
                        .with_child(Html::with(|cx| {
                            html! {
                                p { (L10n::t("colors_val_4", &LOC).using(cx)) }
                            }
                        })),
                )
                .with_child(
                    Block::new()
                        .with_title(L10n::t("colors_block", &LOC).with_arg("n", "5"))
                        .with_child(Html::with(|cx| {
                            html! {
                                p { (L10n::t("colors_val_5", &LOC).using(cx)) }
                            }
                        })),
                )
                .with_child(
                    Block::new()
                        .with_title(L10n::t("colors_block", &LOC).with_arg("n", "6"))
                        .with_child(Html::with(|cx| {
                            html! {
                                p { (L10n::t("colors_val_6", &LOC).using(cx)) }
                            }
                        })),
                ),
        )
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&IntroColors).run()?.await
}
