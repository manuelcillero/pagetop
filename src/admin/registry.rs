use crate::auth::{PermissionRef, PermissionRegistry, has_permission};
use crate::base::action::admin::{
    DeclareAdminActions, DeclareAdminPages, DeclareAdminSections, DeclareAdminTasks,
};
use crate::core::component::{Context, Contextual};
use crate::locale::Lc;
use crate::{Getters, Weight, builder_impl, trace, util};

use super::bag::{AdminActionBag, AdminPageBag, AdminSectionBag, AdminTaskBag};

use std::collections::BTreeMap;
use std::ops::Range;
use std::sync::OnceLock;

// **< AdminSection >*******************************************************************************

/// Sección del panel de administración (agrupa páginas en la navegación y en el *dashboard*).
///
/// Una sección no tiene permiso propio: se muestra a quien pueda acceder a alguna de sus páginas, y
/// se oculta si no le queda ninguna. Una sección declarada con la misma clave que otra anterior la
/// sustituye.
///
/// ```rust
/// # use pagetop::prelude::*;
/// let section = AdminSection::new("tools", Lc::n("Tools")).with_weight(35);
/// assert_eq!(section.path(), "tools");
/// ```
#[derive(Clone, Getters)]
pub struct AdminSection {
    /// Devuelve el identificador único de la sección (p. ej. `"config"`).
    key: String,
    /// Devuelve la ruta de la sección, relativa a la raíz del panel (p. ej. `"config"`).
    path: String,
    /// Devuelve el título visible en la navegación.
    title: Lc,
    /// Devuelve el peso para ordenar las secciones (menor = antes).
    #[getters(copy)]
    weight: Weight,
}

#[builder_impl]
impl AdminSection {
    /// Crea una sección con su clave y su título. Su ruta es la propia clave y su peso, 0.
    pub fn new(key: impl Into<String>, title: Lc) -> Self {
        let key = key.into();
        AdminSection {
            path: relative(key.clone()),
            key,
            title,
            weight: 0,
        }
    }

    // **< AdminSection BUILDER >*******************************************************************

    /// Establece la ruta de la sección, relativa a la raíz del panel, si no es la propia clave.
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = relative(path.into());
        self
    }

    /// Establece el título de la sección.
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }

    /// Establece el peso de la sección.
    pub fn with_weight(mut self, weight: Weight) -> Self {
        self.weight = weight;
        self
    }
}

// **< AdminPage >**********************************************************************************

/// Página del panel de administración.
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// # struct ToolsPermission;
/// # impl Permission for ToolsPermission {
/// #     fn key(&self) -> CowStr { "tools:export".into() }
/// # }
/// let page = AdminPage::new("config/export", "config", Lc::n("Export"), &ToolsPermission)
///     .with_description(Lc::n("Export site data."));
/// ```
#[derive(Clone, Getters)]
pub struct AdminPage {
    /// Devuelve la ruta de la página, relativa a la raíz del panel (p. ej. `"config/site"`).
    path: String,
    /// Devuelve la clave de la sección a la que pertenece.
    section: String,
    /// Devuelve el título visible en la navegación y en el encabezado de la página.
    title: Lc,
    /// Devuelve la descripción breve para el *dashboard* y las páginas de sección, o
    /// [`Lc::none()`] si no tiene.
    description: Lc,
    /// Devuelve el peso dentro de la sección (menor = antes).
    #[getters(copy)]
    weight: Weight,
    /// Devuelve el permiso requerido para acceder a la página: decide si se muestra en la
    /// navegación, y si se muestra su sección, y lo exigen su ruta y las que cuelgan de ella
    /// ([`Routes::admin_route()`](crate::web::Routes::admin_route)).
    #[getters(copy)]
    permission: PermissionRef,
}

#[builder_impl]
impl AdminPage {
    /// Crea una página con su ruta relativa a la raíz del panel, la clave de su sección, su título
    /// y el permiso para acceder a ella. No tiene descripción y su peso es 0.
    ///
    /// La ruta no puede quedar vacía, porque la raíz del panel es el *dashboard*: una página sin
    /// ruta no se registra y se avisa al arrancar.
    pub fn new(
        path: impl Into<String>,
        section: impl Into<String>,
        title: Lc,
        permission: PermissionRef,
    ) -> Self {
        AdminPage {
            path: relative(path.into()),
            section: section.into(),
            title,
            description: Lc::none(),
            weight: 0,
            permission,
        }
    }

    // **< AdminPage BUILDER >**********************************************************************

