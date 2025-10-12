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
        .with_title(L10n::l("welcome_title"))
        .add_component(
            Intro::new()
                .add_component(
                    Block::new()
                        .with_title(L10n::l("welcome_status_title"))
                        .add_component(Html::with(move |cx| {
                            html! {
                                p { (L10n::l("welcome_status_1").using(cx)) }
                                p { (L10n::l("welcome_status_2").using(cx)) }
                            }
                        })),
                )
                .add_component(
                    Block::new()
                        .with_title(L10n::l("welcome_support_title"))
                        .add_component(Html::with(move |cx| {
                            html! {
                                p { (L10n::l("welcome_support_1").using(cx)) }
                                p { (L10n::l("welcome_support_2").with_arg("app", app).using(cx)) }
                            }
                        })),
                ),
        )
        .render()
}
