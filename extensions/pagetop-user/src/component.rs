//! Componentes de UI de `pagetop-user`.

pub(crate) mod admin;

mod account_menu;
mod login_form;
mod password_confirm;
mod password_reset_confirm_form;
mod password_reset_form;
mod register_form;

pub use account_menu::{AccountMenu, account_menu};
pub use login_form::LoginForm;
pub(crate) use password_confirm::PasswordConfirm;
pub use password_reset_confirm_form::PasswordResetConfirmForm;
pub use password_reset_form::PasswordResetForm;
pub use register_form::RegisterForm;

use pagetop::prelude::*;

// Banner de error de formulario; se renderiza vacío si `error` es `None`. Compartido por los
// formularios de autenticación y por los de administración.
pub(crate) fn error_banner(error: Option<Lc>) -> Html {
    Html::with(move |cx| match &error {
        Some(e) => html! { div class="user-form-error" role="alert" { (e.clone().using(cx)) } },
        None => html! {},
    })
}
