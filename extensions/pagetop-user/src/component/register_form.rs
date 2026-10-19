//! Componente de formulario de registro.

use pagetop::prelude::*;

use crate::LOCALES_USER;
use crate::REGISTER_PATH;
use crate::component::{PasswordConfirm, error_banner};
use crate::config::SETTINGS;
use crate::identity::{EMAIL_MAX_CHARS, USERNAME_MIN_CHARS, username_help, username_max_chars};

#[derive(AutoDefault, Clone, Debug)]
pub struct RegisterForm {
    error: Option<Lc>,
    username: String,
    email: String,
}

#[async_trait]
impl Component for RegisterForm {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let mut form = Form::new()
            .with_id("user-register-form")
            .with_action(REGISTER_PATH)
            .with_method(form::Method::Post)
            .with_child(error_banner(self.error.clone()))
            .with_child(
                form::input::Field::text()
                    .with_name("username")
                    .with_value(&self.username)
                    .with_label(Lc::t("field-username-admin", &LOCALES_USER))
                    .with_help_text(username_help(SETTINGS.identity_policy))
                    .with_minlength(USERNAME_MIN_CHARS)
                    .with_maxlength(username_max_chars(SETTINGS.identity_policy))
                    .with_autocomplete(Some(form::Autocomplete::username()))
                    .with_autofocus(true)
                    .with_required(true),
            )
            .with_child(
                form::input::Field::email()
                    .with_name("email")
                    .with_value(&self.email)
                    .with_label(Lc::t("field-email", &LOCALES_USER))
                    .with_maxlength(EMAIL_MAX_CHARS)
                    .with_autocomplete(Some(form::Autocomplete::email()))
                    .with_required(true),
            )
            .with_child(PasswordConfirm::new())
            .with_child(Button::submit(Lc::t("btn-register", &LOCALES_USER)));

        Ok(form.render(cx).await)
    }
}

#[builder_impl]
impl RegisterForm {
    pub fn with_error(mut self, error: impl Into<Option<Lc>>) -> Self {
        self.error = error.into();
        self
    }

    /// Rellena el nombre de usuario, p. ej. con el enviado si el registro falla.
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = username.into();
        self
    }

    /// Rellena el email, p. ej. con el enviado si el registro falla.
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }
}
