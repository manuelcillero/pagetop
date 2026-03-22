use crate::prelude::*;

use crate::base::action::FnActionAlterMarkup;
use crate::html::html;

/// Ejecuta [`FnActionAlterMarkup`] para modificar el renderizado de un componente.
pub struct AlterMarkup<C: Component> {
    f: FnActionAlterMarkup<C>,
    referer_type_id: Option<UniqueId>,
    referer_id: AttrId,
    weight: Weight,
}

/// Filtro para despachar [`FnActionAlterMarkup`] sobre el renderizado de un componente `C`.
impl<C: Component> ActionDispatcher for AlterMarkup<C> {
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

impl<C: Component> AlterMarkup<C> {
    /// Permite [registrar](Extension::actions) una nueva acción [`FnActionAlterMarkup`].
    pub fn new(f: FnActionAlterMarkup<C>) -> Self {
        AlterMarkup {
            f,
            referer_type_id: Some(UniqueId::of::<C>()),
            referer_id: AttrId::default(),
            weight: 0,
        }
    }

    /// Afina el registro para ejecutar la acción [`FnActionAlterMarkup`] sólo para el componente
    /// `C` con identificador `id`.
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
    pub(crate) fn dispatch(component: &mut C, cx: &mut Context, markup: Markup) -> Markup {
        let mut output = markup;

        // Primero despacha las acciones para el tipo de componente.
        dispatch_actions(
            &ActionKey::new(
                UniqueId::of::<Self>(),
                None,
                Some(UniqueId::of::<C>()),
                None,
            ),
            |action: &Self| {
                let taken = std::mem::replace(&mut output, html! {});
                output = (action.f)(component, cx, taken);
            },
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
                |action: &Self| {
                    let taken = std::mem::replace(&mut output, html! {});
                    output = (action.f)(component, cx, taken);
                },
            );
        }

        output
    }
}
