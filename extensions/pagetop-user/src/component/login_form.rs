//! Componente de formulario de inicio de sesión.

use pagetop::prelude::*;

use crate::component::error_banner;
use crate::config::SETTINGS;
use crate::provider::{self, HelpLink, ResetLink};
use crate::{LOCALES_USER, LOGIN_PATH, PASSWORD_RESET_PATH, REGISTER_PATH};

/// Formulario de inicio de sesión.
///
/// Además de los campos de usuario y contraseña, muestra lo que piden los proveedores de
/// autenticación activos (ver
/// [`AuthProvider::login_hints()`](crate::provider::AuthProvider::login_hints)): su nota bajo el
/// formulario y, si `"local"` no está en `[user.login] sources`, su enlace para recuperar la
/// contraseña en lugar del de `pagetop-user`. La casilla «Recuérdame» sólo aparece si alguna fuente
/// la admite.
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct LoginForm {
    /// Mensaje de error sobre el formulario.
    error: Option<Lc>,
    /// Enlace de ayuda junto al error (p. ej. dónde cambiar una contraseña caducada).
    help: Option<HelpLink>,
    /// Usuario o email con el que se rellena el campo.
    username: String,
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
        let strict = SETTINGS.login.strict;
        let (notes, reset) = provider::login_hints();

        // En modo estricto se dificulta que el navegador recuerde o autorrellene las
        // credenciales: nombres de campo neutros y campos `Field::strict_text()`/
        // `Field::strict_password()`, que ya incluyen el resto de medidas (ver su documentación).
        let username = if strict {
            form::input::Field::strict_text()
        } else {
            form::input::Field::text().with_autocomplete(Some(form::Autocomplete::username()))
        }
        .with_name(if strict { "ident" } else { "username" })
        .with_value(self.username())
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
            .with_child(help_link(self.help().cloned()))
            .with_child(username)
            .with_child(password);
        if provider::any_allows_remember() {
            form = form.with_child(
                form::Checkbox::check()
                    .with_name("remember")
                    .with_label(Lc::t("field-remember-me", &LOCALES_USER))
                    .with_help_text(Lc::t("help-remember-me", &LOCALES_USER)),
            );
        }
        form = form
            .with_child(
                Button::submit(Lc::t("btn-login", &LOCALES_USER))
                    .with_style(button::Style::Solid(Intent::Primary)),
            )
            .with_child(links(SETTINGS.allow_registration, notes, reset));

        if strict {
            form = form
                .with_prop(PropsOp::set("autocomplete", "off"))
                .with_prop(PropsOp::set("novalidate", "novalidate"));
        }

        let form = form.render(cx).await;
        Ok(html! {
            div class="user-login-page" {
                div class="user-login-card" {
                    (form)
                }
            }
        })
    }
}

fn links(allow_registration: bool, notes: Vec<Lc>, reset: ResetLink) -> Html {
    Html::with(move |cx| {
        html! {
            @for note in &notes {
                p class="user-login-note" { (note.using(cx)) }
            }
            @if allow_registration {
                p class="user-register-link" {
                    a href=(cx.route(REGISTER_PATH)) {
                        (Lc::t("link-register", &LOCALES_USER).using(cx))
                    }
                }
            }
            @match &reset {
                ResetLink::Local => {
                    p class="user-reset-link" {
                        a href=(cx.route(PASSWORD_RESET_PATH)) {
                            (Lc::t("link-forgot-password", &LOCALES_USER).using(cx))
                        }
                    }
                }
                ResetLink::External(help) => {
                    p class="user-reset-link" {
                        a href=(help.url()) {
                            @match help.text().lookup(cx) {
                                Some(text) => (text),
                                None => (Lc::t("link-forgot-password", &LOCALES_USER).using(cx)),
                            }
                        }
                    }
                }
                ResetLink::Hidden => {}
            }
        }
    })
}

// Enlace de ayuda que acompaña a un error, si lo hay.
fn help_link(help: Option<HelpLink>) -> Html {
    Html::with(move |cx| match &help {
        Some(help) => html! {
            p class="user-login-help" {
                a href=(help.url()) {
                    @match help.text().lookup(cx) {
                        Some(text) => (text),
                        None => (help.url()),
                    }
                }
            }
        },
        None => html! {},
    })
}

#[builder_impl]
impl LoginForm {
    /// Mensaje de error sobre el formulario, p. ej. el del envío que ha fallado.
    pub fn with_error(mut self, error: impl Into<Option<Lc>>) -> Self {
        self.error = error.into();
        self
    }

    /// Enlace de ayuda junto al error.
    pub fn with_help(mut self, help: impl Into<Option<HelpLink>>) -> Self {
        self.help = help.into();
        self
    }

    /// Rellena el usuario o email, p. ej. con el enviado si el inicio de sesión falla.
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = username.into();
        self
    }

    pub fn with_waypoint(mut self, waypoint: impl Into<Waypoint>) -> Self {
        self.waypoint = waypoint.into();
        self
    }
}
