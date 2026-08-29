//! Formulario de alta/edición de usuario.

use pagetop::prelude::*;

use crate::ADMIN_USERS_PATH;
use crate::LOCALES_USER;

use crate::component::{PasswordConfirm, error_banner};

use super::{USER_ADMIN_FORM_ID, roles_fieldset};

#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub(crate) enum UserFormMode {
    #[default]
    New,
    Edit,
}

#[derive(AutoDefault, Clone, Debug, Getters)]
pub(crate) struct UserForm {
    mode: UserFormMode,
    error: Option<Lc>,
    user_id: Option<i32>,
    /// Listado de origen al que volver tras guardar (orden, búsqueda, página).
    waypoint: Waypoint,
    username: String,
    email: String,
    display_name: String,
    language: String,
    timezone: String,
    /// Roles asignables (excluye "anonymous" y "authenticated"); sólo se renderiza en modo `New`.
    roles: Vec<(i32, String, bool)>,
    /// Si se ofrece la casilla "administrador"; sólo cuando quien da de alta ya es administrador.
    allow_admin_field: bool,
    is_admin: bool,
}

#[async_trait]
impl Component for UserForm {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let action = match self.mode() {
            UserFormMode::New => format!("{ADMIN_USERS_PATH}/new"),
            UserFormMode::Edit => {
                format!(
                    "{ADMIN_USERS_PATH}/{}/edit",
                    self.user_id().copied().unwrap_or_default()
                )
            }
        };
        let action = self.waypoint().append_to(cx.route(action));

        let mut form = Form::new()
            .with_id(USER_ADMIN_FORM_ID)
            .with_action(action)
            .with_method(form::Method::Post)
            .with_child(error_banner(self.error().cloned()))
            .with_child(
                form::input::Field::text()
                    .with_name("username")
                    .with_value(self.username())
                    .with_label(Lc::t("field-username-admin", &LOCALES_USER))
                    .with_required(true)
                    .with_maxlength(Some(64)),
            )
            .with_child(
                form::input::Field::email()
                    .with_name("email")
                    .with_value(self.email())
                    .with_label(Lc::t("field-email", &LOCALES_USER))
                    .with_required(true),
            )
            .with_child(
                form::input::Field::text()
                    .with_name("display_name")
                    .with_value(self.display_name())
                    .with_label(Lc::t("field-display-name", &LOCALES_USER)),
            )
            .with_child(
                form::input::Field::text()
                    .with_name("language")
                    .with_value(self.language())
                    .with_label(Lc::t("field-language", &LOCALES_USER)),
            )
            .with_child(
                form::input::Field::text()
                    .with_name("timezone")
                    .with_value(self.timezone())
                    .with_label(Lc::t("field-timezone", &LOCALES_USER)),
            );

        if *self.mode() == UserFormMode::New {
            form = form
                .with_child(PasswordConfirm::new())
                .with_child(roles_fieldset(self.roles()));

            if *self.allow_admin_field() {
                form = form.with_child(
                    form::Checkbox::check()
                        .with_name("is_admin")
                        .with_label(Lc::t("field-is-admin", &LOCALES_USER))
                        .with_checked(*self.is_admin()),
                );
            }
        }

        // En modo `Edit`, "Guardar" se renderiza fuera del formulario, junto al resto de acciones
        // de la pantalla (ver `USER_ADMIN_FORM_ID`); en modo `New` no hay ninguna botonera con la
        // que agruparlo, así que se queda aquí, dentro del propio `<form>`.
        if *self.mode() == UserFormMode::New {
            form = form.with_child(
                Button::submit(Lc::t("btn-save", &LOCALES_USER))
                    .with_style(button::Style::Solid(Intent::Primary)),
            );
        }

        Ok(form.render(cx).await)
    }
}

#[builder_impl]
impl UserForm {
    // **< UserForm BUILDER >***********************************************************************

    pub(crate) fn with_mode(mut self, mode: UserFormMode) -> Self {
        self.mode = mode;
        self
    }

    pub(crate) fn with_error(mut self, error: impl Into<Option<Lc>>) -> Self {
        self.error = error.into();
        self
    }

    pub(crate) fn with_user_id(mut self, user_id: impl Into<Option<i32>>) -> Self {
        self.user_id = user_id.into();
        self
    }

    pub(crate) fn with_waypoint(mut self, waypoint: impl Into<Waypoint>) -> Self {
        self.waypoint = waypoint.into();
        self
    }

    pub(crate) fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = username.into();
        self
    }

    pub(crate) fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }

    pub(crate) fn with_display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = display_name.into();
        self
    }

    pub(crate) fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = language.into();
        self
    }

    pub(crate) fn with_timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = timezone.into();
        self
    }

    pub(crate) fn with_roles(mut self, roles: Vec<(i32, String, bool)>) -> Self {
        self.roles = roles;
        self
    }

    pub(crate) fn with_allow_admin_field(mut self, allow_admin_field: bool) -> Self {
        self.allow_admin_field = allow_admin_field;
        self
    }

    pub(crate) fn with_is_admin(mut self, is_admin: bool) -> Self {
        self.is_admin = is_admin;
        self
    }
}
