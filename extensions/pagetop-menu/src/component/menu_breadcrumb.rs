use pagetop::base::component::breadcrumb;
use pagetop::prelude::*;

use crate::tree::{MenuKey, MenuNode, TreeOptions, build_tree, try_resolve_menu_url};

/// Migas de pan del menú dado, basándose en el active trail.
///
/// Resuelve el árbol del menú y su active trail (de forma asíncrona, en su propio
/// [`prepare()`](Component::prepare)) y delega el renderizado en
/// [`Breadcrumb`](pagetop::base::component::Breadcrumb): esta extensión sólo aporta los datos, la
/// estructura HTML y las clases CSS son responsabilidad del componente base.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_menu::component::MenuBreadcrumb;
///
/// async fn handler(request: HttpRequest) -> Result<Markup, ErrorPage> {
///     Page::new(request)
///         .with_child(MenuBreadcrumb::with("main"))
///         .render().await
/// }
/// ```
#[derive(AutoDefault, Clone, Debug)]
pub struct MenuBreadcrumb {
    menu_name: Option<String>,
    include_current: bool,
}

#[async_trait]
impl Component for MenuBreadcrumb {
    fn new() -> Self {
        MenuBreadcrumb {
            include_current: true,
            ..Self::default()
        }
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let Some(name) = &self.menu_name else {
            return Ok(html! {});
        };

        let opts = TreeOptions::default();
        let Some(tree) = build_tree(MenuKey::Name(name.clone()), cx, &opts).await else {
            return Ok(html! {});
        };

        let path = extract_trail(&tree.roots);
        if path.is_empty() {
            return Ok(html! {});
        }

        let mut inner = Breadcrumb::new();
        let last = path.len() - 1;
        for (i, node) in path.iter().enumerate() {
            let label = Lc::n(node.title.clone());
            if i == last {
                if self.include_current {
                    inner = inner.with_crumb(breadcrumb::Crumb::current(label));
                }
            } else if let Some(url) = node.url.clone() {
                let route =
                    Route::with(move |cx| try_resolve_menu_url(Some(&url), cx).unwrap_or_default());
                inner = inner.with_crumb(breadcrumb::Crumb::new(label, route));
            } else {
                inner = inner.with_crumb(breadcrumb::Crumb::text(label));
            }
        }

        Ok(inner.render(cx).await)
    }
}

impl MenuBreadcrumb {
    /// Crea un `MenuBreadcrumb` para el menú con el `machine_name` dado.
    pub fn with(menu_name: impl Into<String>) -> Self {
        let mut bc = Self::new();
        bc.menu_name = Some(menu_name.into());
        bc
    }

    #[builder_fn]
    pub fn with_include_current(mut self, v: impl Into<Option<bool>>) -> Self {
        if let Some(v) = v.into() {
            self.include_current = v;
        }
        self
    }
}

// Extrae la cadena raíz -> nodo activo recorriendo el active trail.
fn extract_trail(nodes: &[MenuNode]) -> Vec<&MenuNode> {
    for node in nodes {
        if node.in_active_trail || node.is_active {
            let mut path = extract_trail(&node.children);
            path.insert(0, node);
            return path;
        }
    }
    vec![]
}
