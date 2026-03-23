use crate::prelude::*;

use super::FnActionTransformMarkup;

/// Ejecuta [`FnActionTransformMarkup`] para alterar el renderizado de componentes.
pub struct TransformMarkup<C: Component> {
    f: FnActionTransformMarkup<C>,
    referer_type_id: Option<UniqueId>,
    referer_id: AttrId,
    weight: Weight,
}

/// Filtro para despachar [`FnActionTransformMarkup`] sobre el renderizado de un componente `C`.
impl<C: Component> ActionDispatcher for TransformMarkup<C> {
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

impl<C: Component> TransformMarkup<C> {
    /// Permite [registrar](Extension::actions) una nueva acción [`FnActionTransformMarkup`].
    pub fn new(f: FnActionTransformMarkup<C>) -> Self {
        TransformMarkup {
            f,
            referer_type_id: Some(UniqueId::of::<C>()),
            referer_id: AttrId::default(),
            weight: 0,
        }
    }

    /// Afina el registro para ejecutar la acción [`FnActionTransformMarkup`] sólo para el
    /// componente `C` con identificador `id`.
    pub fn filter_by_referer_id(mut self, id: impl AsRef<str>) -> Self {
        self.referer_id.alter_id(id);
        self
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    /// Despacha las acciones encadenando el [`Markup`] entre cada una.
    #[inline]
    pub(crate) fn dispatch(component: &C, cx: &mut Context, markup: Markup) -> Markup {
        let mut output = markup;

        // Primero despacha las acciones para el tipo de componente.
        dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), Some(UniqueId::of::<C>()), None),
            |action: &Self| {
                let taken = std::mem::replace(&mut output, html! {});
                output = (action.f)(component, cx, taken);
            },
        );

        // Y luego despacha las acciones para el tipo de componente con un identificador dado.
        if let Some(id) = component.id() {
            dispatch_actions(
                &ActionKey::new(UniqueId::of::<Self>(), Some(UniqueId::of::<C>()), Some(id)),
                |action: &Self| {
                    let taken = std::mem::replace(&mut output, html! {});
                    output = (action.f)(component, cx, taken);
                },
            );
        }

        output
    }
}
