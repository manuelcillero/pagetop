use crate::prelude::*;

use super::FnActionDeclareAdminActions;

/// Acción para declarar acciones locales (botones de acción) en páginas del panel.
///
/// Cada [`AdminAction`] indica en `for_path` la página en la que aparece el botón.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// fn declare_actions(bag: &mut AdminActionBag) {
///     bag.add(AdminAction {
///         url: "config/export/new".to_owned(),
///         for_path: "config/export".to_owned(),
///         title: Lc::n("Add export"),
///         weight: 0,
///         permission: None,
///     });
/// }
/// // En Extension::actions():
/// // action::admin::DeclareAdminActions::new(declare_actions)
/// ```
pub struct DeclareAdminActions {
    f: FnActionDeclareAdminActions,
    weight: Weight,
}

impl ActionDispatcher for DeclareAdminActions {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareAdminActions {
    /// Permite [registrar](Extension::actions) una nueva acción [`FnActionDeclareAdminActions`].
    pub fn new(f: FnActionDeclareAdminActions) -> Self {
        DeclareAdminActions { f, weight: 0 }
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    /// Despacha las acciones sobre la bolsa de acciones locales.
    #[inline]
    pub(crate) fn dispatch(bag: &mut AdminActionBag) {
        dispatch_actions(|action: &Self| (action.f)(bag));
    }
}
