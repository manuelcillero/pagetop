use crate::prelude::*;

use super::FnActionDeclareAdminPages;

/// Acción para declarar páginas del panel de administración.
///
/// Declarar una página la incluye en la navegación del panel; su *handler* lo monta la propia
/// extensión en [`Extension::configure_router()`] con [`Routes::admin_route()`], que exige el
/// permiso de la página. Ver el ejemplo del módulo [`admin`](crate::admin).
pub struct DeclareAdminPages {
    f: FnActionDeclareAdminPages,
    weight: Weight,
}

impl ActionDispatcher for DeclareAdminPages {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareAdminPages {
    /// Permite [registrar](Extension::actions) una nueva acción [`FnActionDeclareAdminPages`].
    pub fn new(f: FnActionDeclareAdminPages) -> Self {
        DeclareAdminPages { f, weight: 0 }
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    /// Despacha las acciones sobre la bolsa de páginas.
    #[inline]
    pub(crate) fn dispatch(bag: &mut AdminPageBag) {
        dispatch_actions(|action: &Self| (action.f)(bag));
    }
}
