use crate::prelude::*;

use crate::base::action::FnActionWithComponent;

/// Ejecuta [`FnActionWithComponent`] después de renderizar un componente.
pub struct AfterRender<C: Component> {
    f: FnActionWithComponent<C>,
    referer_type_id: Option<UniqueId>,
    referer_id: AttrId,
    weight: Weight,
}

/// Filtro para despachar [`FnActionWithComponent`] después de renderizar un componente `C`.
impl<C: Component> ActionDispatcher for AfterRender<C> {
    /// Devuelve el identificador de tipo ([`UniqueId`]) del componente `C`.
    fn referer_type_id(&self) -> Option<UniqueId> {
        self.referer_type_id
    }

    /// Devuelve el identificador del componente.
    fn referer_id(&self) -> Option<String> {
        self.referer_id.get()
    }

    /// Devuelve el peso para definir el orden de ejecución.
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
            referer_id: AttrId::default(),
            weight: 0,
        }
    }

    /// Afina el registro para ejecutar la acción [`FnActionWithComponent`] sólo para el componente
    /// `C` con identificador `id`.
    pub fn filter_by_referer_id(mut self, id: impl AsRef<str>) -> Self {
        self.referer_id.alter_value(id);
        self
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    // Despacha las acciones.
    #[inline]
    pub(crate) fn dispatch(component: &mut C, cx: &mut Context) {
        // Primero despacha las acciones para el tipo de componente.
        dispatch_actions(
            &ActionKey::new(
                UniqueId::of::<Self>(),
                None,
                Some(UniqueId::of::<C>()),
                None,
            ),
            |action: &Self| (action.f)(component, cx),
        );
        // Y luego despacha las acciones para el tipo de componente con un identificador dado.
        if let Some(id) = component.id() {
            dispatch_actions(
                &ActionKey::new(
                    UniqueId::of::<Self>(),
                    None,
                    Some(UniqueId::of::<C>()),
                    Some(id),
                ),
                |action: &Self| (action.f)(component, cx),
            );
        }
    }
}
