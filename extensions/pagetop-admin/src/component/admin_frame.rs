use pagetop::base::component::breadcrumb;
use pagetop::prelude::*;

use crate::LOCALES_ADMIN;
use crate::registry;

/// Componente de layout principal de una página del panel de administración.
///
/// Renderiza breadcrumb, encabezado (título + acciones locales), tareas locales (pestañas) y el
/// contenido de la página. No incluye ningún menú de navegación: mostrar las secciones del panel
/// (el "sidebar" o el "menú superior", según el tema) es responsabilidad del tema que intercepte
/// [`CoreTemplates::Admin`](pagetop::core::theme::CoreTemplates::Admin) -- por ejemplo
/// `pagetop-bootsier` -- leyendo directamente [`crate::registry::global()`]. El tema básico de
/// PageTop no lo hace, y una página de administración sigue siendo completamente navegable sin él,
/// a través del *dashboard* (`/admin`), las páginas de sección y este mismo breadcrumb.
///
/// ```html
/// <nav class="admin-breadcrumb">...</nav>
/// <header class="admin-header">
///   <h1 class="admin-page-title">Title</h1>
///   <ul class="admin-actions-list">...</ul>
/// </header>
/// <nav class="admin-local-tasks">...</nav>
/// <div class="admin-content">...</div>
/// ```
///
/// El contenido son sus propios hijos ([`Children`]), igual que en cualquier otro componente
/// contenedor (p. ej. [`Block`](pagetop::base::component::Block)): admite tantos `with_child(...)`
/// como haga falta, cada uno renderizado en el ciclo normal de componentes.
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct AdminFrame {
    /// Devuelve el título de la página.
    title: Lc,
    /// Devuelve la lista de componentes hijo de la página.
    children: Children,
}

#[async_trait]
impl Component for AdminFrame {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let path = cx
            .request()
            .map(|r| r.path().to_string())
            .unwrap_or_else(|| "/".to_string());

        let breadcrumbs = render_breadcrumbs(cx, &path).await;
        let local_tasks = render_local_tasks(cx, &path);
        let local_acts = render_local_actions(cx, &path);
        let body = self.children().render(cx).await;

        Ok(html! {
            (breadcrumbs)
            header.admin-header {
                h1.admin-page-title { (self.title().using(cx)) }
                (local_acts)
            }
            @if !local_tasks.0.is_empty() {
                nav.admin-local-tasks { (local_tasks) }
            }
            div.admin-content {
                (body)
            }
        })
    }
}

#[builder_impl]
impl AdminFrame {
    /// Establece el título de la página.
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }

    /// Añade un componente hijo al contenido de la página.
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.children.alter_child(op.into());
        self
    }
}

// **< Breadcrumbs >********************************************************************************

async fn render_breadcrumbs(cx: &mut Context, current_path: &str) -> Markup {
    let reg = registry::global();
    let base = crate::ADMIN_BASE_PATH;

    let home_label = Lc::t("admin-breadcrumb-home", &LOCALES_ADMIN);
    let mut bc = Breadcrumb::new().with_crumb(breadcrumb::Crumb::new(home_label, base));

    if let Some(page) = reg.pages().get(current_path) {
        if let Some(section) = reg.sections().get(&page.section) {
            bc = bc.with_crumb(breadcrumb::Crumb::new(
                section.title.clone(),
                section.path.as_str(),
            ));
        }
        bc = bc.with_crumb(breadcrumb::Crumb::current(page.title.clone()));
    } else if let Some(section) = reg.sections().values().find(|s| s.path == current_path) {
        // La ruta actual es la propia sección (su página de aterrizaje), no una `AdminPage`
        // registrada dentro de ella.
        bc = bc.with_crumb(breadcrumb::Crumb::current(section.title.clone()));
    }

    bc.render(cx).await
}

// **< Tareas locales >*****************************************************************************

fn render_local_tasks(cx: &Context, current_path: &str) -> Markup {
    let reg = registry::global();
    let tasks = reg.tasks_for(current_path);
    if tasks.is_empty() {
        return html! {};
    }
    html! {
        ul.admin-tasks-list {
            @for task in tasks {
                @let is_active = current_path == task.path;
                li class=(if is_active { "admin-task admin-task-active" } else { "admin-task" }) {
                    a href=(cx.route(task.path.as_str())) { (task.title.using(cx)) }
                }
            }
        }
    }
}

// **< Acciones locales >***************************************************************************

fn render_local_actions(cx: &Context, current_path: &str) -> Markup {
    let reg = registry::global();
    let actions = reg.actions_for(current_path);
    if actions.is_empty() {
        return html! {};
    }
    html! {
        ul.admin-actions-list {
            @for action in actions {
                li.admin-action {
                    a.admin-action-link href=(cx.route(action.url.as_str())) { (action.title.using(cx)) }
                }
            }
        }
    }
}
