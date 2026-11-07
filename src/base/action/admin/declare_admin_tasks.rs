use crate::prelude::*;

use super::FnActionDeclareAdminTasks;

/// Acción para declarar tareas (pestañas) locales en páginas del panel.
///
/// Cada [`AdminTask`] indica en `parent_path` la página a la que pertenece.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// fn declare_tasks(bag: &mut AdminTaskBag) {
///     bag.add(AdminTask {
///         path: "config/export/csv".to_owned(),
///         parent_path: "config/export".to_owned(),
///         title: Lc::n("CSV"),
///         weight: 0,
///         is_default: true,
///         permission: None,
///     });
/// }
/// // En Extension::actions():
/// // action::admin::DeclareAdminTasks::new(declare_tasks)
/// ```
pub struct DeclareAdminTasks {
    f: FnActionDeclareAdminTasks,
    weight: Weight,
}

impl ActionDispatcher for DeclareAdminTasks {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareAdminTasks {
    /// Permite [registrar](Extension::actions) una nueva acción [`FnActionDeclareAdminTasks`].
    pub fn new(f: FnActionDeclareAdminTasks) -> Self {
        DeclareAdminTasks { f, weight: 0 }
    }

    /// Opcional. Acciones con pesos más bajos se aplican antes. Se pueden usar valores negativos.
    pub fn with_weight(mut self, value: Weight) -> Self {
        self.weight = value;
        self
    }

    /// Despacha las acciones sobre la bolsa de tareas.
    #[inline]
    pub(crate) fn dispatch(bag: &mut AdminTaskBag) {
        dispatch_actions(|action: &Self| (action.f)(bag));
    }
}
