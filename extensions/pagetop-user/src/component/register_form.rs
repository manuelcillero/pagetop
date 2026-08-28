//! Componente de formulario de registro.

use pagetop::prelude::*;

use crate::LOCALES_USER;
use crate::REGISTER_PATH;
use crate::component::{PasswordConfirm, error_banner};

#[derive(AutoDefault, Clone, Debug)]
pub struct RegisterForm {
    error: Option<Lc>,
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
                    .with_label(Lc::t("field-username", &LOCALES_USER))
                    .with_autocomplete(Some(form::Autocomplete::username()))
                    .with_autofocus(true)
                    .with_required(true),
            )
            .with_child(
                form::input::Field::email()
                    .with_name("email")
                    .with_label(Lc::t("field-email", &LOCALES_USER))
                    .with_autocomplete(Some(form::Autocomplete::email()))
                    .with_required(true),
            )
            .with_child(PasswordConfirm::new())
            .with_child(Button::submit(Lc::t("btn-register", &LOCALES_USER)));

        Ok(form.render(cx).await)
    }
}

impl RegisterForm {
    #[builder_fn]
    pub fn with_error(mut self, error: impl Into<Option<Lc>>) -> Self {
        self.error = error.into();
        self
    }
}
