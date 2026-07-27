use crate::prelude::*;

use super::FnActionWithComponent;

/// Ejecuta [`FnActionWithComponent`] después de renderizar un componente.
pub struct AfterRender<C: Component> {
    f: FnActionWithComponent<C>,
    referer_type_id: Option<UniqueId>,
    referer_id: Option<String>,
    weight: Weight,
}

// Filtro para despachar `FnActionWithComponent` después de renderizar un componente `C`.
impl<C: Component> ActionDispatcher for AfterRender<C> {
    fn referer_type_id(&self) -> Option<UniqueId> {
        self.referer_type_id
    }

    fn referer_id(&self) -> Option<String> {
        self.referer_id.clone()
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl<C: Component> AfterRender<C> {
    /// Permite [registrar](Extension::actions) una nueva acción [`FnActionWithComponent`].
    pub fn new(f: FnActionWithComponent<C>) -> Self {
        AfterRender {
            f,
            referer_type_id: Some(UniqueId::of::<C>()),
            referer_id: None,
            weight: 0,
        }
    }

    /// Afina el registro para ejecutar la acción [`FnActionWithComponent`] sólo para el componente
    /// `C` con identificador `id`.
    pub fn filter_by_referer_id(mut self, id: impl AsRef<str>) -> Self {
        self.referer_id = util::normalize_token(id);
        self
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    /// Despacha las acciones.
    #[inline]
    pub(crate) fn dispatch(component: &mut C, cx: &mut Context) {
        // Primero despacha las acciones para el tipo de componente.
        dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), Some(UniqueId::of::<C>()), None),
            |action: &Self| (action.f)(component, cx),
        );

        // Y luego despacha las acciones para el tipo de componente con un identificador dado.
        if let Some(id) = component.id() {
            dispatch_actions(
                &ActionKey::new(UniqueId::of::<Self>(), Some(UniqueId::of::<C>()), Some(id)),
                |action: &Self| (action.f)(component, cx),
            );
        }
    }
}
