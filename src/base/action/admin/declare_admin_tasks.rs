use crate::prelude::*;

use super::FnActionDeclareAdminTasks;

/// Acción para declarar tareas (pestañas) locales en páginas del panel.
///
/// Cada [`AdminTask`] indica la página a la que pertenece; la pestaña con la ruta de la propia
/// página es la activa al entrar en ella.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// fn declare_tasks(bag: &mut AdminTaskBag) {
///     bag.add(AdminTask::new("config/export", "config/export", Lc::n("All")));
///     bag.add(AdminTask::new("config/export/csv", "config/export", Lc::n("CSV")).with_weight(1));
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
