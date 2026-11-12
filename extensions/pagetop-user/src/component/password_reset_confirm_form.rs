//! Componente de formulario para introducir la nueva contraseña.

use pagetop::prelude::*;

use crate::LOCALES_USER;
use crate::component::{PasswordConfirm, error_banner};

/// Formulario para elegir la nueva contraseña desde el enlace de recuperación.
///
/// Pide la contraseña y su confirmación, y se envía a la propia página del enlace, cuya dirección
/// lleva la cuenta y el *token*: sólo funciona dentro de ella.
#[derive(AutoDefault, Clone, Debug)]
pub struct PasswordResetConfirmForm {
    error: Option<Lc>,
}

#[async_trait]
impl Component for PasswordResetConfirmForm {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        // Sin `with_action()`: se envía a la propia URL, que incluye el `{uid}/{token}` de la
        // petición actual.
        let mut form = Form::new()
            .with_id("user-new-password-form")
            .with_method(form::Method::Post)
            .with_child(error_banner(self.error.clone()))
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
impl PasswordResetConfirmForm {
    /// Mensaje de error sobre el formulario, p. ej. el del envío que ha fallado.
    pub fn with_error(mut self, error: impl Into<Option<Lc>>) -> Self {
        self.error = error.into();
        self
    }
}
