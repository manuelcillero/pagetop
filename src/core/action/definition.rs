use crate::core::AnyInfo;
use crate::{UniqueId, Weight};

/// Tipo dinámico para encapsular cualquier acción que implementa [`ActionTrait`].
pub type ActionBox = Box<dyn ActionTrait>;

/// Identifica una acción con una clave que define las condiciones de selección de las funciones
/// asociadas a esa acción.
///
/// Las funciones seleccionadas se van a [despachar](crate::core::action::dispatch_actions) y
/// ejecutar en un punto concreto del flujo de ejecución.
///
/// # Campos
///
/// - `action_type_id`: Tipo de la acción.
/// - `theme_type_id`: Opcional, filtra las funciones para un tema dado.
/// - `referer_type_id`: Opcional, filtra las funciones para un tipo dado (p.ej. para un tipo de
///   componente).
/// - `referer_id`: Opcional, filtra las funciones por el identificador de una instancia (p.ej. para
///   un formulario concreto).
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
    /// Esta clave permite seleccionar las funciones a ejecutar para ese tipo de acción con filtros
    /// opcionales por tema, un tipo de referencia, o una instancia concreta según su identificador.
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

/// Trait base que permite obtener la clave ([`ActionKey`]) asociada a una acción.
///
/// Implementado automáticamente para cualquier tipo que cumpla [`ActionTrait`].
pub trait ActionBase {
    fn key(&self) -> ActionKey;
}

/// Interfaz común que deben implementar las acciones del código que pueden ser modificadas.
///
/// Este trait combina:
/// - [`AnyInfo`] para identificación única del tipo en tiempo de ejecución.
/// - `Send + Sync` para permitir uso concurrente seguro.
///
/// # Métodos personalizables
/// - `theme_type_id()`: Asocia la acción a un tipo concreto de tema (si aplica).
/// - `referer_type_id()`: Asocia la acción a un tipo de objeto referente (si aplica).
/// - `referer_id()`: Asocia la acción a un identificador concreto.
/// - `weight()`: Controla el orden de aplicación de acciones; valores más bajos se ejecutan antes.
pub trait ActionTrait: ActionBase + AnyInfo + Send + Sync {
    /// Especifica el tipo de tema asociado. Por defecto `None`.
    fn theme_type_id(&self) -> Option<UniqueId> {
        None
    }

    /// Especifica el tipo del objeto referente. Por defecto `None`.
    fn referer_type_id(&self) -> Option<UniqueId> {
        None
    }

    /// Especifica un identificador único del objeto referente. Por defecto `None`.
    fn referer_id(&self) -> Option<String> {
        None
    }

    /// Define el peso lógico de la acción para determinar el orden de aplicación.
    ///
    /// Acciones con pesos más bajos se aplicarán antes. Se pueden usar valores negativos. Por
    /// defecto es `0`.
    fn weight(&self) -> Weight {
        0
    }
}

// Implementación automática que construye la clave `ActionKey` a partir de los métodos definidos.
impl<A: ActionTrait> ActionBase for A {
    fn key(&self) -> ActionKey {
        ActionKey {
            action_type_id: self.type_id(),
            theme_type_id: self.theme_type_id(),
            referer_type_id: self.referer_type_id(),
            referer_id: self.referer_id(),
        }
    }
}
