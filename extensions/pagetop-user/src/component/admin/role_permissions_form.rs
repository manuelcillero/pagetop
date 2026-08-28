//! Formulario de asignación de permisos a un rol, agrupados por categoría. Reemplaza siempre el
//! conjunto completo. Permitido incluso en roles bloqueados (`locked`): los roles de sistema
//! también necesitan permisos gestionables.

use pagetop::prelude::*;

use crate::ADMIN_ROLES_PATH;
use crate::LOCALES_USER;

use crate::component::admin::PermissionGroups;
use crate::component::error_banner;

#[derive(AutoDefault, Clone, Debug, Getters)]
pub(crate) struct RolePermissionsForm {
    role_id: i32,
    error: Option<Lc>,
    groups: PermissionGroups,
    /// Listado de origen al que volver tras guardar (orden).
    waypoint: Waypoint,
}

#[async_trait]
impl Component for RolePermissionsForm {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let action = format!("{ADMIN_ROLES_PATH}/{}/permissions", self.role_id());
        let action = self.waypoint().append_to(cx.route(action));

        let mut form = Form::new()
            .with_id("role-permissions-form")
            .with_action(action)
            .with_method(form::Method::Post)
            .with_child(error_banner(self.error().cloned()));

        for (idx, (group_label, perms)) in self.groups().iter().enumerate() {
            let mut field = form::check::Field::new()
                .with_id(format!("permission-group-{idx}"))
                .with_name("permission_keys");
            for (key, label, checked) in perms {
                let text = label.lookup(cx).unwrap_or_default();
                field = field.with_item(
                    form::check::Item::new(key, Lc::n(format!("{text} ({key})")))
                        .with_checked(*checked),
                );
            }
            let fieldset = form::Fieldset::new()
                .with_legend(group_label.clone())
                .with_child(field);
            form = form.with_child(fieldset);
        }

        form = form.with_child(
            Button::submit(Lc::t("btn-save", &LOCALES_USER))
                .with_style(button::Style::Solid(Intent::Primary)),
        );

        Ok(form.render(cx).await)
    }
}

impl RolePermissionsForm {
    // **< RolePermissionsForm BUILDER >************************************************************

    #[builder_fn]
    pub(crate) fn with_role_id(mut self, role_id: i32) -> Self {
        self.role_id = role_id;
        self
    }

    #[builder_fn]
    pub(crate) fn with_error(mut self, error: impl Into<Option<Lc>>) -> Self {
        self.error = error.into();
        self
    }

    #[builder_fn]
    pub(crate) fn with_groups(mut self, groups: PermissionGroups) -> Self {
        self.groups = groups;
        self
    }

    #[builder_fn]
    pub(crate) fn with_waypoint(mut self, waypoint: impl Into<Waypoint>) -> Self {
        self.waypoint = waypoint.into();
        self
    }
}
