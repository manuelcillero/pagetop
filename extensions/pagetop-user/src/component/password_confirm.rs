//! Par de campos "contraseña" + "confirmar contraseña", reutilizado en los formularios de
//! registro, alta, restablecimiento y cambio de contraseña.

use pagetop::prelude::*;

use crate::LOCALES_USER;
use crate::config::SETTINGS;

/// Campos de contraseña y confirmación, compuestos a partir de
/// [`form::input::Field::password()`] del core. No valida en ningún momento que ambos valores
/// coincidan (eso ocurre en el servidor, tras el envío del formulario), sólo renderiza los dos
/// campos, uno junto al otro, sin ningún contenedor propio. El de la contraseña indica la longitud
/// mínima configurada (`user.password.min_length`), como ayuda y con `minlength`.
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
    // usuario desde administración); `with_password_label()` cubre los casos distintos
    // (restablecimientos y cambio de la contraseña propia).
    fn new() -> Self {
        Self {
            password_label: Lc::t("field-password", &LOCALES_USER),
            confirm_label: Lc::t("field-confirm-password", &LOCALES_USER),
        }
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        // `minlength` cuenta unidades UTF-16 y el servidor caracteres: con caracteres fuera del
        // plano básico (emojis) el navegador puede aceptar una contraseña que el servidor rechaza,
        // pero nunca al revés.
        let min_length = SETTINGS.password.min_length;
        let mut password = form::input::Field::password()
            .with_name("password")
            .with_label(self.password_label().clone())
            .with_help_text(
                Lc::t("help-password-min-length", &LOCALES_USER)
                    .with_number("n", u32::try_from(min_length).unwrap_or(u32::MAX)),
            )
            .with_autocomplete(Some(form::Autocomplete::new_password()))
            .with_minlength(u16::try_from(min_length).unwrap_or(u16::MAX))
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
    // **< PasswordConfirm BUILDER >****************************************************************

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
