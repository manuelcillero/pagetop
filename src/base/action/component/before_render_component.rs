use crate::prelude::*;

use super::FnActionWithComponent;

/// Ejecuta [`FnActionWithComponent`] antes de renderizar el componente.
pub struct BeforeRender<C: Component> {
    f: FnActionWithComponent<C>,
    referer: ActionReferer,
    weight: Weight,
}

// Filtro para despachar `FnActionWithComponent` antes de renderizar un componente `C`.
impl<C: Component> ActionDispatcher for BeforeRender<C> {
    fn referer(&self) -> Option<&ActionReferer> {
        Some(&self.referer)
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl<C: Component> BeforeRender<C> {
    /// Permite [registrar](Extension::actions) una nueva acción [`FnActionWithComponent`].
    pub fn new(f: FnActionWithComponent<C>) -> Self {
        BeforeRender {
            f,
            referer: ActionReferer::of::<C>(),
            weight: 0,
        }
    }

    /// Afina el registro para ejecutar la acción [`FnActionWithComponent`] sólo para el componente
    /// `C` con identificador `id`.
    pub fn filter_by_referer_id(mut self, id: impl AsRef<str>) -> Self {
        self.referer = self.referer.with_id(id);
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
        dispatch_referer(component, |c| c.id(), |action: &Self, c| (action.f)(c, cx));
    }
}
