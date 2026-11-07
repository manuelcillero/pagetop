use crate::Getters;
use crate::auth::{PermissionRef, PermissionRegistry, has_permission};
use crate::base::action::admin::{
    DeclareAdminActions, DeclareAdminPages, DeclareAdminSections, DeclareAdminTasks,
};
use crate::core::component::{Context, Contextual};
use crate::locale::Lc;
use crate::{trace, util};

use super::bag::{AdminActionBag, AdminPageBag, AdminSectionBag, AdminTaskBag};

use std::collections::BTreeMap;
use std::sync::OnceLock;

// **< Tipos del registro >*************************************************************************

/// Sección del panel de administración (agrupa páginas en la navegación y en el *dashboard*).
///
/// Una sección no tiene permiso propio: se muestra a quien pueda acceder a alguna de sus páginas, y
/// se oculta si no le queda ninguna.
#[derive(Clone)]
pub struct AdminSection {
    /// Identificador único de la sección (p. ej. `"config"`).
    pub key: String,
    /// Ruta de la sección, relativa a la raíz del panel (p. ej. `"config"`).
    pub path: String,
    /// Título visible en la navegación.
    pub title: Lc,
    /// Peso para ordenar las secciones (menor = antes).
    pub weight: i32,
}

/// Página del panel de administración.
#[derive(Clone)]
pub struct AdminPage {
    /// Ruta de la página, relativa a la raíz del panel (p. ej. `"config/site"`).
    pub path: String,
    /// Clave de la sección a la que pertenece.
    pub section: String,
    /// Título visible en la navegación y en el encabezado de la página.
    pub title: Lc,
    /// Descripción breve para el *dashboard* y las páginas de sección, o [`Lc::none()`] si no
    /// tiene.
    pub description: Lc,
    /// Peso dentro de la sección (menor = antes).
    pub weight: i32,
    /// Permiso requerido para acceder a la página: decide si se muestra en la navegación, y si se
    /// muestra su sección, y lo exigen su ruta y las que cuelgan de ella
    /// ([`Routes::admin_route()`](crate::web::Routes::admin_route)).
    pub permission: PermissionRef,
}

impl AdminPage {
    /// Devuelve `true` si el usuario actual puede acceder a esta página.
    ///
    /// Sirve para decidir qué se muestra; las rutas de la página ya exigen su permiso (ver el
    /// [módulo](crate::admin)).
    pub fn is_accessible(&self, cx: &Context) -> bool {
        cx.request()
            .is_some_and(|request| has_permission(request, self.permission))
    }
}

/// Tarea (pestaña) local dentro de una página de administración.
#[derive(Clone)]
pub struct AdminTask {
    /// Ruta de la tarea, relativa a la raíz del panel.
    pub path: String,
    /// Ruta de la página padre, relativa a la raíz del panel.
    pub parent_path: String,
    /// Etiqueta de la pestaña.
    pub title: Lc,
    /// Peso (menor = primera pestaña).
    pub weight: i32,
    /// Si es la tarea por defecto (pestaña activa al entrar a la página padre).
    pub is_default: bool,
    /// Permiso requerido para mostrar la pestaña (`None` = se muestra a quien vea la página padre).
    /// Su ruta sólo exige el permiso de la página de la que cuelga; si la pestaña pide otro, lo
    /// comprueba su *handler*.
    pub permission: Option<PermissionRef>,
}

/// Acción local (botón de acción) en una página de administración.
#[derive(Clone)]
pub struct AdminAction {
    /// Ruta de destino de la acción, relativa a la raíz del panel.
    pub url: String,
    /// Página en la que aparece el botón, relativa a la raíz del panel.
    pub for_path: String,
    /// Etiqueta del botón.
    pub title: Lc,
    /// Peso (menor = primero).
    pub weight: i32,
    /// Permiso requerido para mostrar el botón (`None` = se muestra a quien vea la página).
    /// Su ruta de destino sólo exige el permiso de la página de la que cuelga; si el botón pide
    /// otro, lo comprueba su *handler*.
    pub permission: Option<PermissionRef>,
}

