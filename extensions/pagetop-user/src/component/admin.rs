//! Componentes de administración: listados y mantenimiento de usuarios, roles y permisos.
//!
//! Todos son `pub(crate)`: son UI interna de esta extensión, no forman parte del *prelude* público.

mod admin_password_form;
mod role_form;
mod role_permissions_form;
mod role_table;
mod user_form;
mod user_table;

pub(crate) use admin_password_form::AdminPasswordForm;
pub(crate) use role_form::{RoleForm, RoleFormMode};
pub(crate) use role_permissions_form::RolePermissionsForm;
pub(crate) use role_table::{RoleTable, delete_dialog};
pub(crate) use user_form::{UserForm, UserFormMode};
pub(crate) use user_table::{UserTable, status_key};

use pagetop::prelude::*;

use crate::LOCALES_USER;

// **< constantes compartidas >*********************************************************************

/// Identificador del `<form>` de [`UserForm`] en modo [`UserFormMode::Edit`]. En ese modo, el botón
/// "Guardar" no se renderiza dentro del formulario (ver `UserForm::prepare()`). Lo añade la
/// pantalla de edición (`handlers::admin::users::edit_actions()`) junto al resto de acciones,
/// referenciando este `id` mediante el atributo `form` para seguir enviando el formulario aunque
/// esté fuera de él.
pub(crate) const USER_ADMIN_FORM_ID: &str = "user-admin-form";

// **< tipos compartidos >**************************************************************************

/// Un permiso dentro de un grupo del catálogo: `(clave, etiqueta, concedido)`.
pub(crate) type PermissionItem = (CowStr, Lc, bool);

/// Catálogo de permisos agrupado: `(título del grupo, permisos del grupo)`.
pub(crate) type PermissionGroups = Vec<(Lc, Vec<PermissionItem>)>;

// **< HELPERS >************************************************************************************

// `Fieldset` con las casillas para asignar roles (usado en el alta y en la edición de usuario). El
// rol "authenticated" no se lista como casilla ni se envía: todo usuario autenticado lo tiene
// concedido por definición (ver `session::load_user_from_session`), sin necesidad de una fila en
// `user_role`.
//
// Los roles de `managed` los asigna el proveedor `provider` que gestiona la cuenta: se muestran
// deshabilitados, con una nota, y quien recibe el formulario los conserva como estén.
pub(crate) fn roles_fieldset(
    roles: &[(i32, String, bool)],
    managed: &[i32],
    provider: &str,
) -> form::Fieldset {
    let mut field = form::check::Field::new().with_name("role_ids");
    for (role_id, label, checked) in roles {
        let is_managed = managed.contains(role_id);
        let label = if is_managed {
            Lc::t("value-role-managed", &LOCALES_USER)
                .with_arg("role", label.clone())
                .with_arg("provider", provider.to_owned())
        } else {
            Lc::n(label.clone())
        };
        field = field.with_item(
            form::check::Item::new(role_id.to_string(), label)
                .with_checked(*checked)
                .with_disabled(is_managed),
        );
    }
    form::Fieldset::new()
        .with_legend(Lc::t("field-roles", &LOCALES_USER))
        .with_child(field)
}
