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
    let Some(content) = sections_with_pages(page.context()) else {
        return deny_access(request);
    };
    page.with_title(Lc::l("admin_dashboard"))
        .with_child(content)
        .render()
        .await
        .into_response()
}

// **< Sección >************************************************************************************

// Página de una sección: sus páginas a las que puede acceder el usuario, sin repetir el título de
// la sección, que ya es el de la página. Si no puede acceder a ninguna, se le deniega el acceso
// como en cualquier otra ruta.
async fn section_page(request: HttpRequest) -> Response {
    let path = strip_admin_path(request.path()).unwrap_or_default();
    let Some(section) = AdminRegistry::global().section_by_path(path) else {
        return ErrorPage::NotFound(request).into_response();
    };

    let page = Page::admin(request.clone());
    let pages = accessible_pages(page.context(), section);
    if pages.is_empty() {
        return deny_access(request);
    }
    page.with_title(section.title().clone())
        .with_child(PageLinks { pages })
        .render()
        .await
        .into_response()
}

// **< Listado de secciones >***********************************************************************

// Mosaico de secciones, cada una con sus páginas a las que puede acceder el usuario, o `None` si no
// queda ninguna.
fn sections_with_pages(cx: &Context) -> Option<Mosaic> {
    let mut mosaic = Mosaic::new()
        .with_prop(PropsOp::add_classes("admin-dashboard-sections"))
        .with_columns(1)
        .with_columns_at(Breakpoint::Md, 2)
        .with_columns_at(Breakpoint::Xl, 3);
    let mut empty = true;
    for section in AdminRegistry::global().sections() {
        let pages = accessible_pages(cx, section);
        if !pages.is_empty() {
            empty = false;
            mosaic = mosaic.with_child(SectionCard {
                section: Some(section),
                pages,
            });
        }
    }
    (!empty).then_some(mosaic)
}

// Páginas de la sección a las que puede acceder el usuario.
fn accessible_pages(cx: &Context, section: &AdminSection) -> Vec<&'static AdminPage> {
    AdminRegistry::global()
        .pages_for_section(section.key())
        .iter()
        .filter(|p| p.is_accessible(cx))
        .collect()
}

// **< SectionCard >********************************************************************************

// Tarjeta de una sección en el inicio del panel: su icono y su título enlazado a la página de la
// sección, y debajo sus páginas.
#[derive(AutoDefault, Clone)]
struct SectionCard {
    section: Option<&'static AdminSection>,
    pages: Vec<&'static AdminPage>,
}

#[async_trait]
impl Component for SectionCard {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let Some(section) = self.section else {
            return Ok(html! {});
        };
        let icon = match section.icon() {
            Some(icon) => icon.clone().render(cx).await,
            None => html! {},
        };
        let links = PageLinks {
            pages: self.pages.clone(),
        }
        .render(cx)
        .await;
        Ok(html! {
            div class="admin-dashboard-section" {
                h2 class="admin-dashboard-section-title" {
                    a href=(cx.route(admin_path(section.path()))) {
                        (icon)
                        span { (section.title().using(cx)) }
                    }
                }
                (links)
            }
        })
    }
}

// **< PageLinks >**********************************************************************************

// Enlaces a las páginas, cada uno con su icono, su título y su descripción. El hueco del icono se
// reserva aunque la página no tenga, para que los títulos queden alineados.
#[derive(AutoDefault, Clone)]
struct PageLinks {
    pages: Vec<&'static AdminPage>,
}

#[async_trait]
impl Component for PageLinks {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let mut icons = Vec::with_capacity(self.pages.len());
        for page in &self.pages {
            icons.push(match page.icon() {
                Some(icon) => icon.clone().render(cx).await,
                None => html! {},
            });
        }
        Ok(html! {
            ul class="admin-dashboard-section-links" {
                @for (page, icon) in self.pages.iter().zip(icons) {
                    li class="admin-dashboard-link" {
                        a href=(cx.route(admin_path(page.path()))) {
                            span class="admin-dashboard-link-icon" { (icon) }
                            span class="admin-dashboard-link-title" { (page.title().using(cx)) }
                            @if !page.description().is_none() {
                                span class="admin-dashboard-link-desc" {
                                    (page.description().using(cx))
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}
