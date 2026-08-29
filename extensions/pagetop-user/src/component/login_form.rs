//! Componente de formulario de inicio de sesión.

use pagetop::prelude::*;

use crate::component::error_banner;
use crate::config::SETTINGS;
use crate::{LOCALES_USER, LOGIN_PATH, PASSWORD_RESET_PATH, REGISTER_PATH};

#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct LoginForm {
    error: Option<Lc>,
    /// Página a la que volver tras iniciar sesión.
    waypoint: Waypoint,
}

#[async_trait]
impl Component for LoginForm {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let action = self.waypoint().append_to(cx.route(LOGIN_PATH));
        let strict = SETTINGS.login_strict;

        // En modo estricto se dificulta que el navegador recuerde o autorrellene las
        // credenciales: nombres de campo neutros y campos `Field::strict_text()`/
        // `Field::strict_password()`, que ya incluyen el resto de medidas (ver su documentación).
        let username = if strict {
            form::input::Field::strict_text()
        } else {
            form::input::Field::text().with_autocomplete(Some(form::Autocomplete::username()))
        }
        .with_name(if strict { "ident" } else { "username" })
        .with_label(Lc::t("field-username", &LOCALES_USER))
        .with_autofocus(true)
        .with_required(true);

        let password = if strict {
            form::input::Field::strict_password()
        } else {
            form::input::Field::password()
                .with_autocomplete(Some(form::Autocomplete::current_password()))
        }
        .with_name(if strict { "token" } else { "password" })
        .with_label(Lc::t("field-password", &LOCALES_USER))
        .with_required(true);

        let mut form = Form::new()
            .with_id("user-login-form")
            .with_action(action)
            .with_method(form::Method::Post)
            .with_child(error_banner(self.error().cloned()))
            .with_child(username)
            .with_child(password)
            .with_child(
                form::Checkbox::check()
                    .with_name("remember")
                    .with_label(Lc::t("field-remember-me", &LOCALES_USER)),
            )
            .with_child(Button::submit(Lc::t("btn-login", &LOCALES_USER)))
            .with_child(links(SETTINGS.allow_registration));

        if strict {
            form = form
                .with_prop(PropsOp::set("autocomplete", "off"))
                .with_prop(PropsOp::set("novalidate", "novalidate"));
        }

        let form = form.render(cx).await;
        Ok(html! {
            div.user-login-page {
                div.user-login-card {
                    (form)
                }
            }
        })
    }
}

fn links(allow_registration: bool) -> Html {
    Html::with(move |cx| {
        html! {
            @if allow_registration {
                p.user-register-link {
                    a href=(cx.route(REGISTER_PATH)) {
                        (Lc::t("link-register", &LOCALES_USER).using(cx))
                    }
                }
            }
            p.user-reset-link {
                a href=(cx.route(PASSWORD_RESET_PATH)) {
                    (Lc::t("link-forgot-password", &LOCALES_USER).using(cx))
                }
            }
        }
    })
}

#[builder_impl]
impl LoginForm {
    pub fn with_error(mut self, error: impl Into<Option<Lc>>) -> Self {
        self.error = error.into();
        self
    }

    pub fn with_waypoint(mut self, waypoint: impl Into<Waypoint>) -> Self {
        self.waypoint = waypoint.into();
        self
    }
}
