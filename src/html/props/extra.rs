use std::any::Any;
use std::fmt;
use std::sync::Arc;

/// Encapsula un valor tipado extra para almacenar en [`Props`](crate::html::props::Props).
///
/// Internamente usa [`Arc`] para que [`Props`](crate::html::props::Props) pueda implementar
/// [`Clone`] sin requerir que los valores almacenados sean clonables. El nombre del tipo
/// almacenado permite generar mensajes de error precisos.
pub struct PropsExtra {
    // `pub(super)`: `PropsOp::set_extra()` (en el módulo hermano `op`) construye este valor
    // directamente con un literal de struct.
    pub(super) value: Arc<dyn Any + Send + Sync>,
    pub(super) type_name: &'static str,
}

impl Clone for PropsExtra {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            type_name: self.type_name,
        }
    }
}

impl fmt::Debug for PropsExtra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.type_name)
    }
}
