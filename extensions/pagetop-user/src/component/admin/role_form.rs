//! Formulario de alta/edición de rol.

use pagetop::prelude::*;

use crate::ADMIN_ROLES_PATH;
use crate::LOCALES_USER;

use crate::component::error_banner;

#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub(crate) enum RoleFormMode {
    #[default]
    New,
    Edit,
}

#[derive(AutoDefault, Clone, Debug, Getters)]
pub(crate) struct RoleForm {
    mode: RoleFormMode,
    error: Option<Lc>,
    role_id: Option<i32>,
    /// Listado de origen al que volver tras guardar (orden).
    waypoint: Waypoint,
    machine_name: String,
    label: String,
    description: String,
    weight: i32,
}

#[async_trait]
impl Component for RoleForm {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let action = match self.mode() {
            RoleFormMode::New => format!("{ADMIN_ROLES_PATH}/new"),
            RoleFormMode::Edit => {
                format!(
                    "{ADMIN_ROLES_PATH}/{}/edit",
                    self.role_id().copied().unwrap_or_default()
                )
            }
        };
        let action = self.waypoint().append_to(cx.route(action));

        let machine_name_field = match self.mode() {
            RoleFormMode::New => form::input::Field::text()
                .with_name("machine_name")
                .with_value(self.machine_name())
                .with_label(Lc::t("field-machine-name", &LOCALES_USER))
                .with_help_text(Lc::t("help-machine-name-immutable", &LOCALES_USER))
                .with_required(true)
                .with_maxlength(Some(64)),
            RoleFormMode::Edit => form::input::Field::text()
                .with_name("machine_name")
                .with_value(self.machine_name())
                .with_label(Lc::t("field-machine-name", &LOCALES_USER))
                .with_plaintext(true),
        };

        let mut form = Form::new()
            .with_id("role-admin-form")
            .with_action(action)
            .with_method(form::Method::Post)
            .with_child(error_banner(self.error().cloned()))
            .with_child(machine_name_field)
            .with_child(
                form::input::Field::text()
                    .with_name("label")
                    .with_value(self.label())
                    .with_label(Lc::t("field-label", &LOCALES_USER))
                    .with_required(true)
                    .with_maxlength(Some(128)),
            )
            .with_child(
                form::Textarea::new()
                    .with_name("description")
                    .with_value(self.description())
                    .with_label(Lc::t("field-description", &LOCALES_USER))
                    .with_rows(Some(3)),
            )
            .with_child(
                form::input::Field::text()
                    .with_name("weight")
                    .with_value(self.weight().to_string())
                    .with_label(Lc::t("field-weight", &LOCALES_USER))
                    .with_inputmode(Some(form::input::Mode::Numeric)),
            );

        form = form.with_child(
            Button::submit(Lc::t("btn-save", &LOCALES_USER))
                .with_style(button::Style::Solid(Intent::Primary)),
        );

        Ok(form.render(cx).await)
    }
}

#[builder_impl]
impl RoleForm {
    // **< RoleForm BUILDER >***********************************************************************

    pub(crate) fn with_mode(mut self, mode: RoleFormMode) -> Self {
        self.mode = mode;
        self
    }

    pub(crate) fn with_error(mut self, error: impl Into<Option<Lc>>) -> Self {
        self.error = error.into();
        self
    }

    pub(crate) fn with_role_id(mut self, role_id: impl Into<Option<i32>>) -> Self {
        self.role_id = role_id.into();
        self
    }

    pub(crate) fn with_waypoint(mut self, waypoint: impl Into<Waypoint>) -> Self {
        self.waypoint = waypoint.into();
        self
    }

    pub(crate) fn with_machine_name(mut self, machine_name: impl Into<String>) -> Self {
        self.machine_name = machine_name.into();
        self
    }

    pub(crate) fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub(crate) fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub(crate) fn with_weight(mut self, weight: i32) -> Self {
        self.weight = weight;
        self
    }
}
