//! Formulario de asignación de roles a un usuario. Reemplaza siempre el conjunto completo.

use pagetop::prelude::*;

use crate::ADMIN_USERS_PATH;
use crate::LOCALES_USER;

use crate::component::error_banner;

use super::roles_fieldset;

#[derive(AutoDefault, Clone, Debug, Getters)]
pub(crate) struct UserRolesForm {
    user_id: i32,
    error: Option<Lc>,
    roles: Vec<(i32, String, bool)>,
    /// Listado de origen al que volver tras guardar (orden, búsqueda, página).
    waypoint: Waypoint,
}

#[async_trait]
impl Component for UserRolesForm {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let action = format!("{ADMIN_USERS_PATH}/{}/roles", self.user_id());
        let action = self.waypoint().append_to(cx.route(action));

        let mut form = Form::new()
            .with_id("user-roles-form")
            .with_action(action)
            .with_method(form::Method::Post)
            .with_child(error_banner(self.error().cloned()))
            .with_child(roles_fieldset(self.roles()))
            .with_child(
                Button::submit(Lc::t("btn-save", &LOCALES_USER))
                    .with_style(button::Style::Solid(Intent::Primary)),
            );

        Ok(form.render(cx).await)
    }
}

impl UserRolesForm {
    // **< UserRolesForm BUILDER >******************************************************************

    #[builder_fn]
    pub(crate) fn with_user_id(mut self, user_id: i32) -> Self {
        self.user_id = user_id;
        self
    }

    #[builder_fn]
    pub(crate) fn with_error(mut self, error: impl Into<Option<Lc>>) -> Self {
        self.error = error.into();
        self
    }

    #[builder_fn]
    pub(crate) fn with_roles(mut self, roles: Vec<(i32, String, bool)>) -> Self {
        self.roles = roles;
        self
    }

    #[builder_fn]
    pub(crate) fn with_waypoint(mut self, waypoint: impl Into<Waypoint>) -> Self {
        self.waypoint = waypoint.into();
        self
    }
}
