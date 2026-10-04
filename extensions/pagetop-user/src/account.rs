//! Tipos en memoria que representan los datos ricos del usuario durante una petición.

use std::collections::HashSet;

use pagetop::prelude::*;

use crate::entity::user;

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
/// Se almacena en las extensiones de la petición HTTP durante la fase de middleware y se accede
/// desde los handlers o desde handlers de [`CheckPermission`] mediante
/// [`HttpRequest::extension::<Account>()`](pagetop::web::HttpRequest::extension).
#[derive(Clone, Debug, Getters)]
pub struct Account {
    /// Identificador del usuario.
    id: i32,
    /// Nombre de usuario.
    username: String,
    /// Correo electrónico.
    email: String,
    /// Nombre para mostrar; cadena vacía si no está definido (ver [`Account::display`]).
    display_name: String,
    /// Estado de la cuenta.
    #[getters(copy)]
    status: UserStatus,
    /// Nombres de máquina de los roles asignados explícitamente. No incluye "authenticated", que
    /// es implícito (ver [`Account::has_role`]).
    roles: Vec<String>,
    // Unión de permisos de sus roles y del rol implícito "authenticated". Vacío si es
    // administrador: no se cargan porque `has_permission()` ya concede todos. Sin getter para que
    // la única consulta posible sea `has_permission()`.
    #[getters(skip)]
    permissions: PermissionSet,
    /// `true` si el usuario tiene acceso sin restricciones (`user.is_admin`), con independencia de
    /// sus roles.
    #[getters(copy)]
    is_admin: bool,
}

impl Account {
    // Sólo se construye desde la carga de sesión, para que ningún otro crate pueda fabricar una
    // cuenta e inyectarla en la petición.
    pub(crate) fn new(user: user::Model, roles: Vec<String>, permissions: PermissionSet) -> Self {
        Account {
            id: user.id,
            username: user.username,
            email: user.email,
            display_name: user.display_name.unwrap_or_default(),
            status: UserStatus::from_i16(user.status),
            roles,
            permissions,
            is_admin: user.is_admin,
        }
    }

    /// Comprueba si la cuenta tiene el permiso indicado, teniendo en cuenta el flag `is_admin`.
    pub fn has_permission(&self, perm: PermissionRef) -> bool {
        self.is_admin() || self.permissions.contains(perm.key().as_ref())
    }

    /// Devuelve el nombre visible: `display_name` si está definido, o `username`.
    pub fn display(&self) -> &str {
        if self.display_name().is_empty() {
            self.username()
        } else {
            self.display_name()
        }
    }

    /// Comprueba si la cuenta tiene el rol indicado ("authenticated" siempre se cumple).
    pub fn has_role(&self, machine_name: &str) -> bool {
        machine_name == "authenticated" || self.roles().iter().any(|r| r == machine_name)
    }
}
