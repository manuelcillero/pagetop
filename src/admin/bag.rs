use super::{AdminAction, AdminPage, AdminSection, AdminTask};

/// Bolsa de secciones para [`DeclareAdminSections`].
///
/// [`DeclareAdminSections`]: crate::base::action::admin::DeclareAdminSections
pub struct AdminSectionBag {
    pub(super) sections: Vec<AdminSection>,
}

impl AdminSectionBag {
    /// Añade una sección al registro.
    pub fn add(&mut self, section: AdminSection) {
        self.sections.push(section);
    }
}

/// Bolsa de páginas para [`DeclareAdminPages`].
///
/// [`DeclareAdminPages`]: crate::base::action::admin::DeclareAdminPages
pub struct AdminPageBag {
    pub(super) pages: Vec<AdminPage>,
}

impl AdminPageBag {
    /// Añade una página al registro.
    pub fn add(&mut self, page: AdminPage) {
        self.pages.push(page);
    }
}

/// Bolsa de tareas para [`DeclareAdminTasks`].
///
/// [`DeclareAdminTasks`]: crate::base::action::admin::DeclareAdminTasks
pub struct AdminTaskBag {
    pub(super) tasks: Vec<AdminTask>,
}

impl AdminTaskBag {
    /// Añade una tarea local al registro.
    pub fn add(&mut self, task: AdminTask) {
        self.tasks.push(task);
    }
}

/// Bolsa de acciones locales para [`DeclareAdminActions`].
///
/// [`DeclareAdminActions`]: crate::base::action::admin::DeclareAdminActions
pub struct AdminActionBag {
    pub(super) actions: Vec<AdminAction>,
}

impl AdminActionBag {
    /// Añade una acción local al registro.
    pub fn add(&mut self, action: AdminAction) {
        self.actions.push(action);
    }
}
