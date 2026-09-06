use thiserror::Error;

/// Errores de acceso a valores extra de [`Props`](crate::html::props::Props).
#[derive(Debug, PartialEq, Eq, Error)]
pub enum PropsError {
    /// La clave no existe. Incluye la clave (`key`).
    #[error("extra \"{key}\" not found")]
    ExtraNotFound { key: &'static str },
    /// La clave existe pero el tipo solicitado no coincide con el almacenado. Incluye la clave
    /// (`key`), tipo esperado (`expected`) y tipo realmente encontrado (`found`) para facilitar el
    /// diagnóstico.
    #[error("type mismatch for extra \"{key}\": expected \"{expected}\", found \"{found}\"")]
    ExtraTypeMismatch {
        key: &'static str,
        expected: &'static str,
        found: &'static str,
    },
}
