use crate::prelude::*;

/// Página de bienvenida de PageTop.
///
/// Esta extensión se instala por defecto si el ajuste de configuración [`global::App::welcome`] es
/// `true`. Muestra una página de bienvenida de PageTop en la ruta raíz (`/`) o en `/lang/{lang}`,
/// siempre que `{lang}` sea un idioma soportado (si no, devuelve una página de error 404).
///
/// No obstante, cualquier extensión puede sobrescribir este comportamiento si utiliza estas mismas
/// rutas.
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

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        scfg.route("/", service::web::get().to(home_page))
            .route("/lang/{lang}", service::web::get().to(home_lang));
    }
}

async fn home_page(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
    let language = LangMatch::from_request(Some(&request));
    home(request, &language)
}

async fn home_lang(
    request: HttpRequest,
    path: service::web::Path<String>,
) -> ResultPage<Markup, ErrorPage> {
    let language = LangMatch::resolve(path.into_inner());
    match language {
        LangMatch::Found(_) => home(request, &language),
        _ => Err(ErrorPage::NotFound(request)),
    }
}

fn home(request: HttpRequest, language: &impl LangId) -> ResultPage<Markup, ErrorPage> {
    let app = &global::SETTINGS.app.name;

    Page::new(request)
        .with_title(L10n::l("welcome_title"))
        .with_langid(language)
        .add_child(
            Intro::new()
                .add_child(
                    Block::new()
                        .with_title(L10n::l("welcome_status_title"))
                        .add_child(Html::with(move |cx| {
                            html! {
                                p { (L10n::l("welcome_status_1").using(cx)) }
                                p { (L10n::l("welcome_status_2").using(cx)) }
                            }
                        })),
                )
                .add_child(
                    Block::new()
                        .with_title(L10n::l("welcome_support_title"))
                        .add_child(Html::with(move |cx| {
                            html! {
                                p { (L10n::l("welcome_support_1").using(cx)) }
                                p { (L10n::l("welcome_support_2").with_arg("app", app).using(cx)) }
                            }
                        })),
                ),
        )
        .render()
}
