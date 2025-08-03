use crate::prelude::*;

use crate::base::action::FnActionWithComponent;

/// Ejecuta [`FnActionWithComponent`] antes de que un tema renderice el componente.
pub struct BeforeRender<C: Component> {
    f: FnActionWithComponent<C>,
    theme_type_id: Option<UniqueId>,
    referer_type_id: Option<UniqueId>,
}

/// Filtro para despachar [`FnActionWithComponent`] antes de que un tema renderice el componente
/// `C`.
impl<C: Component> ActionDispatcher for BeforeRender<C> {
    /// Devuelve el identificador de tipo ([`UniqueId`]) del tema.
    fn theme_type_id(&self) -> Option<UniqueId> {
        self.theme_type_id
    }

    /// Devuelve el identificador de tipo ([`UniqueId`]) del componente `C`.
    fn referer_type_id(&self) -> Option<UniqueId> {
        self.referer_type_id
    }
}

impl<C: Component> BeforeRender<C> {
    /// Permite [registrar](Extension::actions) una nueva acción [`FnActionWithComponent`] para un
    /// tema dado.
    pub fn new(theme: ThemeRef, f: FnActionWithComponent<C>) -> Self {
        BeforeRender {
            f,
            theme_type_id: Some(theme.type_id()),
            referer_type_id: Some(UniqueId::of::<C>()),
        }
    }

    // Despacha las acciones.
    #[inline]
    pub(crate) fn dispatch(component: &mut C, cx: &mut Context) {
        dispatch_actions(
            &ActionKey::new(
                UniqueId::of::<Self>(),
                Some(cx.theme().type_id()),
                Some(UniqueId::of::<C>()),
                None,
            ),
            |action: &Self| (action.f)(component, cx),
        );
    }
}
