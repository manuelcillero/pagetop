use crate::core::AnyInfo;
use crate::{Getters, UniqueId, Weight, util};

/// Tipo dinámico para encapsular cualquier acción que implementa [`ActionDispatcher`].
pub type ActionBox = Box<dyn ActionDispatcher>;

/// Referente de una acción: el tipo de objeto sobre el que actúa (p. ej. un tipo de componente) y,
/// opcionalmente, el identificador de una instancia concreta.
///
/// Las acciones con referente se despachan con [`dispatch_referer()`]. Sin identificador afectan a
/// cualquier objeto del tipo; con identificador, sólo al que lo tiene.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// let any_button = ActionReferer::of::<Button>();
/// assert_eq!(any_button.id(), None);
///
/// // El identificador se normaliza; uno en blanco equivale a no tenerlo.
/// let one = ActionReferer::of::<Button>().with_id("  My Id ");
/// assert_eq!(one.id(), Some("my_id"));
/// assert_eq!(one.referer_type_id(), any_button.referer_type_id());
/// assert_eq!(one.with_id("   "), any_button);
/// ```
///
/// [`dispatch_referer()`]: crate::core::action::dispatch_referer
#[derive(Clone, Debug, Eq, Getters, PartialEq)]
pub struct ActionReferer {
    #[getters(copy)]
    referer_type_id: UniqueId,
    #[getters(skip)]
    id: Option<String>,
}

impl ActionReferer {
    /// Referente para cualquier objeto del tipo `R`.
    pub fn of<R: 'static>() -> Self {
        ActionReferer {
            referer_type_id: UniqueId::of::<R>(),
            id: None,
        }
    }

    /// Identificador de la instancia a la que se restringe el referente, si lo hay.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// Restringe el referente al objeto con el identificador indicado. Si el identificador queda
    /// vacío tras normalizarlo, el referente vuelve a aplicarse a cualquier objeto del tipo.
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id = util::normalize_token(id);
        self
    }
}

/// Define el comportamiento de una acción con su referente y su peso de ejecución.
///
/// Las acciones sobrescriben [`referer()`](Self::referer) si sólo se aplican a un tipo de objeto
/// (y, opcionalmente, a una instancia concreta). Por defecto no tienen referente.
pub trait ActionDispatcher: AnyInfo + Send + Sync {
    /// Devuelve el [`ActionReferer`] de la acción, o `None` si no actúa sobre un tipo de objeto
    /// concreto.
    fn referer(&self) -> Option<&ActionReferer> {
        None
    }

    /// Devuelve el peso para definir el orden de ejecución.
    fn weight(&self) -> Weight {
        0
    }
}
