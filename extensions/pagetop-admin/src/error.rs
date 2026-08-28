use thiserror::Error;

/// Errores que puede producir la extensión `pagetop-admin`.
#[derive(Debug, Error)]
pub enum AdminError {
    #[error("settings DB error: {0}")]
    DbError(#[from] pagetop_seaorm::db::DbErr),

    #[error("settings serialization error: {0}")]
    SerializeError(#[from] serde_json::Error),

    #[error("setting key not found: {0}")]
    NotFound(String),
}
