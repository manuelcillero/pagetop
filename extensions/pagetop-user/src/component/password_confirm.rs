//! Par de campos "contraseña" + "confirmar contraseña", reutilizado en los formularios de
//! registro, alta y restablecimiento de contraseña.

use pagetop::prelude::*;

use crate::LOCALES_USER;

/// Campos de contraseña y confirmación, compuestos a partir de
/// [`form::input::Field::password()`] del core. No valida en ningún momento que ambos valores
/// coincidan -- eso ocurre en el servidor, tras el envío del formulario -- sólo renderiza los dos
/// campos, uno junto al otro, sin ningún contenedor propio.
#[derive(AutoDefault, Clone, Debug, Getters)]
pub(crate) struct PasswordConfirm {
    /// Devuelve la etiqueta del campo de contraseña.
    password_label: Lc,
    /// Devuelve la etiqueta del campo de confirmación.
    confirm_label: Lc,
}

#[async_trait]
impl Component for PasswordConfirm {
    // Las etiquetas por defecto cubren los dos casos más habituales (alta de cuenta, alta de
    // usuario desde administración); `with_password_label()` cubre el caso distinto
    // (restablecimiento de contraseña por un administrador).
    fn new() -> Self {
        Self {
            password_label: Lc::t("field-password", &LOCALES_USER),
            confirm_label: Lc::t("field-confirm-password", &LOCALES_USER),
        }
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let mut password = form::input::Field::password()
            .with_name("password")
            .with_label(self.password_label().clone())
            .with_autocomplete(Some(form::Autocomplete::new_password()))
            .with_required(true);
        let mut confirm = form::input::Field::password()
            .with_name("confirm_password")
            .with_label(self.confirm_label().clone())
            .with_autocomplete(Some(form::Autocomplete::new_password()))
            .with_required(true);

        Ok(html! {
            (password.render(cx).await)
            (confirm.render(cx).await)
        })
    }
}

#[builder_impl]
impl PasswordConfirm {
    // **< PasswordConfirm BUILDER >********************************************************************

    /// Establece la etiqueta del campo de contraseña (por defecto, "field-password").
    pub(crate) fn with_password_label(mut self, label: Lc) -> Self {
        self.password_label = label;
        self
    }

    /// Establece la etiqueta del campo de confirmación (por defecto, "field-confirm-password").
    pub(crate) fn with_confirm_label(mut self, label: Lc) -> Self {
        self.confirm_label = label;
        self
    }
}