    /// Establece el título de la página.
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }

    /// Establece la descripción breve de la página.
    pub fn with_description(mut self, description: Lc) -> Self {
        self.description = description;
        self
    }

    /// Establece el peso de la página dentro de su sección.
    pub fn with_weight(mut self, weight: Weight) -> Self {
        self.weight = weight;
        self
    }

    // **< AdminPage GETTERS >**********************************************************************

    /// Devuelve `true` si el usuario actual puede acceder a esta página.
    ///
    /// Sirve para decidir qué se muestra; las rutas de la página ya exigen su permiso (ver el
    /// [módulo](crate::admin)).
    pub fn is_accessible(&self, cx: &Context) -> bool {
        cx.request()
            .is_some_and(|request| has_permission(request, self.permission))
    }
}

// **< AdminTask >**********************************************************************************

/// Tarea (pestaña) local dentro de una página de administración.
///
/// Las pestañas de una página se muestran en ella y en la ruta de cada una, con la de la ruta
/// actual activa. La pestaña cuya ruta es la de la propia página es la activa al entrar en ella.
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let all = AdminTask::new("config/export", "config/export", Lc::n("All"));
/// let csv = AdminTask::new("config/export/csv", "config/export", Lc::n("CSV")).with_weight(1);
/// ```
#[derive(Clone, Getters)]
pub struct AdminTask {
    /// Devuelve la ruta de la tarea, relativa a la raíz del panel.
    path: String,
    /// Devuelve la ruta de la página padre, relativa a la raíz del panel.
    parent_path: String,
    /// Devuelve la etiqueta de la pestaña.
    title: Lc,
    /// Devuelve el peso de la pestaña (menor = antes).
    #[getters(copy)]
    weight: Weight,
    /// Devuelve el permiso requerido para mostrar la pestaña (`None` = se muestra a quien vea la
    /// página padre). Su ruta sólo exige el permiso de la página de la que cuelga; si la pestaña
    /// pide otro, lo comprueba su *handler*.
    #[getters(copy)]
    permission: Option<PermissionRef>,
}

#[builder_impl]
impl AdminTask {
    /// Crea una pestaña con su ruta y la de su página padre, ambas relativas a la raíz del panel,
    /// y su etiqueta. Su peso es 0 y se muestra a quien vea la página padre.
    pub fn new(path: impl Into<String>, parent_path: impl Into<String>, title: Lc) -> Self {
        AdminTask {
            path: relative(path.into()),
            parent_path: relative(parent_path.into()),
            title,
            weight: 0,
            permission: None,
        }
    }

    // **< AdminTask BUILDER >**********************************************************************

    /// Establece la etiqueta de la pestaña.
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }

    /// Establece el peso de la pestaña.
    pub fn with_weight(mut self, weight: Weight) -> Self {
        self.weight = weight;
        self
    }

    /// Establece el permiso requerido para mostrar la pestaña.
    pub fn with_permission(mut self, permission: PermissionRef) -> Self {
        self.permission = Some(permission);
        self
    }
}

// **< AdminAction >********************************************************************************

/// Acción local (botón de acción) en una página de administración.
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let add = AdminAction::new("config/export/new", "config/export", Lc::n("Add export"));
/// ```
#[derive(Clone, Getters)]
pub struct AdminAction {
    /// Devuelve la ruta de destino de la acción, relativa a la raíz del panel.
    url: String,
    /// Devuelve la ruta de la página en la que aparece el botón, relativa a la raíz del panel.
    for_path: String,
    /// Devuelve la etiqueta del botón.
    title: Lc,
    /// Devuelve el peso del botón (menor = antes).
    #[getters(copy)]
    weight: Weight,
    /// Devuelve el permiso requerido para mostrar el botón (`None` = se muestra a quien vea la
    /// página). Su ruta de destino sólo exige el permiso de la página de la que cuelga; si el botón
    /// pide otro, lo comprueba su *handler*.
    #[getters(copy)]
    permission: Option<PermissionRef>,
}

#[builder_impl]
impl AdminAction {
    /// Crea un botón con su ruta de destino y la de la página en la que aparece, ambas relativas a
    /// la raíz del panel, y su etiqueta. Su peso es 0 y se muestra a quien vea la página.
    pub fn new(url: impl Into<String>, for_path: impl Into<String>, title: Lc) -> Self {
        AdminAction {
            url: relative(url.into()),
            for_path: relative(for_path.into()),
            title,
            weight: 0,
            permission: None,
        }
    }

