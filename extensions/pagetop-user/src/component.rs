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
use crate::account::UserStatus;
use crate::component::admin::status_key;
use crate::config::{user_language_applies, user_timezone_applies};
use crate::entity::user;

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

// Bloque de sólo lectura con los datos de perfil de un usuario, para «Mi cuenta» y para la ficha de
// administración.
pub(crate) async fn user_details(user: &user::Model, cx: &mut Context) -> Block {
    let status = UserStatus::from_i16(user.status);
    let mut table = Table::new()
        .with_prop(PropsOp::add_classes("user-admin-table"))
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-username-admin", &LOCALES_USER))
                .with_cell(user.username.as_str()),
        )
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-email", &LOCALES_USER))
                .with_cell(user.email.as_str()),
        )
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-display-name", &LOCALES_USER))
                .with_cell(user.display_name.as_deref().unwrap_or("-")),
        )
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-about", &LOCALES_USER))
                .with_cell(multiline_text(
                    user.about.clone().unwrap_or_else(|| "-".into()),
                )),
        );
    // Sólo se muestran si se aplican, igual que en el formulario de edición.
    if user_language_applies() {
        table = table.with_row(
            table::Row::new()
                .with_cell(Lc::t("field-language", &LOCALES_USER))
                .with_cell(language_name(user.language.as_deref())),
        );
    }
    if user_timezone_applies() {
        table = table.with_row(
            table::Row::new()
                .with_cell(Lc::t("field-timezone", &LOCALES_USER))
                .with_cell(user.timezone.as_deref().unwrap_or("-")),
        );
    }
    table = table
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-theme", &LOCALES_USER))
                .with_cell(theme_name(user.theme.as_deref())),
        )
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("col-status", &LOCALES_USER))
                .with_cell(Lc::t(status_key(status), &LOCALES_USER)),
        )
        .with_row(
            table::Row::new()
                .with_cell(Lc::t("field-member-since", &LOCALES_USER))
                .with_cell(cx.format_date(
                    user.created_at.with_timezone(&cx.timezone()).date_naive(),
                    DateFormat::Long,
                )),
        );

    if user.is_admin {
        let badge = Badge::severe(Lc::t("badge-admin", &LOCALES_USER))
            .render(cx)
            .await;
        table = table.with_row(
            table::Row::new()
                .with_cell("")
                .with_cell(Html::with(move |_| badge.clone())),
        );
    }

    Block::new()
        .with_title(Lc::t("title-user-details", &LOCALES_USER))
        .with_child(table)
}
