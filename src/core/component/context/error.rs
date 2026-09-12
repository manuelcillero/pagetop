use thiserror::Error;

/// Errores de acceso a parámetros dinámicos del contexto.
#[derive(Debug, Error)]
pub enum ContextError {
    /// La clave no existe.
    #[error("parameter not found")]
    ParamNotFound,
    /// La clave existe, pero el valor guardado no coincide con el tipo solicitado. Incluye
    /// nombre de la clave (`key`), tipo esperado (`expected`) y tipo realmente guardado (`saved`)
    /// para facilitar el diagnóstico.
    #[error("type mismatch for parameter \"{key}\": expected \"{expected}\", found \"{saved}\"")]
    ParamTypeMismatch {
        key: &'static str,
        expected: &'static str,
        saved: &'static str,
    },
}