    // **< AdminAction BUILDER >********************************************************************

    /// Establece la etiqueta del botón.
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }

    /// Establece el peso del botón.
    pub fn with_weight(mut self, weight: Weight) -> Self {
        self.weight = weight;
        self
    }

    /// Establece el permiso requerido para mostrar el botón.
    pub fn with_permission(mut self, permission: PermissionRef) -> Self {
        self.permission = Some(permission);
        self
    }
}

// **< AdminRegistry >******************************************************************************

/// Registro del panel de administración, construido una sola vez al arrancar.
///
/// Reúne lo que declaran las extensiones con las acciones de
/// [`action::admin`](crate::base::action::admin), con todas las rutas relativas a la raíz del
/// panel. Las listas se ordenan por peso al construirlo.
pub struct AdminRegistry {
    // Por peso.
    sections: Vec<AdminSection>,
    // Agrupadas por clave de sección y, en cada una, por peso.
    pages: Vec<AdminPage>,
    // Posición en `pages` de cada página, por su ruta.
    page_index: BTreeMap<String, usize>,
    // Páginas de cada sección en `pages`, por clave de sección.
    section_pages: BTreeMap<String, Range<usize>>,
    // Por ruta de la página padre y, en cada una, por peso.
    tasks: BTreeMap<String, Vec<AdminTask>>,
    // Por ruta de la página y, en cada una, por peso.
    actions: BTreeMap<String, Vec<AdminAction>>,
}

static REGISTRY: OnceLock<AdminRegistry> = OnceLock::new();

static EMPTY_REGISTRY: AdminRegistry = AdminRegistry {
    sections: Vec::new(),
    pages: Vec::new(),
    page_index: BTreeMap::new(),
    section_pages: BTreeMap::new(),
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

    /// Devuelve las secciones ordenadas por peso.
    pub fn sections(&self) -> &[AdminSection] {
        &self.sections
    }

    /// Devuelve la sección con la clave indicada.
    pub fn section(&self, key: &str) -> Option<&AdminSection> {
        self.sections.iter().find(|s| s.key == key)
    }

    // Sección con la ruta indicada; si varias la comparten, la primera por peso.
    pub(crate) fn section_by_path(&self, path: &str) -> Option<&AdminSection> {
        self.sections.iter().find(|s| s.path == path)
    }

    /// Devuelve todas las páginas, agrupadas por la clave de su sección y, en cada grupo, ordenadas
    /// por peso.
    pub fn pages(&self) -> &[AdminPage] {
        &self.pages
    }

    /// Devuelve la página declarada con la ruta indicada, relativa a la raíz del panel.
    pub fn page(&self, path: &str) -> Option<&AdminPage> {
        self.page_index.get(path).map(|&i| &self.pages[i])
    }

    /// Devuelve las páginas de una sección ordenadas por peso.
    pub fn pages_for_section(&self, section_key: &str) -> &[AdminPage] {
        self.section_pages
            .get(section_key)
            .map_or(&[], |range| &self.pages[range.clone()])
    }

    /// Devuelve las tareas de una página ordenadas por peso.
    pub fn tasks_for(&self, path: &str) -> &[AdminTask] {
        self.tasks.get(path).map_or(&[], Vec::as_slice)
    }

    /// Devuelve las acciones de una página ordenadas por peso.
    pub fn actions_for(&self, path: &str) -> &[AdminAction] {
        self.actions.get(path).map_or(&[], Vec::as_slice)
    }

    // Página declarada que corresponde a la ruta, o la más cercana que la contiene (`"user/users"`
    // para `"user/users/5/edit"`).
    pub(crate) fn nearest_page(&self, path: &str) -> Option<&AdminPage> {
        let mut candidate = path;
        loop {
            if let Some(page) = self.page(candidate) {
                return Some(page);
            }
            candidate = candidate.rsplit_once('/')?.0;
        }
    }

    // Pestaña declarada con la ruta indicada.
    pub(crate) fn task(&self, path: &str) -> Option<&AdminTask> {
        self.tasks.values().flatten().find(|t| t.path == path)
    }

    // Construye el registro con lo declarado, en orden de declaración, y devuelve también los
    // avisos de las páginas que no se registran tal cual. Una sección con la clave de otra anterior
    // la sustituye, y una página con la ruta de otra anterior, también. Una página sin ruta se
    // descarta: la raíz del panel es el *dashboard*, y montar ahí su ruta impediría arrancar.
    fn new(
        sections: Vec<AdminSection>,
        pages: Vec<AdminPage>,
        tasks: Vec<AdminTask>,
        actions: Vec<AdminAction>,
    ) -> (Self, Vec<String>) {
        let mut warnings = Vec::new();

        let mut by_key = BTreeMap::new();
        for s in sections {
            by_key.insert(s.key.clone(), s);
        }
        let mut sections: Vec<_> = by_key.into_values().collect();
        sections.sort_by_key(|s| s.weight);

        let mut by_path = BTreeMap::new();
        for p in pages {
            if p.path.is_empty() {
                warnings.push(format!(
                    "An admin page of the section \"{}\" has an empty path, ignored: the root of \
                     the panel is the dashboard",
                    p.section
                ));
                continue;
            }
            if let Some(previous) = by_path.insert(p.path.clone(), p) {
                warnings.push(format!(
                    "Admin page \"{}\" is declared more than once; the last declaration wins",
                    previous.path
                ));
            }
        }
        let mut pages: Vec<_> = by_path.into_values().collect();
        pages.sort_by(|a, b| a.section.cmp(&b.section).then(a.weight.cmp(&b.weight)));
        let mut page_index = BTreeMap::new();
        let mut section_pages: BTreeMap<String, Range<usize>> = BTreeMap::new();
        for (i, p) in pages.iter().enumerate() {
            page_index.insert(p.path.clone(), i);
            section_pages
                .entry(p.section.clone())
                .and_modify(|range| range.end = i + 1)
                .or_insert(i..i + 1);
        }

        let registry = AdminRegistry {
            sections,
            pages,
            page_index,
            section_pages,
            tasks: grouped(tasks, |t| &t.parent_path, |t| t.weight),
            actions: grouped(actions, |a| &a.for_path, |a| a.weight),
        };
        (registry, warnings)
    }
}

