use crate::prelude::*;

// **< Rutas >**************************************************************************************

// Monta el inicio del panel (*dashboard*) y la página de cada sección con páginas declaradas, sólo
// si hay alguna. Una sección cuya ruta reclama una página declarada no se monta: esa página la
// monta su extensión y gana. Una ruta se monta una sola vez (montarla dos veces detendría el
// arranque): si dos secciones la comparten, sirve la primera por peso.
pub(crate) fn configure_routes(router: Router) -> Router {
    let registry = AdminRegistry::global();
    if registry.pages().is_empty() {
        return router;
    }
    let mut router = router.route(&admin_path(""), web::get(dashboard));
    let mut mounted = std::collections::BTreeSet::new();
    for section in registry.sections() {
        if section.path().is_empty()
            || registry.page(section.path()).is_some()
            || registry.pages_for_section(section.key()).is_empty()
            || !mounted.insert(section.path())
        {
            continue;
        }
        router = router.route(&admin_path(section.path()), web::get(section_page));
    }
    router
}

// **< Dashboard >**********************************************************************************

// Inicio del panel: todas las secciones con las páginas a las que puede acceder el usuario. Si no
// puede acceder a ninguna, se le deniega el acceso como en cualquier otra ruta.
async fn dashboard(request: HttpRequest) -> Response {
    let page = Page::admin(request.clone());
    let sections = AdminRegistry::global().sections();
    let Some(content) = sections_with_pages(page.context(), sections) else {
        return deny_access(request);
    };
    page.with_title(Lc::l("admin_dashboard"))
        .with_child(content)
        .render()
        .await
        .into_response()
}

// **< Sección >************************************************************************************

// Página de una sección: sus páginas a las que puede acceder el usuario. Si no puede acceder a
// ninguna, se le deniega el acceso como en cualquier otra ruta.
async fn section_page(request: HttpRequest) -> Response {
    let path = strip_admin_path(request.path()).unwrap_or_default();
    let Some(section) = AdminRegistry::global().section_by_path(path) else {
        return ErrorPage::NotFound(request).into_response();
    };

    let page = Page::admin(request.clone());
    let Some(content) = sections_with_pages(page.context(), std::slice::from_ref(section)) else {
        return deny_access(request);
    };
    page.with_title(section.title().clone())
        .with_child(content)
        .render()
        .await
        .into_response()
}

// **< Listado de secciones >***********************************************************************

// Secciones con sus páginas (título y descripción) a las que puede acceder el usuario, o `None` si
// no queda ninguna.
fn sections_with_pages(cx: &Context, candidates: &'static [AdminSection]) -> Option<Container> {
    let registry = AdminRegistry::global();
    let visible: Vec<_> = candidates
        .iter()
        .map(|section| {
            let pages: Vec<_> = registry
                .pages_for_section(section.key())
                .iter()
                .filter(|p| p.is_accessible(cx))
                .collect();
            (section, pages)
        })
        .filter(|(_, pages)| !pages.is_empty())
        .collect();
    if visible.is_empty() {
        return None;
    }
    let mut sections = Container::new().with_prop(PropsOp::add_classes("admin-dashboard-sections"));
    for (section, pages) in visible {
        sections = sections.with_child(
            Container::new()
                .with_prop(PropsOp::add_classes("admin-dashboard-section"))
                .with_child(Html::with(move |cx| {
                    html! {
                        h2 class="admin-dashboard-section-title" {
                            a href=(cx.route(admin_path(section.path()))) {
                                (section.title().using(cx))
                            }
                        }
                        ul class="admin-dashboard-section-links" {
                            @for page in &pages {
                                li class="admin-dashboard-link" {
                                    a href=(cx.route(admin_path(page.path()))) {
                                        (page.title().using(cx))
                                    }
                                    @if !page.description().is_none() {
                                        span class="admin-dashboard-link-desc" {
                                            " - " (page.description().using(cx))
                                        }
                                    }
                                }
                            }
                        }
                    }
                })),
        );
    }
    Some(sections)
}
