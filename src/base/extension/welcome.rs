use crate::prelude::*;

/// Página de bienvenida de PageTop.
///
/// Se registra automáticamente cuando la aplicación arranca sin extensión raíz. Muestra una página
/// de bienvenida de PageTop en la ruta raíz (`/`) usando el componente [`Intro`].
///
/// También puede incluirse explícitamente como dependencia de la extensión raíz o de cualquier otra
/// extensión dentro de la estructura de la aplicación.
///
/// Resulta útil en demos o para comprobar rápidamente que el servidor ha arrancado correctamente.
pub struct Welcome;

impl Extension for Welcome {
    fn name(&self) -> L10n {
        L10n::l("welcome_extension_name")
    }

    fn description(&self) -> L10n {
        L10n::l("welcome_extension_description")
    }

    fn configure_router(&self, router: Router) -> Router {
        router.route("/", web::get(home))
    }
}

async fn home(request: HttpRequest) -> Result<Markup, ErrorPage> {
    let app = &global::SETTINGS.app.name;

    Page::new(request)
        .with_title(L10n::l("welcome_title"))
        .with_child(
            Intro::new()
                .with_child(
                    Block::new()
                        .with_title(L10n::l("welcome_status_title"))
                        .with_child(Html::with(move |cx| {
                            html! {
                                p class="intro-text-lead" {
                                    (L10n::l("welcome_status_1").using(cx))
                                }
                                p class="intro-text-lead" {
                                    (L10n::l("welcome_status_2").using(cx))
                                }
                            }
                        })),
                )
                .with_child(
                    Block::new()
                        .with_title(L10n::l("welcome_support_title"))
                        .with_child(Html::with(move |cx| {
                            html! {
                                p class="intro-text-lead" {
                                    (L10n::l("welcome_support_1").using(cx))
                                }
                                p class="intro-text-lead" {
                                    (L10n::l("welcome_support_2").with_arg("app", app).using(cx))
                                }
                            }
                        })),
                ),
        )
        .render()
}