// Agrupa por la clave y ordena cada grupo por peso, conservando el orden de declaración entre los
// del mismo peso.
fn grouped<T>(
    items: Vec<T>,
    key: impl Fn(&T) -> &String,
    weight: impl Fn(&T) -> Weight,
) -> BTreeMap<String, Vec<T>> {
    let mut groups: BTreeMap<String, Vec<T>> = BTreeMap::new();
    for item in items {
        groups.entry(key(&item).clone()).or_default().push(item);
    }
    for group in groups.values_mut() {
        group.sort_by_key(&weight);
    }
    groups
}

// **< Construcción del registro >******************************************************************

// Construye el registro despachando las acciones de declaración, después de las secciones
// habituales para que una extensión pueda sustituirlas con la misma clave. Se llama al arrancar,
// justo después de registrar las acciones de las extensiones; si ya está construido (p. ej. por
// otra llamada a `Application::prepare()` en el mismo proceso, como hacen las pruebas), no hace
// nada.
pub(crate) fn build_admin_registry() {
    REGISTRY.get_or_init(|| {
        let mut section_bag = AdminSectionBag {
            sections: default_sections(),
        };
        DeclareAdminSections::dispatch(&mut section_bag);
        let mut page_bag = AdminPageBag { pages: Vec::new() };
        DeclareAdminPages::dispatch(&mut page_bag);
        let mut task_bag = AdminTaskBag { tasks: Vec::new() };
        DeclareAdminTasks::dispatch(&mut task_bag);
        let mut action_bag = AdminActionBag {
            actions: Vec::new(),
        };
        DeclareAdminActions::dispatch(&mut action_bag);

        let (registry, mut warnings) = AdminRegistry::new(
            section_bag.sections,
            page_bag.pages,
            task_bag.tasks,
            action_bag.actions,
        );
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
// dos secciones con la misma ruta, porque sólo la primera por peso tendría página.
fn check_references(registry: &AdminRegistry) -> Vec<String> {
    let mut warnings = Vec::new();
    let mut section_paths: BTreeMap<&str, &str> = BTreeMap::new();
    for section in &registry.sections {
        let first = *section_paths.entry(&section.path).or_insert(&section.key);
        if first != section.key {
            warnings.push(format!(
                "Admin sections \"{first}\" and \"{}\" share the path \"{}\", only the first is \
                 shown on it",
                section.key, section.path
            ));
        }
    }
    for page in &registry.pages {
        if registry.section(&page.section).is_none() {
            warnings.push(format!(
                "Admin page \"{}\" belongs to the unknown section \"{}\", not shown",
                page.path, page.section
            ));
        }
    }
    for (parent, tasks) in &registry.tasks {
        if registry.page(parent).is_none() {
            for task in tasks {
                warnings.push(format!(
                    "Admin task \"{}\" belongs to the undeclared page \"{parent}\", not shown",
                    task.path
                ));
            }
        }
    }
    for (for_path, actions) in &registry.actions {
        if registry.page(for_path).is_none() && registry.task(for_path).is_none() {
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
    let mut pages: Vec<_> = registry.pages.iter().collect();
    pages.sort_by_key(|p| &p.path);
    for page in pages {
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
    .map(|(key, title, weight)| AdminSection::new(key, Lc::l(title)).with_weight(weight))
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

    struct OtherPermission;

    impl Permission for OtherPermission {
        fn key(&self) -> CowStr {
            "test:other".into()
        }
    }

    fn page(path: &str, section: &str) -> AdminPage {
        AdminPage::new(path, section, Lc::n("Page"), &AnyPermission)
    }

    fn task(path: &str, parent_path: &str) -> AdminTask {
        AdminTask::new(path, parent_path, Lc::n("Task"))
    }

    fn action(url: &str, for_path: &str) -> AdminAction {
        AdminAction::new(url, for_path, Lc::n("Action"))
    }

    #[test]
    fn duplicated_pages_and_pages_without_path_are_reported() {
        let (registry, warnings) = AdminRegistry::new(
            default_sections(),
            vec![
                page("config/site", "config"),
                page("config/site", "config").with_weight(5),
                page("/", "config"),
            ],
            vec![],
            vec![],
        );
        assert_eq!(warnings.len(), 2, "{warnings:?}");
        assert!(warnings[0].contains("\"config/site\" is declared more than once"));
        assert!(warnings[1].contains("\"config\" has an empty path, ignored"));
        assert_eq!(registry.page("config/site").unwrap().weight(), 5);
        assert!(registry.page("").is_none());
    }

    #[test]
    fn lists_are_sorted_by_weight_once() {
        let (registry, _) = AdminRegistry::new(
            default_sections(),
            vec![
                page("config/b", "config").with_weight(2),
                page("people/a", "people"),
                page("config/a", "config").with_weight(1),
                page("config/c", "config").with_weight(1),
            ],
            vec![
                task("config/a/y", "config/a").with_weight(1),
                task("config/a/x", "config/a"),
            ],
            vec![],
        );
        let paths = |pages: &[AdminPage]| pages.iter().map(|p| p.path.clone()).collect::<Vec<_>>();
        assert_eq!(
            paths(registry.pages_for_section("config")),
            ["config/a", "config/c", "config/b"]
        );
        assert_eq!(paths(registry.pages_for_section("people")), ["people/a"]);
        assert!(registry.pages_for_section("help").is_empty());
        assert_eq!(registry.page("config/b").unwrap().weight(), 2);
        let tasks: Vec<_> = registry
            .tasks_for("config/a")
            .iter()
            .map(|t| t.path())
            .collect();
        assert_eq!(tasks, ["config/a/x", "config/a/y"]);
    }

    #[test]
    fn references_to_undeclared_pieces_are_reported() {
        let mut sections = default_sections();
        sections.push(AdminSection::new("settings", Lc::n("Settings")).with_path("config"));
        let (registry, _) = AdminRegistry::new(
            sections,
            vec![page("config/site", "config"), page("tools/x", "tools")],
            vec![
                task("config/site/advanced", "config/site"),
                task("config/sit/typo", "config/sit"),
            ],
            vec![
                action("config/site/new", "config/site"),
                action("config/site/advanced/new", "config/site/advanced"),
                action("config/other/new", "config/other"),
            ],
        );

        let warnings = check_references(&registry);
        assert_eq!(warnings.len(), 4, "{warnings:?}");
        assert!(warnings[0].contains("\"settings\" and \"config\" share the path \"config\""));
        assert!(warnings[1].contains("\"tools/x\" belongs to the unknown section \"tools\""));
        assert!(warnings[2].contains("\"config/sit/typo\" belongs to the undeclared page"));
        assert!(warnings[3].contains("\"config/other/new\" is for the undeclared page"));
    }

    #[test]
    fn permissions_missing_from_the_catalog_are_reported_once_each() {
        let (registry, _) = AdminRegistry::new(
            vec![],
            vec![
                page("config/site", "config"),
                page("config/other", "config"),
            ],
            vec![task("config/site/advanced", "config/site").with_permission(&OtherPermission)],
            vec![],
        );

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
