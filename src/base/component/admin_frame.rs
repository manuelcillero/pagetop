use crate::prelude::*;

/// Marco de una página del panel de administración, alrededor de la región
/// [`CoreRegions::Content`].
///
/// Pinta, para la ruta de la petición actual, las migas de pan del panel, el título de la página
/// ([`Contextual::title()`]) con sus botones de acción ([`AdminAction`]), las pestañas de la
/// página ([`AdminTask`]) y, después, el contenido. Si la página no tiene título, usa el de la
/// página declarada para esa ruta, también en el `<title>` del documento:
///
/// ```html
/// <nav class="breadcrumb">...</nav>
/// <header class="admin-header">
///   <h1 class="admin-page-title">Title</h1>
///   <ul class="admin-actions-list">...</ul>
/// </header>
/// <nav class="admin-local-tasks">...</nav>
/// <div class="admin-content">...</div>
/// ```
///
/// La plantilla [`CoreTemplates::Admin`] lo pinta en lugar de la región `Content`, así que un
/// *handler* sólo tiene que crear la página con [`Page::admin()`], darle título y añadir su
/// contenido. Un tema puede capturarlo en [`Theme::render_component()`] para darle otro aspecto,
/// pero entonces le corresponde pintar también la región `Content`
/// ([`layout::Region::content()`](crate::base::component::layout::Region::content)).
///
/// Para una ruta que no es una página declarada (p. ej. la edición de un elemento de un listado),
/// las migas enlazan con la página declarada más cercana que la contiene.
///
/// [`Page::admin()`]: crate::response::Page::admin
/// [`Theme::render_component()`]: crate::core::theme::Theme::render_component
#[derive(AutoDefault, Clone, Debug)]
pub struct AdminFrame;

#[async_trait]
impl Component for AdminFrame {
    fn new() -> Self {
        Self
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        // Ruta de la petición relativa a la raíz del panel, como se guarda en el registro.
        let path = cx
            .request()
            .and_then(|r| strip_admin_path(r.path()))
            .unwrap_or_default()
            .to_owned();

        // Sin título propio, la página declarada aporta el suyo, también al `<head>` (que se
        // renderiza después del `<body>`).
        if cx.title().is_none()
            && let Some(page) = AdminRegistry::global().page(&path)
        {
            cx.alter_title(page.title().clone());
        }

        let breadcrumbs = breadcrumbs(cx, &path).render(cx).await;
        let actions = local_actions(cx, &path);
        let tasks = local_tasks(cx, &path);
        let content = layout::Region::content().render(cx).await;
        let title = cx.title().using(cx);

        Ok(html! {
            (breadcrumbs)
            header class="admin-header" {
                @if !title.is_empty() {
                    h1 class="admin-page-title" { (title) }
                }
                (actions)
            }
            @if !tasks.is_empty() {
                nav class="admin-local-tasks" { (tasks) }
            }
            div class="admin-content" {
                (content)
            }
        })
    }
}

// Inicio del panel, sección y página de la ruta. La última miga es la página actual, sin enlace.
fn breadcrumbs(cx: &Context, path: &str) -> Breadcrumb {
    if path.is_empty() {
        return Breadcrumb::new().with_crumb(breadcrumb::Crumb::current(Lc::l("admin_dashboard")));
    }
    let registry = AdminRegistry::global();
    let mut bc = Breadcrumb::new().with_crumb(breadcrumb::Crumb::new(
        Lc::l("admin_dashboard"),
        admin_path(""),
    ));
    if let Some(page) = registry.nearest_page(path) {
        if let Some(section) = registry.section(page.section()) {
            bc = bc.with_crumb(breadcrumb::Crumb::new(
                section.title().clone(),
                admin_path(section.path()),
            ));
        }
        if page.path() == path {
            bc = bc.with_crumb(breadcrumb::Crumb::current(page.title().clone()));
        } else {
            bc = bc.with_crumb(breadcrumb::Crumb::new(
                page.title().clone(),
                admin_path(page.path()),
            ));
            if !cx.title().is_none() {
                bc = bc.with_crumb(breadcrumb::Crumb::current(cx.title().clone()));
            }
        }
    } else if let Some(section) = registry.section_by_path(path) {
        // La ruta es la propia sección (su página de aterrizaje), no una página declarada en ella.
        bc = bc.with_crumb(breadcrumb::Crumb::current(section.title().clone()));
    }
    bc
}

// Pestañas de la página de la ruta, también cuando la ruta es una de ellas. Las que exigen un
// permiso que el usuario no tiene no se muestran.
fn local_tasks(cx: &Context, path: &str) -> Markup {
    let registry = AdminRegistry::global();
    let parent = registry.task(path).map_or(path, |task| task.parent_path());
    let tasks: Vec<_> = registry
        .tasks_for(parent)
        .iter()
        .filter(|task| is_granted(cx, task.permission()))
        .collect();
    if tasks.is_empty() {
        return html! {};
    }
    html! {
        ul class="admin-tasks-list" {
            @for task in tasks {
                li class="admin-task" .admin-task-active[task.path() == path] {
                    a href=(cx.route(admin_path(task.path()))) { (task.title().using(cx)) }
                }
            }
        }
    }
}

// Botones de acción de la página de la ruta. Los que exigen un permiso que el usuario no tiene no
// se muestran.
fn local_actions(cx: &Context, path: &str) -> Markup {
    let actions: Vec<_> = AdminRegistry::global()
        .actions_for(path)
        .iter()
        .filter(|action| is_granted(cx, action.permission()))
        .collect();
    if actions.is_empty() {
        return html! {};
    }
    html! {
        ul class="admin-actions-list" {
            @for action in actions {
                li class="admin-action" {
                    a class="admin-action-link" href=(cx.route(admin_path(action.url()))) {
                        (action.title().using(cx))
                    }
                }
            }
        }
    }
}

// Sin permiso propio (`None`) basta con ver la página, cuyo permiso ya ha exigido su ruta.
fn is_granted(cx: &Context, permission: Option<PermissionRef>) -> bool {
    permission.is_none_or(|permission| {
        cx.request()
            .is_some_and(|request| has_permission(request, permission))
    })
}
