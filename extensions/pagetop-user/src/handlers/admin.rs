//! Handlers HTTP para el mantenimiento de usuarios, roles y permisos.
//!
//! Accesibles sólo por URL directa (sin entradas de menú). Cada handler comprueba el permiso
//! correspondiente con [`require_permission()`](pagetop::auth::require_permission) antes de
//! construir la página.

pub(crate) mod permissions;
pub(crate) mod roles;
pub(crate) mod users;

use pagetop::prelude::*;
use pagetop_admin::component::AdminFrame;

use crate::LOCALES_USER;
use crate::error::AuthError;

// Envuelve el contenido de una página de administración en el frame de `pagetop-admin`:
// breadcrumb, tareas y acciones locales registradas para la ruta actual en su `AdminRegistry`.
pub(crate) fn frame(title: Lc) -> AdminFrame {
    AdminFrame::new().with_title(title)
}

// Traduce un `AuthError` a su clave Lc, para mostrarlo en el formulario que falló. Cubre tanto
// los errores de autenticación existentes como los nuevos de administración.
pub(crate) fn map_auth_error(err: &AuthError) -> Lc {
    match err {
        AuthError::PasswordTooShort(n) => {
            Lc::t("error-password-too-short", &LOCALES_USER).with_arg("n", n.to_string())
        }
        AuthError::PasswordMismatch => Lc::t("error-password-mismatch", &LOCALES_USER),
        AuthError::UsernameTaken => Lc::t("error-username-taken", &LOCALES_USER),
        AuthError::EmailTaken => Lc::t("error-email-taken", &LOCALES_USER),
        AuthError::UserNotFound => Lc::t("error-user-not-found", &LOCALES_USER),
        AuthError::RoleNotFound => Lc::t("error-role-not-found", &LOCALES_USER),
        AuthError::RoleMachineNameTaken => Lc::t("error-role-machine-name-taken", &LOCALES_USER),
        AuthError::InvalidMachineName => Lc::t("error-invalid-machine-name", &LOCALES_USER),
        AuthError::RoleLocked => Lc::t("error-role-locked", &LOCALES_USER),
        AuthError::RoleInUse => Lc::t("error-role-in-use", &LOCALES_USER),
        AuthError::LastAdministrator => Lc::t("error-last-administrator", &LOCALES_USER),
        AuthError::CannotBlockSelf => Lc::t("error-cannot-block-self", &LOCALES_USER),
        AuthError::CannotModifyOwnAdminFlag => {
            Lc::t("error-cannot-modify-own-admin-flag", &LOCALES_USER)
        }
        AuthError::UnknownPermission(_) => Lc::t("error-unknown-permission", &LOCALES_USER),
        _ => Lc::t("error-internal", &LOCALES_USER),
    }
}

// Enlace de vuelta al listado, usado en las pantallas de alta/edición/asignación.
pub(crate) fn back_link(href: impl Into<RoutePath>) -> Html {
    let href = href.into();
    Html::with(move |cx| {
        html! {
            p.user-admin-back-link {
                a href=(href.clone()) { (Lc::t("link-back-to-list", &LOCALES_USER).using(cx)) }
            }
        }
    })
}
