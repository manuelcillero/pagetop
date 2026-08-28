//! Catálogo de permisos en memoria y acción `DeclarePermissions`.
//!
//! El catálogo se construye una sola vez durante `Extension::initialize()` a partir de
//! las acciones `DeclarePermissions` registradas por todas las extensiones. Los permisos
//! otorgados a cada rol se persisten en la tabla `role_permissions`; la definición
//! de qué permisos existen vive únicamente en memoria.

use std::sync::OnceLock;

use pagetop::prelude::*;

use crate::LOCALES_USER;

// **< DeclarePermissions >*************************************************************************

/// Acción que las extensiones dispatcean para registrar sus permisos en el catálogo.
///
/// Cada extensión añade una instancia `DeclarePermissions::new(fn)` en `Extension::actions()`.
/// Durante `initialize()`, [`build_registry()`] despacha todas las instancias registradas
/// y construye el catálogo global.
///
/// `label()`, `group()` y `group_label()` los aporta el propio [`Permission`] registrado
/// (con implementación por defecto si la extensión no los sobrecarga).
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::auth::Permission;
/// # use pagetop::CowStr;
/// # use pagetop_user::permission::{DeclarePermissions, PermissionRegistry};
/// #[derive(Clone, Copy, Debug)]
/// enum MyPermission {
///     DoSomething,
/// }
///
/// impl Permission for MyPermission {
///     fn key(&self) -> CowStr {
///         match self {
///             Self::DoSomething => "myext:do_something".into(),
///         }
///     }
/// }
///
/// fn my_permissions(registry: &mut PermissionRegistry) {
///     registry.register(&MyPermission::DoSomething);
/// }
/// // En Extension::actions():
/// // DeclarePermissions::new(my_permissions)
/// ```
pub struct DeclarePermissions {
    pub(crate) handler: fn(&mut PermissionRegistry),
}

impl DeclarePermissions {
    pub fn new(handler: fn(&mut PermissionRegistry)) -> Self {
        DeclarePermissions { handler }
    }

    /// Despacha todas las acciones `DeclarePermissions` registradas construyendo el catálogo.
    pub(crate) fn dispatch(registry: &mut PermissionRegistry) {
        dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), None, None),
            |action: &Self| (action.handler)(registry),
        );
    }
}

impl ActionDispatcher for DeclarePermissions {}

// **< PermissionRegistry >*************************************************************************

/// Catálogo mutable de permisos, construido durante la fase de inicialización.
///
/// Un `Vec` basta: el catálogo se construye una sola vez con un puñado de entradas y se recorre
/// entero en la UI de administración, así que conserva el orden de registro sin estructuras
/// adicionales y sin el coste de mantenerlas sincronizadas.
#[derive(Default)]
pub struct PermissionRegistry {
    permissions: Vec<PermissionRef>,
    /// `(identificador de grupo, título traducible)`, en orden de primer registro.
    groups: Vec<(&'static str, Lc)>,
}

impl PermissionRegistry {
    /// Registra un permiso. Se ignora si ya está registrado.
    pub fn register(&mut self, perm: PermissionRef) {
        let group = perm.group();
        if !self.groups.iter().any(|(g, _)| *g == group) {
            self.groups.push((group, perm.group_label()));
        }
        if !self.has(perm) {
            self.permissions.push(perm);
        }
    }

    /// Comprueba si un permiso está en el catálogo.
    pub fn has(&self, perm: PermissionRef) -> bool {
        self.permissions.iter().any(|p| p.key() == perm.key())
    }

    /// Comprueba si una clave textual (p. ej. procedente de un formulario) corresponde a un
    /// permiso del catálogo.
    pub fn has_key(&self, key: &str) -> bool {
        self.permissions.iter().any(|p| p.key().as_ref() == key)
    }

    pub fn all(&self) -> impl Iterator<Item = PermissionRef> + '_ {
        self.permissions.iter().copied()
    }

