//! Acciones para conectar las extensiones de autenticación y declarar permisos.
//!
//! Con ellas una extensión concede los permisos al usuario actual ([`CheckPermission`]), da a
//! conocer los suyos en el catálogo ([`DeclarePermissions`]) o decide qué responder a un usuario
//! anónimo sin acceso a una ruta ([`AnonymousDenied`]). El resto del sistema de autorización está
//! en el módulo [`auth`](crate::auth).

use crate::auth::{PermissionRef, PermissionRegistry};
use crate::web::{HttpRequest, Response};

/// Tipo de función para comprobar si el usuario actual tiene un permiso concreto.
///
/// Se invoca con:
///
/// - `request`: petición HTTP desde la que se accede a los datos inyectados por el middleware de
///   autenticación.
/// - `perm`: permiso a comprobar; el handler usará
///   [`Permission::key()`](crate::auth::Permission::key) para identificarlo contra su propio
///   modelo de permisos.
/// - `granted`: referencia mutable; el handler debe asignarla a `true` si concede el permiso.
pub type FnActionCheckPerm = fn(request: &HttpRequest, perm: PermissionRef, granted: &mut bool);

/// Tipo de función para declarar permisos en el catálogo.
pub type FnActionDeclarePermissions = fn(registry: &mut PermissionRegistry);

/// Tipo de función para responder a un usuario anónimo sin acceso a una ruta.
///
/// Devuelve la respuesta (p. ej. una redirección al formulario de inicio de sesión), o `None` para
/// dejar que responda otra acción o, si ninguna lo hace, la página de acceso denegado.
pub type FnActionAnonymousDenied = fn(request: &HttpRequest) -> Option<Response>;

mod check_permission;
pub use check_permission::*;

mod declare_permissions;
pub use declare_permissions::*;

mod anonymous_denied;
pub use anonymous_denied::*;
