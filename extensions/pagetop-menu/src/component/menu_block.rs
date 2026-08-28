use pagetop::prelude::*;

use crate::tree::{MenuKey, MenuNode, TreeOptions, build_tree, try_resolve_menu_url};

/// Renderiza un menú completo como bloque de navegación HTML.
///
/// Se construye componiendo [`Nav`] y [`nav::Item`] -- con [`Dropdown`]/[`dropdown::Item`] para
/// los nodos con hijos -- a partir del árbol del menú, así que produce el mismo marcado accesible
/// que cualquier `Nav`/`Dropdown` y se beneficia igual del CSS/JavaScript que aporta el tema
/// activo:
///
/// ```html
/// <nav aria-label="Main">
///   <ul class="nav">
///     <li class="nav-item"><a class="nav-link" aria-current="page" href="/">Home</a></li>
///   </ul>
/// </nav>
/// ```
///
/// Los temas pueden sobreescribir el render con `handle_component()`, tanto de `MenuBlock` como,
/// más generalmente, de [`Nav`]/[`nav::Item`]/[`Dropdown`]/[`dropdown::Item`]. `pagetop-bootsier`
/// ya intercepta `Dropdown` así (ver `theme::bs::dropdown`), y por tanto también los que cuelguen
/// de un `nav::Item::dropdown()`; `MenuBlock`, `Nav` y `Navbar` siguen sin interceptarse: Bootsier
/// mantiene sus propios `bs::Navbar`/`bs::Nav`, sin relación con este componente.
///
/// # Limitaciones conocidas
///
/// - **Profundidad máxima de 2 niveles.** [`Dropdown`] no admite submenús anidados (como
///   Bootstrap, del que toma su marcado): un nodo de tercer nivel o más profundo nunca se
///   construye -- [`TreeOptions::max_depth`] se acota internamente a `2` con independencia de lo
///   que indique [`with_max_depth()`](Self::with_max_depth), así que no hay pérdida silenciosa de
///   datos, sencillamente no se piden a la base de datos.
/// - **Sin colapso responsive propio.** A diferencia del antiguo `Menu::collapsible`, `Nav` es una
///   lista plana sin botón ni JavaScript de colapso; una aplicación que necesite ese
///   comportamiento debe envolver `MenuBlock` en su propia chrome hasta que exista un componente
///   `Navbar` en el core.
/// - `MenuNode::in_active_trail` y `MenuNode::expanded` (ver [`crate::tree::MenuNode`]) todavía no
///   se reflejan en el marcado -- ni [`nav::Item`] ni [`dropdown::Item`] tienen hoy una forma
///   verificada de pre-abrirse en el servidor sin que la mejora progresiva del tema
///   (`accessible-menu` en `Basic`) lo sobrescriba al inicializarse. Sólo se traduce el estado
///   `is_active`/`enabled` de cada nodo.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_menu::component::MenuBlock;
///
/// async fn handler(request: HttpRequest) -> Result<Markup, ErrorPage> {
///     Page::new(request)
///         .with_child(MenuBlock::with("main"))
///         .render().await
/// }
/// ```
#[derive(AutoDefault, Clone, Debug)]
pub struct MenuBlock {
    menu_name: Option<String>,
    show_title: bool,
    max_depth: Option<u8>,
    include_disabled: bool,
    hide_when_empty: bool,
}

#[async_trait]
impl Component for MenuBlock {
    fn new() -> Self {
        MenuBlock {
            hide_when_empty: true,
            ..Self::default()
        }
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let Some(name) = &self.menu_name else {
            return Ok(html! {});
        };

        // `Dropdown` no admite submenús: nunca se piden más de 2 niveles al árbol.
        let opts = TreeOptions {
            max_depth: Some(self.max_depth.map(|d| d.min(2)).unwrap_or(2)),
            include_disabled: self.include_disabled,
        };

        let Some(tree) = build_tree(MenuKey::Name(name.clone()), cx, &opts).await else {
            return Ok(html! {});
        };

        if tree.roots.is_empty() && self.hide_when_empty {
            return Ok(html! {});
        }

        let mut nav = Nav::new();
        for node in &tree.roots {
            nav = nav.with_item(node_to_item(node, cx));
        }

        let aria_label = (!tree.title.is_empty()).then_some(&tree.title);

        Ok(html! {
            @if self.show_title {
                h2.menu-title { (&tree.title) }
            }
            nav aria-label=[aria_label] {
                (nav.render(cx).await)
            }
        })
    }
}

