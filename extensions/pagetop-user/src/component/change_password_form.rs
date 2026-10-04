//! Formulario para que el usuario autenticado cambie su propia contraseña.

use pagetop::prelude::*;

use crate::{LOCALES_USER, PROFILE_PASSWORD_PATH};

use crate::component::{PasswordConfirm, error_banner};

/// Pide la contraseña actual además de la nueva y su confirmación, a diferencia del
/// restablecimiento por un administrador.
#[derive(AutoDefault, Clone, Debug, Getters)]
pub(crate) struct ChangePasswordForm {
    error: Option<Lc>,
}

#[async_trait]
impl Component for ChangePasswordForm {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let mut form = Form::new()
            .with_id("user-change-password-form")
            .with_action(cx.route(PROFILE_PASSWORD_PATH))
            .with_method(form::Method::Post)
            .with_child(error_banner(self.error().cloned()))
            .with_child(
                form::input::Field::password()
                    .with_name("current_password")
                    .with_label(Lc::t("field-current-password", &LOCALES_USER))
                    .with_autocomplete(Some(form::Autocomplete::current_password()))
                    .with_required(true),
            )
            .with_child(
                PasswordConfirm::new()
                    .with_password_label(Lc::t("field-new-password", &LOCALES_USER)),
            )
            .with_child(
                Button::submit(Lc::t("btn-set-password", &LOCALES_USER))
                    .with_style(button::Style::Solid(Intent::Primary)),
            );

        Ok(form.render(cx).await)
    }
}

#[builder_impl]
impl ChangePasswordForm {
    // **< ChangePasswordForm BUILDER >*************************************************************

    pub(crate) fn with_error(mut self, error: impl Into<Option<Lc>>) -> Self {
        self.error = error.into();
        self
    }
}
