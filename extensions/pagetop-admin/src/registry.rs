//! Registro global de secciones, páginas, tareas y acciones del panel de administración.
//!
//! Se construye una única vez durante `Extension::initialize()` y permanece
//! inmutable durante la vida de la aplicación.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use pagetop::prelude::*;

use crate::action::{
    ActionBag, DeclareAdminActions, DeclareAdminPages, DeclareAdminSections, DeclareAdminTasks,
    PageBag, SectionBag, TaskBag,
};
use crate::settings::SettingsSchema;

// **< AdminPermission >*****************************************************************************

/// Permisos propios de `pagetop-admin`.
#[derive(Clone, Copy, Debug)]
pub enum AdminPermission {
    /// Acceso por defecto a una página de administración que no declara un permiso propio.
    Access,
    /// Acceso a la sección integrada "people".
    AccessPeople,
    /// Acceso a la sección integrada "structure".
    AccessStructure,
    /// Acceso a la sección integrada "config".
    AccessConfig,
    /// Acceso a la sección integrada "reports".
    AccessReports,
}

impl Permission for AdminPermission {
    fn key(&self) -> CowStr {
        match self {
            Self::Access => "admin:access".into(),
            Self::AccessPeople => "admin.access_people".into(),
            Self::AccessStructure => "admin.access_structure".into(),
            Self::AccessConfig => "admin.access_config".into(),
            Self::AccessReports => "admin.access_reports".into(),
        }
    }
}

// **< Tipos del registro >**************************************************************************

/// Sección del panel de administración (agrupación en el sidebar).
#[derive(Clone)]
pub struct AdminSection {
    /// Identificador único de la sección (p. ej. `"config"`).
    pub key: String,
    /// Ruta base de la sección (p. ej. `"/admin/config"`).
    pub path: String,
    /// Título visible en el sidebar.
    pub title: Lc,
    /// Permiso requerido para ver la sección (`None` = siempre visible).
    pub permission: Option<PermissionRef>,
    /// Peso para ordenar en el sidebar (menor = antes).
    pub weight: i32,
}

impl AdminSection {
    /// Devuelve `true` si el usuario actual puede ver esta sección.
    pub fn is_visible(&self, cx: &Context) -> bool {
        match self.permission {
            None => true,
            Some(permission) => cx
                .request()
                .is_some_and(|request| has_permission(request, permission)),
        }
    }
}

/// Página del panel de administración.
#[derive(Clone)]
pub struct AdminPage {
    /// Ruta exacta de la página (p. ej. `"/admin/config/site"`).
    pub path: String,
    /// Clave de la sección a la que pertenece.
    pub section: String,
    /// Título visible en el sidebar y encabezado de página.
    pub title: Lc,
    /// Descripción breve (para el dashboard y listas de páginas), si tiene una.
    pub description: Option<Lc>,
    /// Peso dentro de la sección (menor = antes).
    pub weight: i32,
    /// Permiso requerido para acceder (`None` = requiere [`AdminPermission::Access`]).
    pub permission: Option<PermissionRef>,
    /// Tipo de página y datos asociados.
    pub kind: AdminPageKind,
}

impl AdminPage {
    /// Permiso efectivo: el declarado, o [`AdminPermission::Access`] si no se especificó ninguno.
    pub fn permission_key(&self) -> PermissionRef {
        self.permission.unwrap_or(&AdminPermission::Access)
    }

    /// Devuelve `true` si el usuario actual puede acceder a esta página.
    pub fn is_accessible(&self, cx: &Context) -> bool {
        cx.request()
            .is_some_and(|request| has_permission(request, self.permission_key()))
    }
}

/// Variantes de comportamiento de una [`AdminPage`].
#[derive(Clone, Debug)]
pub enum AdminPageKind {
    /// Página genérica cuyo handler se registra externamente.
    View,
    /// Formulario de configuración gestionado automáticamente por `pagetop-admin`.
    ConfigForm(SettingsSchema),
}

/// Tarea (pestaña) local dentro de una página de administración.
#[derive(Clone)]
pub struct AdminTask {
    /// Ruta de la tarea.
    pub path: String,
    /// Ruta de la página padre.
    pub parent_path: String,
    /// Etiqueta de la pestaña.
    pub title: Lc,
    /// Peso (menor = primera pestaña).
    pub weight: i32,
    /// Si es la tarea por defecto (pestaña activa al entrar a la página padre).
    pub is_default: bool,
    /// Permiso requerido (`None` = mismo que la página padre).
    pub permission: Option<PermissionRef>,
}

/// Acción local (botón de acción) en una página de administración.
#[derive(Clone, Debug)]
pub struct AdminAction {
    /// Ruta de destino de la acción.
    pub url: String,
    /// Página en la que aparece el botón.
    pub for_path: String,
    /// Etiqueta del botón.
    pub title: Lc,
    /// Peso (menor = primero).
    pub weight: i32,
}

// **< AdminRegistry >*******************************************************************************

