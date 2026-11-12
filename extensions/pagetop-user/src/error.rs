//! Errores de `pagetop-user`.

use thiserror::Error;

use crate::provider::RejectReason;

/// Definición de errores.
///
/// Incluye errores de las credenciales y el estado de las cuentas, de los datos que no superan su
/// validación, de la administración de usuarios y roles, de los proveedores de autenticación y de
/// la configuración.
///
/// El mensaje de cada error (`Display`) está en inglés, para las trazas; los formularios muestran
/// al usuario un texto traducido.
#[derive(Debug, Error)]
pub enum AuthError {
    /// El usuario o la contraseña no son correctos (o no hay cuenta con ese usuario).
    #[error("invalid credentials")]
    InvalidCredentials,

    /// La cuenta está bloqueada por un administrador o por el proveedor que la gestiona.
    #[error("account is blocked")]
    AccountBlocked,

    /// La cuenta está pendiente de que su titular confirme el email.
    #[error("account is pending email verification")]
    AccountPending,

    /// La cuenta está bloqueada temporalmente por demasiados intentos fallidos.
    #[error("account is temporarily locked due to too many failed login attempts")]
    AccountLocked,

    /// No se ha podido calcular o comprobar el *hash* de una contraseña.
    #[error("password hashing failed: {0}")]
    PasswordHash(String),

    /// Error de la base de datos.
    #[error("database error: {0}")]
    Database(#[from] pagetop_seaorm::db::DbErr),

    /// El enlace (*token*) de un solo uso no es válido o ha caducado.
    #[error("token is invalid or has expired")]
    InvalidToken,

    /// El nombre de usuario ya lo usa otra cuenta, o uno que se confunde con él.
    #[error("username is already taken")]
    UsernameTaken,

    /// El nombre de usuario no cumple la política de identidad configurada.
    #[error("username does not meet the identity policy (user.identity_policy)")]
    InvalidUsername,

    /// La dirección de email no es válida.
    #[error("invalid email address")]
    InvalidEmail,

    /// Un ajuste de la configuración no es válido; `source` dice por qué.
    #[error("invalid setting \"{key}\"")]
    InvalidSetting {
        /// Clave del ajuste, p. ej. `user.seed.admin_email`.
        key: &'static str,
        /// Motivo.
        #[source]
        source: Box<AuthError>,
    },

    /// La dirección de email ya la usa otra cuenta.
    #[error("email is already registered")]
    EmailTaken,

    /// La nueva dirección de email es la misma que la actual.
    #[error("new email is the same as the current one")]
    SameEmail,

    /// Se ha alcanzado un límite de solicitudes o de intentos fallidos.
    #[error("too many requests")]
    TooManyRequests,

    /// La contraseña y su confirmación no coinciden.
    #[error("passwords do not match")]
    PasswordMismatch,

    /// La contraseña no llega a la longitud mínima indicada.
    #[error("password must be at least {0} characters")]
    PasswordTooShort(usize),

    /// La zona horaria no es una de las que ofrece la aplicación.
    #[error("invalid IANA timezone identifier")]
    InvalidTimezone,

    /// El idioma no es uno de los que ofrece la aplicación.
    #[error("unsupported language identifier")]
    InvalidLanguage,

    /// El tema no existe o no está habilitado.
    #[error("unknown or disabled theme")]
    InvalidTheme,

    /// El texto «Sobre mí» supera la longitud máxima indicada.
    #[error("about text must be at most {0} characters")]
    AboutTooLong(usize),

    /// El nombre visible supera la longitud máxima indicada.
    #[error("display name must be at most {0} characters")]
    DisplayNameTooLong(usize),

    /// La cuenta no existe.
    #[error("user not found")]
    UserNotFound,

    /// El rol no existe.
    #[error("role not found")]
    RoleNotFound,

    /// El nombre interno del rol ya lo usa otro rol.
    #[error("role machine name is already taken")]
    RoleMachineNameTaken,

    /// El nombre interno del rol no es válido.
    #[error(
        "role machine name must contain only lowercase letters, digits and underscores, up to 64"
    )]
    InvalidMachineName,

    /// La etiqueta del rol está vacía o supera la longitud máxima indicada.
    #[error("role label must not be blank and must be at most {0} characters")]
    InvalidRoleLabel(usize),

    /// El rol está protegido (`locked`, como los de sistema) y no se puede modificar ni eliminar.
    #[error("role is locked and cannot be modified or deleted")]
    RoleLocked,

    /// El rol tiene usuarios asignados y no se puede eliminar.
    #[error("role has users assigned and cannot be deleted")]
    RoleInUse,

    /// Es la única cuenta de administrador, y no se puede bloquear.
    #[error("cannot remove the last administrator")]
    LastAdministrator,

    /// Nadie puede bloquear su propia cuenta.
    #[error("cannot block your own account")]
    CannotBlockSelf,

    /// Nadie puede concederse ni retirarse a sí mismo el acceso total.
    #[error("cannot modify your own administrator flag")]
    CannotModifyOwnAdminFlag,

    /// La clave de permiso no está en el catálogo.
    #[error("unknown permission key: {0}")]
    UnknownPermission(String),

    /// La cuenta espera a que un administrador la apruebe.
    #[error("account is awaiting approval by an administrator")]
    AccountAwaitingApproval,

    /// Un proveedor de autenticación no ha podido comprobar las credenciales.
    #[error("the authentication provider is not available")]
    ExternalUnavailable,

    /// Un proveedor de autenticación deniega el acceso, con su motivo.
    #[error("access rejected by the authentication provider: {0:?}")]
    ExternalRejected(RejectReason),

    /// La cuenta no está vinculada al proveedor que intenta modificarla.
    #[error("the account is not linked to this authentication provider")]
    NotOwnedByProvider,

    /// La cuenta no espera a ser vinculada con esa persona por ese proveedor.
    #[error(
        "the account is not waiting to be linked with this person by this authentication provider"
    )]
    NotPendingIdentity,

    /// Otra cuenta ya está vinculada a esa identidad del proveedor.
    #[error("another account is already linked to this external identity")]
    IdentityTaken,

    /// La operación haría que un proveedor gestionara una cuenta de administrador.
    #[error("administrator accounts cannot be managed by an authentication provider")]
    ProtectedAccount,

    /// Un proveedor sólo puede crear cuentas activas o pendientes de aprobación.
    #[error(
        "an authentication provider can only create active accounts or accounts awaiting approval"
    )]
    InvalidStatus,

    /// `[user.login] sources` está vacía.
    #[error("[user.login] sources must not be empty")]
    NoLoginSources,

    /// `[user.login] sources` repite una fuente.
    #[error("[user.login] sources has \"{0}\" more than once")]
    DuplicateLoginSource(String),

    /// `[user.login] sources` incluye una fuente que no es `"local"` ni un proveedor declarado.
    #[error(
        "[user.login] sources has \"{0}\", which is neither \"local\" nor a declared \
         authentication provider"
    )]
    UnknownLoginSource(String),

    /// La contraseña de la cuenta la gestiona un proveedor y no se puede cambiar aquí.
    #[error("the password is managed by an external authentication provider")]
    PasswordManagedExternally,
}
