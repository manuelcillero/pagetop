//! Errores de `pagetop-user`.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("account is blocked")]
    AccountBlocked,

    #[error("account is pending email verification")]
    AccountPending,

    #[error("account is temporarily locked due to too many failed login attempts")]
    AccountLocked,

    #[error("password hashing failed: {0}")]
    PasswordHash(String),

    #[error("database error: {0}")]
    Database(#[from] pagetop_seaorm::db::DbErr),

    #[error("token is invalid or has expired")]
    InvalidToken,

    #[error("username is already taken")]
    UsernameTaken,

    #[error("username does not meet the identity policy (user.identity_policy)")]
    InvalidUsername,

    #[error("invalid email address")]
    InvalidEmail,

    #[error("invalid setting \"{key}\"")]
    InvalidSetting {
        key: &'static str,
        #[source]
        source: Box<AuthError>,
    },

    #[error("email is already registered")]
    EmailTaken,

    #[error("passwords do not match")]
    PasswordMismatch,

    #[error("password must be at least {0} characters")]
    PasswordTooShort(usize),

    #[error("invalid IANA timezone identifier")]
    InvalidTimezone,

    #[error("unsupported language identifier")]
    InvalidLanguage,

    #[error("unknown or disabled theme")]
    InvalidTheme,

    #[error("about text must be at most {0} characters")]
    AboutTooLong(usize),

    #[error("display name must be at most {0} characters")]
    DisplayNameTooLong(usize),

    #[error("user not found")]
    UserNotFound,

    #[error("role not found")]
    RoleNotFound,

    #[error("role machine name is already taken")]
    RoleMachineNameTaken,

    #[error(
        "role machine name must contain only lowercase letters, digits and underscores, up to 64"
    )]
    InvalidMachineName,

    #[error("role label must not be blank and must be at most {0} characters")]
    InvalidRoleLabel(usize),

    #[error("role is locked and cannot be modified or deleted")]
    RoleLocked,

    #[error("role has users assigned and cannot be deleted")]
    RoleInUse,

    #[error("cannot remove the last administrator")]
    LastAdministrator,

    #[error("cannot block your own account")]
    CannotBlockSelf,

    #[error("cannot modify your own administrator flag")]
    CannotModifyOwnAdminFlag,

    #[error("unknown permission key: {0}")]
    UnknownPermission(String),
}
