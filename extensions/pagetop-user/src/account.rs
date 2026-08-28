//! Tipos en memoria que representan los datos ricos del usuario durante una petición.

use std::collections::HashSet;

use pagetop::auth::PermissionRef;

// **< UserStatus >*********************************************************************************

/// Estado de la cuenta de usuario.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UserStatus {
    Active,
    Blocked,
    Pending,
}

impl UserStatus {
    pub fn from_i16(v: i16) -> Self {
        match v {
            1 => UserStatus::Active,
            2 => UserStatus::Pending,
            _ => UserStatus::Blocked,
        }
    }

    pub fn as_i16(self) -> i16 {
        match self {
            UserStatus::Blocked => 0,
            UserStatus::Active => 1,
            UserStatus::Pending => 2,
        }
    }
}

// **< PermissionSet >******************************************************************************

/// Conjunto de permisos resuelto para un usuario concreto.
#[derive(Clone, Debug, Default)]
pub struct PermissionSet(HashSet<String>);

impl PermissionSet {
    pub fn new(keys: impl IntoIterator<Item = String>) -> Self {
        PermissionSet(keys.into_iter().collect())
    }

    pub fn contains(&self, key: &str) -> bool {
        self.0.contains(key)
    }

    pub fn extend(&mut self, keys: impl IntoIterator<Item = String>) {
        self.0.extend(keys);
    }
}

// **< Account >************************************************************************************

/// Datos ricos del usuario autenticado inyectados por el middleware de sesión.
///
/// Se almacena en las extensiones de la petición HTTP durante la fase de middleware y se
/// accede desde los handlers o desde handlers de [`CheckPermission`](pagetop::auth::CheckPermission)
/// mediante [`HttpRequest::extension::<Account>()`](pagetop::web::HttpRequest::extension).
#[derive(Clone, Debug)]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub status: UserStatus,
    /// Nombres de máquina de los roles asignados, incluido "authenticated" (se asigna
    /// automáticamente a toda cuenta en el alta, ver `auth::assign_role`).
    pub roles: Vec<String>,
    /// Unión de permisos de todos sus roles.
    pub permissions: PermissionSet,
    /// `true` si alguno de sus roles tiene `is_admin = true`.
    pub is_admin: bool,
}

impl Account {
    /// Comprueba si la cuenta tiene el permiso indicado, teniendo en cuenta el flag `is_admin`.
    pub fn has_permission(&self, perm: PermissionRef) -> bool {
        self.is_admin || self.permissions.contains(perm.key().as_ref())
    }

    /// Devuelve el nombre visible: `display_name` si está definido, o `username`.
    pub fn display(&self) -> &str {
        if self.display_name.is_empty() {
            &self.username
        } else {
            &self.display_name
        }
    }

    /// Comprueba si la cuenta tiene el rol indicado.
    pub fn has_role(&self, machine_name: &str) -> bool {
        self.roles.iter().any(|r| r == machine_name)
    }
}