/// Registro global del panel de administración, construido una sola vez en `initialize()`.
#[derive(Getters)]
pub struct AdminRegistry {
    /// Devuelve las secciones indexadas por clave (`BTreeMap` para orden estable por clave).
    sections: BTreeMap<String, AdminSection>,
    /// Devuelve las páginas indexadas por ruta.
    pages: BTreeMap<String, AdminPage>,
    /// Devuelve las tareas indexadas por ruta de página padre.
    tasks: BTreeMap<String, Vec<AdminTask>>,
    /// Devuelve las acciones indexadas por ruta de página.
    actions: BTreeMap<String, Vec<AdminAction>>,
}

impl AdminRegistry {
    fn new() -> Self {
        AdminRegistry {
            sections: BTreeMap::new(),
            pages: BTreeMap::new(),
            tasks: BTreeMap::new(),
            actions: BTreeMap::new(),
        }
    }

    /// Devuelve las páginas de una sección ordenadas por peso.
    pub fn pages_for_section(&self, section_key: &str) -> Vec<&AdminPage> {
        let mut pages: Vec<&AdminPage> = self
            .pages
            .values()
            .filter(|p| p.section == section_key)
            .collect();
        pages.sort_by_key(|p| p.weight);
        pages
    }

    /// Devuelve las secciones ordenadas por peso.
    pub fn ordered_sections(&self) -> Vec<&AdminSection> {
        let mut sections: Vec<&AdminSection> = self.sections.values().collect();
        sections.sort_by_key(|s| s.weight);
        sections
    }

    /// Devuelve las tareas de una página ordenadas por peso.
    pub fn tasks_for(&self, path: &str) -> Vec<&AdminTask> {
        let Some(tasks) = self.tasks.get(path) else {
            return vec![];
        };
        let mut t: Vec<&AdminTask> = tasks.iter().collect();
        t.sort_by_key(|t| t.weight);
        t
    }

    /// Devuelve las acciones de una página ordenadas por peso.
    pub fn actions_for(&self, path: &str) -> Vec<&AdminAction> {
        let Some(actions) = self.actions.get(path) else {
            return vec![];
        };
        let mut a: Vec<&AdminAction> = actions.iter().collect();
        a.sort_by_key(|a| a.weight);
        a
    }
}

// **< Registro global >****************************************************************************

static REGISTRY: OnceLock<AdminRegistry> = OnceLock::new();

/// Construye el registro despachando todas las acciones de declaración.
///
/// Se llama una sola vez desde `Admin::initialize()`.
pub(crate) fn build() {
    let mut registry = AdminRegistry::new();

    // Secciones
    let mut section_bag = SectionBag {
        sections: Vec::new(),
    };
    DeclareAdminSections::dispatch(&mut section_bag);
    for s in section_bag.sections {
        registry.sections.insert(s.key.clone(), s);
    }

    // Páginas
    let mut page_bag = PageBag { pages: Vec::new() };
    DeclareAdminPages::dispatch(&mut page_bag);
    for p in page_bag.pages {
        registry.pages.insert(p.path.clone(), p);
    }

    // Tareas
    let mut task_bag = TaskBag { tasks: Vec::new() };
    DeclareAdminTasks::dispatch(&mut task_bag);
    for t in task_bag.tasks {
        registry
            .tasks
            .entry(t.parent_path.clone())
            .or_default()
            .push(t);
    }

    // Acciones locales
    let mut action_bag = ActionBag {
        actions: Vec::new(),
    };
    DeclareAdminActions::dispatch(&mut action_bag);
    for a in action_bag.actions {
        registry
            .actions
            .entry(a.for_path.clone())
            .or_default()
            .push(a);
    }

    REGISTRY.set(registry).ok();
}

/// Accede al registro global del panel de administración.
///
/// # Panics
///
/// Entra en pánico si se llama antes de que `Admin::initialize()` haya completado.
pub fn global() -> &'static AdminRegistry {
    REGISTRY.get().expect("AdminRegistry not initialized")
}

// **< Menú de administración >*********************************************************************

/// Construye el menú plano de secciones visibles para el usuario de la petición actual.
///
/// Pensado para que un tema lo use como navegación de `CoreTemplates::Admin` (p. ej. un sidebar) --
/// ver [`crate::component::AdminMenu`]. `pagetop-admin` no impone ningún marcado propio: el
/// [`Nav`] resultante se renderiza con su aspecto por defecto salvo que el tema lo intercepte en
/// [`Theme::handle_component()`](pagetop::core::theme::Theme::handle_component).
pub fn admin_menu(cx: &Context) -> Nav {
    let reg = global();
    let current_path = cx.request().map(|r| r.path()).unwrap_or("");

    let mut result = Nav::new();
    for section in reg.ordered_sections() {
        if !section.is_visible(cx) {
            continue;
        }
        let active = current_path.starts_with(section.path.as_str());
        result = result.with_item(
            nav::Item::link(section.title.clone(), section.path.clone()).with_active(active),
        );
    }
    result
}
