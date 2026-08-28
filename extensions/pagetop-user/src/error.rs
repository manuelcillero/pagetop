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

    #[error("email is already registered")]
    EmailTaken,

    #[error("passwords do not match")]
    PasswordMismatch,

    #[error("password must be at least {0} characters")]
    PasswordTooShort(usize),

    #[error("user not found")]
    UserNotFound,

    #[error("role not found")]
    RoleNotFound,

    #[error("role machine name is already taken")]
    RoleMachineNameTaken,

    #[error("role machine name must contain only lowercase letters, digits and underscores")]
    InvalidMachineName,

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
