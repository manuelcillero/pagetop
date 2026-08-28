//! Componente de formulario de solicitud de restablecimiento de contraseña.

use pagetop::prelude::*;

use crate::component::error_banner;
use crate::{LOCALES_USER, LOGIN_PATH, PASSWORD_RESET_PATH};

#[derive(AutoDefault, Clone, Debug)]
pub struct PasswordResetForm {
    error: Option<Lc>,
}

#[async_trait]
impl Component for PasswordResetForm {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let mut form = Form::new()
            .with_id("user-password-reset-form")
            .with_action(PASSWORD_RESET_PATH)
            .with_method(form::Method::Post)
            .with_child(error_banner(self.error.clone()))
            .with_child(
                form::input::Field::email()
                    .with_name("email")
                    .with_label(Lc::t("field-email", &LOCALES_USER))
                    .with_autocomplete(Some(form::Autocomplete::email()))
                    .with_autofocus(true)
                    .with_required(true),
            )
            .with_child(Button::submit(Lc::t("btn-send-reset-link", &LOCALES_USER)))
            .with_child(back_to_login());

        Ok(form.render(cx).await)
    }
}

fn back_to_login() -> Html {
    Html::with(|cx| {
        html! {
            p {
                a href=(cx.route(LOGIN_PATH)) {
                    (Lc::t("link-back-to-login", &LOCALES_USER).using(cx))
                }
            }
        }
    })
}

impl PasswordResetForm {
    #[builder_fn]
    pub fn with_error(mut self, error: impl Into<Option<Lc>>) -> Self {
        self.error = error.into();
        self
    }
}
