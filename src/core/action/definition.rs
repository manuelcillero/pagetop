use crate::core::AnyInfo;
use crate::{UniqueId, Weight};

/// Tipo dinámico para encapsular cualquier acción que implementa [`ActionDispatcher`].
pub type ActionBox = Box<dyn ActionDispatcher>;

/// Clave para registrar las acciones y seleccionar las funciones asociadas.
///
/// Las funciones seleccionadas se van a [despachar](crate::core::action::dispatch_actions) y
/// ejecutar en un punto concreto del flujo de ejecución.
#[derive(Eq, PartialEq, Hash)]
pub struct ActionKey {
    action_type_id: UniqueId,
    theme_type_id: Option<UniqueId>,
    referer_type_id: Option<UniqueId>,
    referer_id: Option<String>,
}

impl ActionKey {
    /// Crea una nueva clave para un tipo de acción.
    ///
    /// Se crea con los siguientes campos:
    ///
    /// - `action_type_id`: Tipo de la acción.
    /// - `theme_type_id`: Opcional, identificador de tipo ([`UniqueId`]) del tema asociado.
    /// - `referer_type_id`: Opcional, identificador de tipo ([`UniqueId`]) del componente referido.
    /// - `referer_id`: Opcional, identificador de la instancia (p.ej. para un formulario concreto).
    ///
    /// Esta clave permitirá seleccionar las funciones a ejecutar para ese tipo de acción, con
    /// filtros opcionales por tema, componente, o una instancia concreta según su identificador.
    pub fn new(
        action_type_id: UniqueId,
        theme_type_id: Option<UniqueId>,
        referer_type_id: Option<UniqueId>,
        referer_id: Option<String>,
    ) -> Self {
        ActionKey {
            action_type_id,
            theme_type_id,
            referer_type_id,
            referer_id,
        }
    }
}

/// Implementa el filtro predeterminado para despachar las funciones de una acción dada.
///
/// Las acciones tienen que sobrescribir los métodos para el filtro que apliquen. Por defecto
/// implementa un filtro nulo.
pub trait ActionDispatcher: AnyInfo + Send + Sync {
    /// Identificador de tipo ([`UniqueId`]) del tema asociado. En este caso devuelve `None`.
    fn theme_type_id(&self) -> Option<UniqueId> {
        None
    }

    /// Identificador de tipo ([`UniqueId`]) del objeto referido. En este caso devuelve `None`.
    fn referer_type_id(&self) -> Option<UniqueId> {
        None
    }

    /// Identificador del objeto referido. En este caso devuelve `None`.
    fn referer_id(&self) -> Option<String> {
        None
    }

    /// Funciones con pesos más bajos se aplican antes. En este caso siempre devuelve `0`.
    fn weight(&self) -> Weight {
        0
    }
}
