use pagetop::prelude::*;

pub_handle!(THEME_BOOTSIER);

pub_locale!("locales");

include!(concat!(env!("OUT_DIR"), "/bootsier.rs"));

pub struct Bootsier;

impl ModuleTrait for Bootsier {
    fn handle(&self) -> Handle {
        THEME_BOOTSIER
    }

    fn theme(&self) -> Option<ThemeStaticRef> {
        Some(&Bootsier)
    }

    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![&pagetop_jquery::JQuery]
    }

    fn configure_service(&self, cfg: &mut server::web::ServiceConfig) {
        serve_static_files!(cfg, "/bootsier", bundle_bootsier);
    }
}

impl ThemeTrait for Bootsier {
    fn before_render_page(&self, page: &mut Page) {
        page.alter_favicon(Some(Favicon::new().with_icon("/theme/favicon.ico")))
            .alter_context(ContextOp::AddStyleSheet(
                StyleSheet::located("/bootsier/css/bootstrap.min.css")
                    .with_version("5.1.3")
                    .with_weight(-99),
            ))
            .alter_context(ContextOp::AddJavaScript(
                JavaScript::located("/bootsier/js/bootstrap.bundle.min.js")
                    .with_version("5.1.3")
                    .with_weight(-99),
            ));
        pagetop_jquery::JQuery::add_jquery(page.context());
    }

    fn error_404_not_found(&self) -> Container {
        Container::new().with_component(Html::with(html! {
            div class="jumbotron" {
                div class="media" {
                    img
                        src="/bootsier/images/caution.png"
                        class="mr-4"
                        style="width: 20%; max-width: 188px"
                        alt="Caution!";
                    div class="media-body" {
                        h1 class="display-4" { ("RESOURCE NOT FOUND") }
                        p class="lead" { (l("e404-description")) }
                        hr class="my-4";
                        p { (l("e404-description")) }
                        a
                            class="btn btn-primary btn-lg"
                            href="/"
                            role="button"
                        {
                            (l("back-homepage"))
                        }
                    }
                }
            }
        }))
    }
}