// **< AdminRegistry >******************************************************************************

/// Registro del panel de administración, construido una sola vez al arrancar.
///
/// Reúne lo que declaran las extensiones con las acciones de
/// [`action::admin`](crate::base::action::admin), con todas las rutas relativas a la raíz del
/// panel.
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

static REGISTRY: OnceLock<AdminRegistry> = OnceLock::new();

static EMPTY_REGISTRY: AdminRegistry = AdminRegistry {
    sections: BTreeMap::new(),
    pages: BTreeMap::new(),
    tasks: BTreeMap::new(),
    actions: BTreeMap::new(),
};

impl AdminRegistry {
    /// Devuelve el registro global del panel de administración.
    ///
    /// Fuera de una aplicación, o antes de que ésta registre las acciones de sus extensiones,
    /// devuelve un registro vacío.
    pub fn global() -> &'static AdminRegistry {
        REGISTRY.get().unwrap_or(&EMPTY_REGISTRY)
    }

    // Página declarada que corresponde a la ruta, o la más cercana que la contiene (`"user/users"`
    // para `"user/users/5/edit"`).
    pub(crate) fn nearest_page(&self, path: &str) -> Option<&AdminPage> {
        let mut candidate = path;
        loop {
            if let Some(page) = self.pages.get(candidate) {
                return Some(page);
            }
            candidate = candidate.rsplit_once('/')?.0;
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

// **< Construcción del registro >******************************************************************

// Construye el registro despachando las acciones de declaración, después de las secciones
// habituales para que una extensión pueda sustituirlas con la misma clave. Se llama al arrancar,
// justo después de registrar las acciones de las extensiones; si ya está construido (p. ej. por
// otra llamada a `Application::prepare()` en el mismo proceso, como hacen las pruebas), no hace
// nada.
pub(crate) fn build_admin_registry() {
    REGISTRY.get_or_init(|| {
        let mut registry = AdminRegistry {
            sections: BTreeMap::new(),
            pages: BTreeMap::new(),
            tasks: BTreeMap::new(),
            actions: BTreeMap::new(),
        };

        let mut section_bag = AdminSectionBag {
            sections: default_sections(),
        };
        DeclareAdminSections::dispatch(&mut section_bag);
        for mut s in section_bag.sections {
            s.path = relative(s.path);
            registry.sections.insert(s.key.clone(), s);
        }

        let mut warnings = Vec::new();

        let mut page_bag = AdminPageBag { pages: Vec::new() };
        DeclareAdminPages::dispatch(&mut page_bag);
        for mut p in page_bag.pages {
            p.path = relative(p.path);
            if registry.pages.contains_key(&p.path) {
                warnings.push(format!(
                    "Admin page \"{}\" is declared more than once; the last declaration wins",
                    p.path
                ));
            }
            registry.pages.insert(p.path.clone(), p);
        }

        let mut task_bag = AdminTaskBag { tasks: Vec::new() };
        DeclareAdminTasks::dispatch(&mut task_bag);
        for mut t in task_bag.tasks {
            t.path = relative(t.path);
            t.parent_path = relative(t.parent_path);
            registry
                .tasks
                .entry(t.parent_path.clone())
                .or_default()
                .push(t);
        }

        let mut action_bag = AdminActionBag {
            actions: Vec::new(),
        };
        DeclareAdminActions::dispatch(&mut action_bag);
        for mut a in action_bag.actions {
            a.url = relative(a.url);
            a.for_path = relative(a.for_path);
            registry
                .actions
                .entry(a.for_path.clone())
                .or_default()
                .push(a);
        }

        warnings.extend(check_references(&registry));
        warnings.extend(check_permissions(&registry, PermissionRegistry::global()));
        for warning in warnings {
            trace::warn!("{warning}");
        }
        registry
    });
}

// Avisos de las referencias por texto que no llevan a nada: la pieza no se mostraría y nada lo
// indicaría. Una sección declarada dos veces no se avisa, porque sustituir una es lo previsto; sí
// dos secciones con la misma ruta, porque sólo la primera por clave tendría página.
fn check_references(registry: &AdminRegistry) -> Vec<String> {
    let mut warnings = Vec::new();
    let mut section_paths: BTreeMap<&str, &str> = BTreeMap::new();
    for section in registry.sections.values() {
        let first = *section_paths.entry(&section.path).or_insert(&section.key);
        if first != section.key {
            warnings.push(format!(
                "Admin sections \"{first}\" and \"{}\" share the path \"{}\", only the first is \
                 shown on it",
                section.key, section.path
            ));
        }
    }
    for page in registry.pages.values() {
        if !registry.sections.contains_key(&page.section) {
            warnings.push(format!(
                "Admin page \"{}\" belongs to the unknown section \"{}\", not shown",
                page.path, page.section
            ));
        }
    }
    for (parent, tasks) in &registry.tasks {
        if !registry.pages.contains_key(parent) {
            for task in tasks {
                warnings.push(format!(
                    "Admin task \"{}\" belongs to the undeclared page \"{parent}\", not shown",
                    task.path
                ));
            }
        }
    }
    let is_task = |path: &str| registry.tasks.values().flatten().any(|t| t.path == path);
    for (for_path, actions) in &registry.actions {
        if !registry.pages.contains_key(for_path) && !is_task(for_path) {
            for action in actions {
                warnings.push(format!(
                    "Admin action \"{}\" is for the undeclared page \"{for_path}\", not shown",
                    action.url
                ));
            }
        }
    }
    warnings
}

// Avisos de los permisos que exigen las páginas, pestañas y botones del panel sin estar en el
// catálogo: ninguna extensión de autenticación podría ofrecerlos para concederlos, y la pieza sólo
// la vería quien tenga todos los permisos. Uno por permiso, con todo lo que lo exige.
fn check_permissions(registry: &AdminRegistry, catalog: &PermissionRegistry) -> Vec<String> {
    let mut missing: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut check = |perm: PermissionRef, piece: String| {
        if !catalog.has(perm) {
            missing
                .entry(perm.key().into_owned())
                .or_default()
                .push(piece);
        }
    };
    for page in registry.pages.values() {
        check(page.permission, util::join!("page \"", &page.path, "\""));
    }
    for task in registry.tasks.values().flatten() {
        if let Some(perm) = task.permission {
            check(perm, util::join!("task \"", &task.path, "\""));
        }
    }
    for action in registry.actions.values().flatten() {
        if let Some(perm) = action.permission {
            check(perm, util::join!("action \"", &action.url, "\""));
        }
    }
    missing
        .into_iter()
        .map(|(key, pieces)| undeclared_permission(&key, &pieces))
        .collect()
}

// Aviso de un permiso que no está en el catálogo, compartido con las rutas de `Routes`.
pub(crate) fn undeclared_permission(key: &str, required_by: &[String]) -> String {
    format!(
        "Permission \"{key}\" is not declared with DeclarePermissions, so no authentication \
         extension can offer it to be granted (required by {})",
        required_by.join(", ")
    )
}

// Ruta relativa sin `/` al principio ni al final, la forma en que se guardan y se buscan.
fn relative(path: String) -> String {
    let trimmed = path.trim_matches('/');
    if trimmed.len() == path.len() {
        path
    } else {
        trimmed.to_owned()
    }
}

// Secciones habituales del panel, declaradas antes que las de las extensiones.
fn default_sections() -> Vec<AdminSection> {
    [
        ("people", "admin_section_people", 10),
        ("structure", "admin_section_structure", 20),
        ("config", "admin_section_config", 30),
        ("reports", "admin_section_reports", 40),
        ("help", "admin_section_help", 50),
    ]
    .into_iter()
    .map(|(key, title, weight)| AdminSection {
        key: key.to_owned(),
        path: key.to_owned(),
        title: Lc::l(title),
        weight,
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CowStr;
    use crate::auth::Permission;

    struct AnyPermission;

    impl Permission for AnyPermission {
        fn key(&self) -> CowStr {
            "test:any".into()
        }
    }

    fn page(path: &str, section: &str) -> AdminPage {
        AdminPage {
            path: path.to_owned(),
            section: section.to_owned(),
            title: Lc::n("Page"),
            description: Lc::none(),
            weight: 0,
            permission: &AnyPermission,
        }
    }

    fn task(path: &str, parent_path: &str) -> AdminTask {
        AdminTask {
            path: path.to_owned(),
            parent_path: parent_path.to_owned(),
            title: Lc::n("Task"),
            weight: 0,
            is_default: false,
            permission: None,
        }
    }

    fn action(url: &str, for_path: &str) -> AdminAction {
        AdminAction {
            url: url.to_owned(),
            for_path: for_path.to_owned(),
            title: Lc::n("Action"),
            weight: 0,
            permission: None,
        }
    }

    #[test]
    fn references_to_undeclared_pieces_are_reported() {
        let mut registry = AdminRegistry {
            sections: default_sections()
                .into_iter()
                .map(|s| (s.key.clone(), s))
                .collect(),
            pages: BTreeMap::new(),
            tasks: BTreeMap::new(),
            actions: BTreeMap::new(),
        };
        let mut settings = registry.sections["config"].clone();
        settings.key = "settings".to_owned();
        registry.sections.insert(settings.key.clone(), settings);
        for p in [page("config/site", "config"), page("tools/x", "tools")] {
            registry.pages.insert(p.path.clone(), p);
        }
        registry.tasks.insert(
            "config/site".to_owned(),
            vec![task("config/site/advanced", "config/site")],
        );
        registry.tasks.insert(
            "config/sit".to_owned(),
            vec![task("config/sit/typo", "config/sit")],
        );
        for a in [
            action("config/site/new", "config/site"),
            action("config/site/advanced/new", "config/site/advanced"),
            action("config/other/new", "config/other"),
        ] {
            registry
                .actions
                .entry(a.for_path.clone())
                .or_default()
                .push(a);
        }

        let warnings = check_references(&registry);
        assert_eq!(warnings.len(), 4, "{warnings:?}");
        assert!(warnings[0].contains("\"config\" and \"settings\" share the path \"config\""));
        assert!(warnings[1].contains("\"tools/x\" belongs to the unknown section \"tools\""));
        assert!(warnings[2].contains("\"config/sit/typo\" belongs to the undeclared page"));
        assert!(warnings[3].contains("\"config/other/new\" is for the undeclared page"));
    }

    struct OtherPermission;

    impl Permission for OtherPermission {
        fn key(&self) -> CowStr {
            "test:other".into()
        }
    }

    #[test]
    fn permissions_missing_from_the_catalog_are_reported_once_each() {
        let mut registry = AdminRegistry {
            sections: BTreeMap::new(),
            pages: BTreeMap::new(),
            tasks: BTreeMap::new(),
            actions: BTreeMap::new(),
        };
        for p in [
            page("config/site", "config"),
            page("config/other", "config"),
        ] {
            registry.pages.insert(p.path.clone(), p);
        }
        let mut advanced = task("config/site/advanced", "config/site");
        advanced.permission = Some(&OtherPermission);
        registry
            .tasks
            .insert("config/site".to_owned(), vec![advanced]);

        let mut catalog = PermissionRegistry::default();
        catalog.register(&AnyPermission);
        catalog.register(&OtherPermission);
        assert!(check_permissions(&registry, &catalog).is_empty());

        let warnings = check_permissions(&registry, &PermissionRegistry::default());
        assert_eq!(warnings.len(), 2, "{warnings:?}");
        assert!(warnings[0].contains("\"test:any\" is not declared with DeclarePermissions"));
        assert!(warnings[0].contains("page \"config/other\", page \"config/site\""));
        assert!(warnings[1].contains("\"test:other\""));
        assert!(warnings[1].contains("task \"config/site/advanced\""));
    }
}
