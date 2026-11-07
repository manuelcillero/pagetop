//! Permisos propios de `pagetop-user`.
//!
//! Se declaran en el catálogo de permisos de PageTop ([`PermissionRegistry`]) como los de cualquier
//! otra extensión. Los permisos concedidos a cada rol se guardan en la tabla `role_permissions`.

use pagetop::prelude::*;

use crate::LOCALES_USER;

// **< Permisos integrados de pagetop-user >********************************************************

/// Permisos propios de `pagetop-user`.
#[derive(Clone, Copy, Debug)]
pub enum UserPermission {
    /// Ver el perfil público de otros usuarios (`/user/{id}`): nombre de usuario, nombre visible,
    /// "Sobre mí" y fecha de alta.
    ViewProfiles,
    /// Editar los datos del perfil propio: email, nombre visible, "Sobre mí", idioma, zona horaria
    /// y tema. Nunca los roles, el estado de la cuenta ni el acceso irrestricto.
    EditOwnProfile,
    /// Cambiar el nombre de usuario propio. Complementa a `EditOwnProfile`: el campo está en el
    /// mismo formulario, así que sin aquel no tiene efecto.
    ChangeOwnUsername,
    /// Cambiar la contraseña propia.
    ChangeOwnPassword,
    /// Acceder al mantenimiento de usuarios (listado, alta, edición, asignación de roles,
    /// restablecimiento de contraseñas).
    ///
    /// Es un permiso sensible: quien lo tiene puede asignar cualquier rol y hacerse con cualquier
    /// cuenta que no sea de administrador, así que sólo debe concederse a personas de confianza.
    AdminUsers,
    /// Acceder al mantenimiento de roles (listado, alta, edición, borrado).
    AdminRoles,
    /// Acceder al listado de permisos y, junto con `AdminRoles`, a la asignación de permisos a
    /// roles.
    ///
    /// Es el permiso más sensible del catálogo: con `AdminRoles`, quien lo tiene puede conceder
    /// cualquier permiso al rol "authenticated", que tienen todas las cuentas, incluida la suya.
    AdminPermissions,
    /// Bloquear y activar cuentas de usuario, desde la edición del usuario; requiere también
    /// `AdminUsers`.
    BlockAccounts,
}

impl UserPermission {
    /// Todas las variantes, usado para registrarlas en el catálogo.
    pub const ALL: &'static [Self] = &[
        Self::ViewProfiles,
        Self::EditOwnProfile,
        Self::ChangeOwnUsername,
        Self::ChangeOwnPassword,
        Self::AdminUsers,
        Self::AdminRoles,
        Self::AdminPermissions,
        Self::BlockAccounts,
    ];
}

impl Permission for UserPermission {
    fn key(&self) -> CowStr {
        match self {
            Self::ViewProfiles => "user:view_profiles".into(),
            Self::EditOwnProfile => "user:edit_own_profile".into(),
            Self::ChangeOwnUsername => "user:change_own_username".into(),
            Self::ChangeOwnPassword => "user:change_own_password".into(),
            Self::AdminUsers => "user:admin_users".into(),
            Self::AdminRoles => "user:admin_roles".into(),
            Self::AdminPermissions => "user:admin_permissions".into(),
            Self::BlockAccounts => "user:block_accounts".into(),
        }
    }

    fn label(&self) -> Lc {
        let key = match self {
            Self::ViewProfiles => "perm-view-profiles",
            Self::EditOwnProfile => "perm-edit-own-profile",
            Self::ChangeOwnUsername => "perm-change-own-username",
            Self::ChangeOwnPassword => "perm-change-own-password",
            Self::AdminUsers => "perm-admin-users",
            Self::AdminRoles => "perm-admin-roles",
            Self::AdminPermissions => "perm-admin-permissions",
            Self::BlockAccounts => "perm-block-accounts",
        };
        Lc::t(key, &LOCALES_USER)
    }

    fn group(&self) -> &'static str {
        match self {
            Self::ViewProfiles
            | Self::EditOwnProfile
            | Self::ChangeOwnUsername
            | Self::ChangeOwnPassword => GROUP_USERS,
            Self::AdminUsers | Self::AdminRoles | Self::AdminPermissions | Self::BlockAccounts => {
                GROUP_ADMINISTRATION
            }
        }
    }

    fn group_label(&self) -> Lc {
        builtin_group_label(self.group())
    }
}

const GROUP_USERS: &str = "users";
const GROUP_ADMINISTRATION: &str = "administration";

// Título traducible de un grupo de permisos integrado.
fn builtin_group_label(group: &str) -> Lc {
    let key = match group {
        GROUP_USERS => "group-users",
        GROUP_ADMINISTRATION => "group-administration",
        _ => unreachable!("unknown built-in permission group: {group}"),
    };
    Lc::t(key, &LOCALES_USER)
}

/// Registra los permisos propios de `pagetop-user`.
pub fn declare_builtin_permissions(r: &mut PermissionRegistry) {
    for permission in UserPermission::ALL {
        r.register(permission);
    }
}