impl MenuBlock {
    /// Crea un `MenuBlock` para el menú con el `machine_name` dado.
    pub fn with(menu_name: impl Into<String>) -> Self {
        let mut block = Self::new();
        block.menu_name = Some(menu_name.into());
        block
    }

    #[builder_fn]
    pub fn with_show_title(mut self, v: impl Into<Option<bool>>) -> Self {
        if let Some(v) = v.into() {
            self.show_title = v;
        }
        self
    }

    /// Establece la profundidad máxima de nodos a incluir. El efectivo nunca supera `2`, por muy
    /// alto que sea el valor indicado (ver "Limitaciones conocidas" en [`MenuBlock`]).
    #[builder_fn]
    pub fn with_max_depth(mut self, v: impl Into<Option<u8>>) -> Self {
        self.max_depth = v.into();
        self
    }

    #[builder_fn]
    pub fn with_include_disabled(mut self, v: impl Into<Option<bool>>) -> Self {
        if let Some(v) = v.into() {
            self.include_disabled = v;
        }
        self
    }

    #[builder_fn]
    pub fn with_hide_when_empty(mut self, v: impl Into<Option<bool>>) -> Self {
        if let Some(v) = v.into() {
            self.hide_when_empty = v;
        }
        self
    }
}

// **< Traducción de MenuNode a nav::Item / dropdown::Item >****************************************

// Convierte un `MenuNode` de nivel 1 (raíz) en un `nav::Item`: sin hijos, enlace (o etiqueta sin
// ruta propia); con hijos, activador de un `Dropdown` con sus hijos como `dropdown::Item`. Los
// hijos de un `MenuNode` de nivel 1 nunca tienen a su vez hijos propios -- `TreeOptions::max_depth`
// se acota a 2 en `MenuBlock::prepare()`, así que no hay un tercer nivel que representar.
fn node_to_item(node: &MenuNode, cx: &Context) -> nav::Item {
    if !node.children.is_empty() {
        let mut dropdown = Dropdown::new().with_title(Lc::n(node.title.clone()));
        for child in &node.children {
            dropdown = dropdown.with_item(child_to_item(child, cx));
        }
        return nav::Item::dropdown(dropdown);
    }

    let label = Lc::n(node.title.clone());
    let is_external = node.url.as_ref().is_some_and(RoutePath::is_external);
    let disabled = !node.enabled;

    let Some(route) = try_resolve_menu_url(node.url.as_ref(), cx).map(Route::from) else {
        return nav::Item::label(label);
    };
    match (is_external, disabled) {
        (true, true) => nav::Item::link_blank_disabled(label, route),
        (true, false) => nav::Item::link_blank(label, route),
        (false, true) => nav::Item::link_disabled(label, route),
        (false, false) => nav::Item::link(label, route),
    }
    .with_active(node.is_active)
}

// Convierte un `MenuNode` de nivel 2 en un `dropdown::Item`: sin ruta, etiqueta no interactiva; con
// ruta, enlace. `Dropdown` no admite submenús, así que no hay recursión posible aquí.
fn child_to_item(node: &MenuNode, cx: &Context) -> dropdown::Item {
    let label = Lc::n(node.title.clone());
    let is_external = node.url.as_ref().is_some_and(RoutePath::is_external);
    let disabled = !node.enabled;

    let Some(route) = try_resolve_menu_url(node.url.as_ref(), cx).map(Route::from) else {
        return dropdown::Item::label(label);
    };
    match (is_external, disabled) {
        (true, true) => dropdown::Item::link_blank_disabled(label, route),
        (true, false) => dropdown::Item::link_blank(label, route),
        (false, true) => dropdown::Item::link_disabled(label, route),
        (false, false) => dropdown::Item::link(label, route),
    }
}
