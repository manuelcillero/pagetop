use crate::prelude::*;

/// Página de bienvenida predeterminada de PageTop.
///
/// Esta extensión se instala por defecto y muestra una página en la ruta raíz (`/`) cuando no se ha
/// configurado ninguna página de inicio personalizada. Permite confirmar que el servidor está
/// funcionando correctamente.
pub struct Welcome;

impl Extension for Welcome {
    fn name(&self) -> L10n {
        L10n::l("welcome_extension_name")
    }

    fn description(&self) -> L10n {
        L10n::l("welcome_extension_description")
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/", service::web::get().to(homepage));
    }
}

async fn homepage(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    let app = &global::SETTINGS.app.name;

    Page::new(request)
        .with_theme("basic")
        .with_layout("intro")
        .with_title(L10n::l("welcome_title"))
        .with_description(L10n::l("welcome_intro").with_arg("app", app))
        .with_param("intro_button_text", L10n::l("welcome_powered"))
        .with_param("intro_button_link", "https://pagetop.cillero.es".to_owned())
        .add_component(Html::with(|cx| {
            html! {
                p { (L10n::l("welcome_text1").using(cx)) }
                p { (L10n::l("welcome_text2").using(cx)) }
            }
        }))
        .add_component(
            Block::new()
                .with_title(L10n::l("welcome_about"))
                .add_component(Html::with(move |cx| {
                    html! {
                        p { (L10n::l("welcome_pagetop").using(cx)) }
                        p { (L10n::l("welcome_issues1").using(cx)) }
                        p { (L10n::l("welcome_issues2").with_arg("app", app).using(cx)) }
                    }
                })),
        )
        .render()
}