    pub fn groups(&self) -> &[(&'static str, Lc)] {
        &self.groups
    }

    pub fn by_group<'a>(&'a self, group: &'a str) -> impl Iterator<Item = PermissionRef> + 'a {
        self.permissions
            .iter()
            .copied()
            .filter(move |p| p.group() == group)
    }
}

// **< Catálogo global >****************************************************************************

static PERMISSIONS: OnceLock<PermissionRegistry> = OnceLock::new();

/// Construye y almacena el catálogo global de permisos ejecutando todas las acciones
/// `DeclarePermissions` registradas. Se llama exactamente una vez desde `initialize()`.
pub fn build_registry() {
    let mut registry = PermissionRegistry::default();
    DeclarePermissions::dispatch(&mut registry);
    let _ = PERMISSIONS.set(registry);
}

/// Devuelve el catálogo global ya construido.
///
/// Entra en pánico si se llama antes de `build_registry()`.
pub fn registry() -> &'static PermissionRegistry {
    PERMISSIONS
        .get()
        .expect("permission registry not initialized")
}

// **< Permisos integrados de pagetop-user >********************************************************

/// Permisos propios de `pagetop-user`.
#[derive(Clone, Copy, Debug)]
pub enum UserPermission {
    /// Iniciar sesión.
    Login,
    /// Registrar una cuenta nueva.
    Register,
    /// Ver perfiles de otros usuarios.
    ViewProfiles,
    /// Editar el perfil propio.
    EditOwnProfile,
    /// Cambiar la contraseña propia.
    ChangeOwnPassword,
    /// Acceder al mantenimiento de usuarios (listado, alta, edición).
    AdminUsers,
    /// Acceder al mantenimiento de roles (listado, alta, edición, borrado).
    AdminRoles,
    /// Acceder al listado de permisos y a la asignación de permisos a roles.
    AdminPermissions,
    /// Bloquear y desbloquear cuentas de usuario.
    BlockAccounts,
    /// Asignar roles a usuarios.
    AssignRoles,
}

impl UserPermission {
    /// Todas las variantes, usado para registrarlas en el catálogo.
    pub const ALL: &'static [Self] = &[
        Self::Login,
        Self::Register,
        Self::ViewProfiles,
        Self::EditOwnProfile,
        Self::ChangeOwnPassword,
        Self::AdminUsers,
        Self::AdminRoles,
        Self::AdminPermissions,
        Self::BlockAccounts,
        Self::AssignRoles,
    ];
}

impl Permission for UserPermission {
    fn key(&self) -> CowStr {
        match self {
            Self::Login => "user:login".into(),
            Self::Register => "user:register".into(),
            Self::ViewProfiles => "user:view_profiles".into(),
            Self::EditOwnProfile => "user:edit_own_profile".into(),
            Self::ChangeOwnPassword => "user:change_own_password".into(),
            Self::AdminUsers => "user:admin_users".into(),
            Self::AdminRoles => "user:admin_roles".into(),
            Self::AdminPermissions => "user:admin_permissions".into(),
            Self::BlockAccounts => "user:block_accounts".into(),
            Self::AssignRoles => "user:assign_roles".into(),
        }
    }

    fn label(&self) -> Lc {
        let key = match self {
            Self::Login => "perm-login",
            Self::Register => "perm-register",
            Self::ViewProfiles => "perm-view-profiles",
            Self::EditOwnProfile => "perm-edit-own-profile",
            Self::ChangeOwnPassword => "perm-change-own-password",
            Self::AdminUsers => "perm-admin-users",
            Self::AdminRoles => "perm-admin-roles",
            Self::AdminPermissions => "perm-admin-permissions",
            Self::BlockAccounts => "perm-block-accounts",
            Self::AssignRoles => "perm-assign-roles",
        };
        Lc::t(key, &LOCALES_USER)
    }

    fn group(&self) -> &'static str {
        match self {
            Self::Login
            | Self::Register
            | Self::ViewProfiles
            | Self::EditOwnProfile
            | Self::ChangeOwnPassword => GROUP_USERS,
            Self::AdminUsers
            | Self::AdminRoles
            | Self::AdminPermissions
            | Self::BlockAccounts
            | Self::AssignRoles => GROUP_ADMINISTRATION,
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
        _ => unreachable!("grupo de permisos integrado desconocido: {group}"),
    };
    Lc::t(key, &LOCALES_USER)
}

/// Registra los permisos propios de `pagetop-user`.
pub fn declare_builtin_permissions(r: &mut PermissionRegistry) {
    for permission in UserPermission::ALL {
        r.register(permission);
    }
}
