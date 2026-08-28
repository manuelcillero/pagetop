//! Componentes de administración: listados y mantenimiento de usuarios, roles y permisos.
//!
//! Todos son `pub(crate)`: son UI interna de esta extensión, no forman parte del *prelude* público.

mod admin_password_form;
mod role_form;
mod role_permissions_form;
mod role_table;
mod user_form;
mod user_roles_form;
mod user_table;

pub(crate) use admin_password_form::AdminPasswordForm;
pub(crate) use role_form::{RoleForm, RoleFormMode};
pub(crate) use role_permissions_form::RolePermissionsForm;
pub(crate) use role_table::RoleTable;
pub(crate) use user_form::{UserForm, UserFormMode};
pub(crate) use user_roles_form::UserRolesForm;
pub(crate) use user_table::{UserTable, status_key};

use pagetop::prelude::*;

use crate::LOCALES_USER;

// **< constantes compartidas >**********************************************************************

/// Identificador del `<form>` de [`UserForm`] en modo [`UserFormMode::Edit`]. En ese modo, el botón
/// "Guardar" no se renderiza dentro del formulario (ver `UserForm::prepare()`): lo añade la pantalla
/// de edición (`handlers::admin::users::edit_actions()`) junto al resto de acciones, referenciando
/// este id mediante el atributo `form` para seguir enviando el formulario aunque esté fuera de él.
pub(crate) const USER_ADMIN_FORM_ID: &str = "user-admin-form";

// **< tipos compartidos >***************************************************************************

/// Un permiso dentro de un grupo del catálogo: `(clave, etiqueta, concedido)`.
pub(crate) type PermissionItem = (CowStr, Lc, bool);

/// Catálogo de permisos agrupado: `(título del grupo, permisos del grupo)`.
pub(crate) type PermissionGroups = Vec<(Lc, Vec<PermissionItem>)>;

// **< helpers compartidos >*************************************************************************

// `Fieldset` con las casillas para asignar roles (usado en el alta de usuario y en la pantalla
// dedicada de asignación de roles). El rol "authenticated" no se lista como casilla ni se envía:
// todo usuario autenticado lo tiene concedido por definición (ver `session::load_user_from_session`),
// sin necesidad de una fila en `user_role`.
pub(crate) fn roles_fieldset(roles: &[(i32, String, bool)]) -> form::Fieldset {
    let mut field = form::check::Field::new().with_name("role_ids");
    for (role_id, label, checked) in roles {
        field = field.with_item(
            form::check::Item::new(role_id.to_string(), Lc::n(label.clone()))
                .with_checked(*checked),
        );
    }
    form::Fieldset::new()
        .with_legend(Lc::t("field-roles", &LOCALES_USER))
        .with_child(field)
}
