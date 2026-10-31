//! Formulario para que el usuario autenticado cambie su propia dirección de email.

use pagetop::prelude::*;

use crate::{EMAIL_PATH, LOCALES_USER};

use crate::component::error_banner;
use crate::identity::EMAIL_MAX_CHARS;

/// Pide la nueva dirección y la contraseña actual. Si ya hay un cambio pendiente de confirmar, lo
/// indica: pedir otro lo sustituye.
#[derive(AutoDefault, Clone, Debug, Getters)]
pub(crate) struct ChangeEmailForm {
    error: Option<Lc>,
    email: String,
    pending_email: Option<String>,
}

#[async_trait]
impl Component for ChangeEmailForm {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let mut form = Form::new()
            .with_id("user-change-email-form")
            .with_action(cx.route(EMAIL_PATH))
            .with_method(form::Method::Post)
            .with_child(error_banner(self.error().cloned()));
        if let Some(pending) = self.pending_email().cloned() {
            form = form.with_child(Html::with(move |cx| {
                html! {
                    p {
                        (Lc::t("msg-email-change-pending", &LOCALES_USER)
                            .with_arg("email", pending.clone())
                            .using(cx))
                    }
                }
            }));
        }
        form = form
            .with_child(
                form::input::Field::email()
                    .with_name("email")
                    .with_value(self.email())
                    .with_label(Lc::t("field-new-email", &LOCALES_USER))
                    .with_autocomplete(Some(form::Autocomplete::email()))
                    .with_required(true)
                    .with_maxlength(EMAIL_MAX_CHARS),
            )
            .with_child(
                form::input::Field::password()
                    .with_name("current_password")
                    .with_label(Lc::t("field-current-password", &LOCALES_USER))
                    .with_autocomplete(Some(form::Autocomplete::current_password()))
                    .with_required(true),
            )
            .with_child(
                Button::submit(Lc::t("btn-change-email", &LOCALES_USER))
                    .with_style(button::Style::Solid(Intent::Primary)),
            );

        Ok(form.render(cx).await)
    }
}

#[builder_impl]
impl ChangeEmailForm {
    // **< ChangeEmailForm BUILDER >****************************************************************

    pub(crate) fn with_error(mut self, error: impl Into<Option<Lc>>) -> Self {
        self.error = error.into();
        self
    }

    pub(crate) fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }

    pub(crate) fn with_pending_email(mut self, pending_email: impl Into<Option<String>>) -> Self {
        self.pending_email = pending_email.into();
        self
    }
}
