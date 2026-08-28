//! Tipos de error de `pagetop-menu`.

use thiserror::Error;

/// Errores que puede producir `pagetop-menu`.
#[derive(Debug, Error)]
pub enum MenuError {
    #[error("invalid machine name: {0}")]
    InvalidName(String),

    #[error("database error: {0}")]
    Database(#[from] pagetop_seaorm::db::DbErr),
}
