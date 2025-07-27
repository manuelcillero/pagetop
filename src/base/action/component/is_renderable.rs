use crate::prelude::*;

/// Tipo de función para determinar si un componente se renderiza o no.
///
/// Se usa en la acción [`IsRenderable`] para controlar dinámicamente la visibilidad del componente
/// `component` según el contexto `cx`. El componente **no se renderiza** en cuanto una de las
/// funciones devuelva `false`.
pub type FnIsRenderable<C> = fn(component: &C, cx: &Context) -> bool;

/// Con la función [`FnIsRenderable`] se puede decidir si se renderiza o no un componente.
pub struct IsRenderable<C: ComponentTrait> {
    f: FnIsRenderable<C>,
    referer_type_id: Option<UniqueId>,
    referer_id: OptionId,
    weight: Weight,
}

/// Filtro para despachar [`FnIsRenderable`] para decidir si se renderiza o no un componente `C`.
impl<C: ComponentTrait> ActionDispatcher for IsRenderable<C> {
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

impl<C: ComponentTrait> IsRenderable<C> {
    /// Permite [registrar](ExtensionTrait::actions) una nueva acción [`FnIsRenderable`].
    pub fn new(f: FnIsRenderable<C>) -> Self {
        IsRenderable {
            f,
            referer_type_id: Some(UniqueId::of::<C>()),
            referer_id: OptionId::default(),
            weight: 0,
        }
    }

    /// Afina el registro para ejecutar la acción [`FnIsRenderable`] sólo para el componente `C`
    /// con identificador `id`.
    pub fn filter_by_referer_id(mut self, id: impl AsRef<str>) -> Self {
        self.referer_id.alter_value(id);
        self
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    // Despacha las acciones. Se detiene en cuanto una [`FnIsRenderable`] devuelve `false`.
    #[inline]
    pub(crate) fn dispatch(component: &C, cx: &mut Context) -> bool {
        let mut renderable = true;
        dispatch_actions(
            &ActionKey::new(
                UniqueId::of::<Self>(),
                None,
                Some(UniqueId::of::<C>()),
                None,
            ),
            |action: &Self| {
                if renderable && !(action.f)(component, cx) {
                    renderable = false;
                }
            },
        );
        if renderable {
            if let Some(id) = component.id() {
                dispatch_actions(
                    &ActionKey::new(
                        UniqueId::of::<Self>(),
                        None,
                        Some(UniqueId::of::<C>()),
                        Some(id),
                    ),
                    |action: &Self| {
                        if renderable && !(action.f)(component, cx) {
                            renderable = false;
                        }
                    },
                );
            }
        }
        renderable
    }
}
