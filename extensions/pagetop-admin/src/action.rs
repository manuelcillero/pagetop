//! Tipos de acción que `pagetop-admin` expone para que otras extensiones registren
//! secciones, páginas, tareas y acciones en el panel de administración.
//!
//! El flujo de `registry::build()` despacha estas acciones en este orden:
//!
//! 1. `DeclareAdminSections` - secciones (agrupaciones del sidebar).
//! 2. `DeclareAdminPages` - páginas del panel.
//! 3. `DeclareAdminTasks` - tareas (pestañas) locales por página.
//! 4. `DeclareAdminActions` - acciones locales (botones) por página.

use pagetop::prelude::*;

use crate::registry::{AdminAction, AdminPage, AdminSection, AdminTask};

// **< Tipo de callbacks >**************************************************************************

pub type FnSectionBag = fn(&mut SectionBag);
pub type FnPageBag = fn(&mut PageBag);
pub type FnTaskBag = fn(&mut TaskBag);
pub type FnActionBag = fn(&mut ActionBag);

// **< Bolsas de declaración >**********************************************************************

/// Bolsa de secciones para [`DeclareAdminSections`].
pub struct SectionBag {
    pub(crate) sections: Vec<AdminSection>,
}

impl SectionBag {
    /// Añade una sección al registro.
    pub fn add(&mut self, section: AdminSection) {
        self.sections.push(section);
    }
}

/// Bolsa de páginas para [`DeclareAdminPages`].
pub struct PageBag {
    pub(crate) pages: Vec<AdminPage>,
}

impl PageBag {
    /// Añade una página al registro.
    pub fn add(&mut self, page: AdminPage) {
        self.pages.push(page);
    }
}

/// Bolsa de tareas para [`DeclareAdminTasks`].
pub struct TaskBag {
    pub(crate) tasks: Vec<AdminTask>,
}

impl TaskBag {
    /// Añade una tarea local al registro.
    pub fn add(&mut self, task: AdminTask) {
        self.tasks.push(task);
    }
}

/// Bolsa de acciones locales para [`DeclareAdminActions`].
pub struct ActionBag {
    pub(crate) actions: Vec<AdminAction>,
}

impl ActionBag {
    /// Añade una acción local al registro.
    pub fn add(&mut self, action: AdminAction) {
        self.actions.push(action);
    }
}

// **< DeclareAdminSections >***********************************************************************

/// Acción para declarar secciones del panel de administración.
///
/// Se despacha durante `registry::build()` (dentro de `initialize()`). El callback
/// recibe un [`SectionBag`] y puede llamar a `add()` para registrar secciones.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::locale::Lc;
/// use pagetop_admin::action::{DeclareAdminSections, SectionBag};
/// use pagetop_admin::registry::AdminSection;
///
/// fn declare_sections(bag: &mut SectionBag) {
///     bag.add(AdminSection {
///         key:        "tools".to_owned(),
///         path:       "/admin/tools".to_owned(),
///         title:      Lc::n("Tools"),
///         permission: None,
///         weight:     60,
///     });
/// }
/// // En Extension::actions():
/// // DeclareAdminSections::new(declare_sections)
/// ```
pub struct DeclareAdminSections {
    f: FnSectionBag,
    weight: Weight,
}

impl ActionDispatcher for DeclareAdminSections {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareAdminSections {
    pub fn new(f: FnSectionBag) -> Self {
        DeclareAdminSections { f, weight: 0 }
    }

    pub fn with_weight(mut self, w: Weight) -> Self {
        self.weight = w;
        self
    }

    pub(crate) fn dispatch(bag: &mut SectionBag) {
        dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), None, None),
            |action: &Self| (action.f)(bag),
        );
    }
}

// **< DeclareAdminPages >**************************************************************************

