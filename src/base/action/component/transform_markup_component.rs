use crate::prelude::*;

use super::FnActionTransformMarkup;

/// Ejecuta [`FnActionTransformMarkup`] para alterar el renderizado de componentes.
pub struct TransformMarkup<C: Component> {
    f: FnActionTransformMarkup<C>,
    referer: ActionReferer,
    weight: Weight,
}

// Filtro para despachar `FnActionTransformMarkup` sobre el renderizado de un componente `C`.
impl<C: Component> ActionDispatcher for TransformMarkup<C> {
    fn referer(&self) -> Option<&ActionReferer> {
        Some(&self.referer)
    }

    fn weight(&self) -> Weight {
        self.weight
    }
}

impl<C: Component> TransformMarkup<C> {
    /// Permite [registrar](Extension::actions) una nueva acción [`FnActionTransformMarkup`].
    pub fn new(f: FnActionTransformMarkup<C>) -> Self {
        TransformMarkup {
            f,
            referer: ActionReferer::of::<C>(),
            weight: 0,
        }
    }

    /// Afina el registro para ejecutar la acción [`FnActionTransformMarkup`] sólo para el
    /// componente `C` con identificador `id`.
    pub fn filter_by_referer_id(mut self, id: impl AsRef<str>) -> Self {
        self.referer = self.referer.with_id(id);
        self
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    /// Despacha las acciones encadenando el [`Markup`] entre cada una.
    #[inline]
    pub(crate) fn dispatch(component: &mut C, cx: &Context, markup: Markup) -> Markup {
        let mut output = markup;
        dispatch_referer(
            component,
            |c| c.id(),
            |action: &Self, c| {
                let taken = std::mem::replace(&mut output, html! {});
                output = (action.f)(c, cx, taken);
            },
        );
        output
    }
}
