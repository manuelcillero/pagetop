//! Componentes de UI de `pagetop-user`.

pub(crate) mod admin;

mod account_menu;
mod change_password_form;
mod login_form;
mod password_confirm;
mod password_reset_confirm_form;
mod password_reset_form;
mod register_form;
mod user_name;

pub use account_menu::{AccountMenu, account_menu};
pub(crate) use change_password_form::ChangePasswordForm;
pub use login_form::LoginForm;
pub(crate) use password_confirm::PasswordConfirm;
pub use password_reset_confirm_form::PasswordResetConfirmForm;
pub use password_reset_form::PasswordResetForm;
pub use register_form::RegisterForm;
pub use user_name::UserName;

use pagetop::prelude::*;

use crate::LOCALES_USER;

// Texto libre de varias líneas como contenido seguro: cada salto de línea se convierte en `<br>`,
// sin depender del CSS del tema. Se usa para mostrar el "Sobre mí" del usuario.
pub(crate) fn multiline_text(text: String) -> Html {
    Html::with(move |_| {
        html! {
            @for (i, line) in text.lines().enumerate() {
                @if i > 0 { br; }
                (line)
            }
        }
    })
}

// Nombre traducido del idioma guardado en el perfil de un usuario, o "-" si no tiene ninguno. Un
// identificador que ya no esté entre los soportados se muestra tal cual.
pub(crate) fn language_name(language: Option<&str>) -> Lc {
    let Some(code) = language else {
        return Lc::n("-");
    };
    Locale::supported_languages()
        .into_iter()
        .find(|(langid, _)| langid.to_string() == code)
        .map_or_else(|| Lc::n(code.to_owned()), |(_, name)| name)
}

// Nombre traducido del tema guardado en el perfil de un usuario o, si no tiene ninguno, el del tema
// predeterminado del sitio, que es el que se le aplica. Un tema que ya no esté habilitado se
// muestra con el nombre guardado.
pub(crate) fn theme_name(theme: Option<&str>) -> table::Cell {
    let Some(name) = theme else {
        // Los argumentos de `Lc` son texto fijo: el nombre del tema se traduce al renderizar.
        return Html::with(|cx| {
            let theme = default_theme();
            let name = theme
                .name()
                .lookup(cx)
                .unwrap_or_else(|| theme.short_name().to_owned());
            Lc::t("value-theme-site-default", &LOCALES_USER)
                .with_arg("theme", name)
                .using(cx)
        })
        .into();
    };
    theme_by_short_name(name)
        .map_or_else(|| Lc::n(name.to_owned()), |theme| theme.name())
        .into()
}

// Banner de error de formulario; se renderiza vacío si `error` es `None`. Compartido por los
// formularios de autenticación y por los de administración.
pub(crate) fn error_banner(error: Option<Lc>) -> Html {
    Html::with(move |cx| match &error {
        Some(e) => html! { div class="user-form-error" role="alert" { (e.clone().using(cx)) } },
        None => html! {},
    })
}