/// Acción para declarar páginas del panel de administración.
///
/// Se despacha durante `registry::build()`. El callback recibe un [`PageBag`].
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::locale::Lc;
/// use pagetop_admin::action::{DeclareAdminPages, PageBag};
/// use pagetop_admin::registry::{AdminPage, AdminPageKind};
///
/// fn declare_pages(bag: &mut PageBag) {
///     bag.add(AdminPage {
///         path:        "/admin/tools/export".to_owned(),
///         section:     "tools".to_owned(),
///         title:       Lc::n("Export"),
///         description: Some(Lc::n("Export site data.")),
///         weight:      0,
///         permission:  None,
///         kind:        AdminPageKind::View,
///     });
/// }
/// // En Extension::actions():
/// // DeclareAdminPages::new(declare_pages)
/// ```
pub struct DeclareAdminPages {
    f: FnPageBag,
    weight: Weight,
}

impl ActionDispatcher for DeclareAdminPages {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareAdminPages {
    pub fn new(f: FnPageBag) -> Self {
        DeclareAdminPages { f, weight: 0 }
    }

    pub fn with_weight(mut self, w: Weight) -> Self {
        self.weight = w;
        self
    }

    pub(crate) fn dispatch(bag: &mut PageBag) {
        dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), None, None),
            |action: &Self| (action.f)(bag),
        );
    }
}

// **< DeclareAdminTasks >**************************************************************************

/// Acción para declarar tareas (pestañas) locales en páginas del panel.
///
/// Se despacha durante `registry::build()`. El callback recibe un [`TaskBag`].
/// Cada [`AdminTask`] indica en `parent_path` la página a la que pertenece.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::locale::Lc;
/// use pagetop_admin::action::{DeclareAdminTasks, TaskBag};
/// use pagetop_admin::registry::AdminTask;
///
/// fn declare_tasks(bag: &mut TaskBag) {
///     bag.add(AdminTask {
///         path:        "/admin/tools/export/csv".to_owned(),
///         parent_path: "/admin/tools/export".to_owned(),
///         title:       Lc::n("CSV"),
///         weight:      0,
///         is_default:  true,
///         permission:  None,
///     });
/// }
/// // En Extension::actions():
/// // DeclareAdminTasks::new(declare_tasks)
/// ```
pub struct DeclareAdminTasks {
    f: FnTaskBag,
    weight: Weight,
}

impl ActionDispatcher for DeclareAdminTasks {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareAdminTasks {
    pub fn new(f: FnTaskBag) -> Self {
        DeclareAdminTasks { f, weight: 0 }
    }

    pub fn with_weight(mut self, w: Weight) -> Self {
        self.weight = w;
        self
    }

    pub(crate) fn dispatch(bag: &mut TaskBag) {
        dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), None, None),
            |action: &Self| (action.f)(bag),
        );
    }
}

// **< DeclareAdminActions >************************************************************************

/// Acción para declarar acciones locales (botones de acción) en páginas del panel.
///
/// Se despacha durante `registry::build()`. El callback recibe un [`ActionBag`].
/// Cada [`AdminAction`] indica en `for_path` la página en la que aparece el botón.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::locale::Lc;
/// use pagetop_admin::action::{DeclareAdminActions, ActionBag};
/// use pagetop_admin::registry::AdminAction;
///
/// fn declare_actions(bag: &mut ActionBag) {
///     bag.add(AdminAction {
///         url:      "/admin/tools/export/new".to_owned(),
///         for_path: "/admin/tools/export".to_owned(),
///         title:    Lc::n("Add export"),
///         weight:   0,
///     });
/// }
/// // En Extension::actions():
/// // DeclareAdminActions::new(declare_actions)
/// ```
pub struct DeclareAdminActions {
    f: FnActionBag,
    weight: Weight,
}

impl ActionDispatcher for DeclareAdminActions {
    fn weight(&self) -> Weight {
        self.weight
    }
}

impl DeclareAdminActions {
    pub fn new(f: FnActionBag) -> Self {
        DeclareAdminActions { f, weight: 0 }
    }

    pub fn with_weight(mut self, w: Weight) -> Self {
        self.weight = w;
        self
    }

    pub(crate) fn dispatch(bag: &mut ActionBag) {
        dispatch_actions(
            &ActionKey::new(UniqueId::of::<Self>(), None, None),
            |action: &Self| (action.f)(bag),
        );
    }
}
