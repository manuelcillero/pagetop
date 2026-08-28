//! Componente de formulario para introducir la nueva contraseña.

use pagetop::prelude::*;

use crate::LOCALES_USER;
use crate::component::error_banner;

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
                form::input::Field::password()
                    .with_name("password")
                    .with_label(Lc::t("field-new-password", &LOCALES_USER))
                    .with_autocomplete(Some(form::Autocomplete::new_password()))
                    .with_autofocus(true)
                    .with_required(true),
            )
            .with_child(
                form::input::Field::password()
                    .with_name("confirm_password")
                    .with_label(Lc::t("field-confirm-password", &LOCALES_USER))
                    .with_autocomplete(Some(form::Autocomplete::new_password()))
                    .with_required(true),
            )
            .with_child(Button::submit(Lc::t("btn-set-password", &LOCALES_USER)));

        Ok(form.render(cx).await)
    }
}

impl PasswordResetConfirmForm {
    #[builder_fn]
    pub fn with_error(mut self, error: impl Into<Option<Lc>>) -> Self {
        self.error = error.into();
        self
    }
}
