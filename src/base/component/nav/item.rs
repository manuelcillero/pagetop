use crate::prelude::*;

// **< ItemKind >***********************************************************************************

/// Tipos de [`nav::Item`](super::Item) disponibles en un menú [`Nav`](super::Nav).
///
/// Define internamente la naturaleza del elemento y su comportamiento al mostrarse o interactuar
/// con él.
#[derive(AutoDefault, Clone, Debug)]
pub enum ItemKind {
    /// Elemento vacío, no produce salida.
    #[default]
    Void,
    /// Etiqueta sin comportamiento interactivo.
    Label(Lc),
    /// Elemento de navegación basado en una [`RoutePath`] dinámica resuelta por una [`Route`].
    /// Opcionalmente, puede abrirse en una nueva ventana y estar inicialmente deshabilitado.
    Link {
        label: Lc,
        route: Route,
        blank: bool,
        disabled: bool,
    },
    /// Contenido HTML arbitrario. El componente [`Html`] se renderiza tal cual como elemento del
    /// menú, sin añadir ningún comportamiento de navegación adicional.
    Html(Embed<Html>),
    /// Elemento que despliega un menú [`Dropdown`](super::super::Dropdown).
    Dropdown(Embed<Dropdown>),
}

impl ItemKind {
    const ITEM: &str = "nav-item";
    const DROPDOWN: &str = "nav-item dropdown";

    /// Devuelve las clases base asociadas al tipo de elemento.
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Void => "",
            Self::Dropdown(_) => Self::DROPDOWN,
            _ => Self::ITEM,
        }
    }
}

// **< Item >***************************************************************************************

/// Representa un **elemento individual** de un menú [`Nav`](super::Nav).
///
/// Cada instancia de [`nav::Item`](super::Item) se traduce en un componente visible que puede
/// comportarse como texto, enlace, contenido HTML o menú desplegable, según su [`ItemKind`].
///
/// Permite definir el identificador, las clases de estilo adicionales y el tipo de interacción
/// asociada, manteniendo una interfaz común para renderizar todos los elementos del menú.
///
/// Las etiquetas, los enlaces y el título de un desplegable pueden llevar un icono al principio y
/// otro al final ([`with_icon_start()`](Self::with_icon_start) y
/// [`with_icon_end()`](Self::with_icon_end)).
///
/// Un enlace no se renderiza si el usuario actual no puede acceder a su ruta (ver [`Endpoint`]).
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Item {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el tipo de elemento representado.
    item_kind: ItemKind,
    /// Devuelve el valor de `active` forzado explícitamente, si lo hay.
    ///
    /// Sin forzar (`None`), un [`ItemKind::Link`] se marca activo cuando su ruta coincide
    /// exactamente con la del *request* actual (ver [`prepare()`](Component::prepare)). Forzarlo
    /// permite otros criterios, como marcar activa una sección mientras la ruta actual sea
    /// cualquiera de sus subrutas.
    active_override: Option<bool>,
    /// Devuelve el icono al principio del elemento, antes de la etiqueta (si lo hay).
    icon_start: Embed<Icon>,
    /// Devuelve el icono al final del elemento, después de la etiqueta (si lo hay).
    icon_end: Embed<Icon>,
}

#[async_trait]
impl Component for Item {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        match self.item_kind() {
            ItemKind::Link { route, .. } => route.is_accessible(cx),
            _ => true,
        }
    }

    fn setup(&mut self, _cx: &mut Context) {
        self.alter_prop(PropsOp::prepend_classes(self.item_kind().as_str()));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let start = self.icon_start().render(cx).await;
        let end = self.icon_end().render(cx).await;
        Ok(match self.item_kind() {
            ItemKind::Void => html! {},

            ItemKind::Label(label) => html! {
                li (self.props().unpack(cx)) {
                    span class="nav-link disabled" aria-disabled="true" {
                        (start) (label.using(cx)) (end)
                    }
                }
            },

            ItemKind::Link {
                label,
                route,
                blank,
                disabled,
            } => {
                let enabled = !*disabled;

                // Deshabilitado, `href` no la usa: evita resolver la ruta para nada.
                let route_link = enabled.then(|| route.resolve(cx));
                let current_path = cx.request().map(|request| request.path());
                let is_current = self
                    .active_override()
                    .copied()
                    .unwrap_or(enabled && route_link.as_ref().map(RoutePath::path) == current_path);

                let active_class = if is_current { " active" } else { "" };
                let disabled_class = if *disabled { " disabled" } else { "" };
                let classes = util::join!("nav-link", active_class, disabled_class);

                let target = (enabled && *blank).then_some("_blank");
                let rel = (enabled && *blank).then_some("noopener noreferrer");
                let aria_current = (enabled && is_current).then_some("page");
                let aria_disabled = (*disabled).then_some("true");

                html! {
                    li (self.props().unpack(cx)) {
                        a
                            class=(classes)
                            href=[route_link]
                            target=[target]
                            rel=[rel]
                            aria-current=[aria_current]
                            aria-disabled=[aria_disabled]
                        {
                            (start) (label.using(cx)) (end)
                        }
                    }
                }
            }

            ItemKind::Html(html) => html! {
                li (self.props().unpack(cx)) {
                    (html.render(cx).await)
                }
            },

            ItemKind::Dropdown(menu) => {
                if let Some(dd) = menu.get() {
                    let items = dd.items().render(cx).await;
                    if items.is_empty() {
                        return Ok(html! {});
                    }
                    let title = dd.title().using(cx);
                    let title = if title.is_empty() {
                        Lc::l("dropdown_default_title").using(cx)
                    } else {
                        title
                    };
                    html! {
                        li (self.props().unpack(cx)) {
                            a
                                class="nav-link dropdown-toggle"
                                href="#"
                                role="button"
                                aria-haspopup="true"
                                aria-expanded="false"
                            {
                                (start) (title) (end)
                            }
                            ul class=(dd.menu_classes()) {
                                (items)
                            }
                        }
                    }
                } else {
                    html! {}
                }
            }
        })
    }
}

#[builder_impl]
impl Item {
    /// Crea un elemento de tipo texto, mostrado sin interacción.
    pub fn label(label: Lc) -> Self {
        Self {
            item_kind: ItemKind::Label(label),
            ..Default::default()
        }
    }

    /// Crea un enlace para la navegación.
    ///
    /// La ruta se obtiene invocando [`Route::resolve()`], que devuelve dinámicamente una
    /// [`RoutePath`] en función del [`Context`]. El enlace se marca como `active` si la ruta
    /// actual del *request* coincide con la ruta de destino (devuelta por `RoutePath::path`).
    pub fn link(label: Lc, route: impl Into<Route>) -> Self {
        Self {
            item_kind: ItemKind::Link {
                label,
                route: route.into(),
                blank: false,
                disabled: false,
            },
            ..Default::default()
        }
    }

    /// Crea un enlace que se abre en una nueva ventana o pestaña.
    pub fn link_blank(label: Lc, route: impl Into<Route>) -> Self {
        Self {
            item_kind: ItemKind::Link {
                label,
                route: route.into(),
                blank: true,
                disabled: false,
            },
            ..Default::default()
        }
    }

    /// Crea un elemento con contenido HTML arbitrario.
    ///
    /// El contenido se renderiza tal cual lo devuelve el componente [`Html`], dentro de un `<li>`
    /// con las clases de navegación asociadas a [`Item`].
    pub fn html(html: Html) -> Self {
        Self {
            item_kind: ItemKind::Html(Embed::with(html)),
            ..Default::default()
        }
    }

    /// Crea un elemento de navegación que contiene un menú desplegable
    /// [`Dropdown`](super::super::Dropdown).
    ///
    /// Sólo se tienen en cuenta **el título** (si no existe, se asigna uno por defecto) y **la
    /// lista de elementos** del [`Dropdown`](super::super::Dropdown); el resto de propiedades no
    /// afectarán a su representación en [`Nav`](super::Nav).
    pub fn dropdown(menu: Dropdown) -> Self {
        Self {
            item_kind: ItemKind::Dropdown(Embed::with(menu)),
            ..Default::default()
        }
    }

    // **< Item BUILDER >***************************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    pub fn with_prop(mut self, op: impl Into<PropsOp>) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece si el elemento está deshabilitado: se muestra atenuado y no permite la
    /// interacción. Sólo se aplica a los enlaces; en los demás tipos no tiene efecto.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        if let ItemKind::Link { disabled: d, .. } = &mut self.item_kind {
            *d = disabled;
        }
        self
    }

    /// Asigna o quita el icono al principio del elemento, antes de la etiqueta. Si se pasa `None`,
    /// no se mostrará.
    ///
    /// *Principio* y *final* siguen el sentido de la escritura. Se aplica a las etiquetas, los
    /// enlaces y el título de un desplegable. El icono lleva la clase `icon-start` (`icon-end` el
    /// del final) para que cada tema lo coloque. Basic, en un menú vertical, deja ese hueco libre
    /// en los elementos sin icono, para que todas las etiquetas queden alineadas.
    pub fn with_icon_start(mut self, icon: impl Into<Option<Icon>>) -> Self {
        let icon = icon
            .into()
            .map(|icon| icon.with_prop(PropsOp::add_classes("icon-start")));
        self.icon_start.alter_component(icon);
        self
    }

    /// Asigna o quita el icono al final del elemento, después de la etiqueta. Si se pasa `None`, no
    /// se mostrará. Basic, en un menú vertical, lo lleva junto al borde (en el título de un
    /// desplegable, seguido de su flecha). Ver
    /// [`with_icon_start()`](Self::with_icon_start).
    pub fn with_icon_end(mut self, icon: impl Into<Option<Icon>>) -> Self {
        let icon = icon
            .into()
            .map(|icon| icon.with_prop(PropsOp::add_classes("icon-end")));
        self.icon_end.alter_component(icon);
        self
    }

    /// Fuerza si un [`ItemKind::Link`] se marca activo, o `None` para volver a la detección
    /// automática por coincidencia exacta de ruta.
    pub fn with_active(mut self, active: impl Into<Option<bool>>) -> Self {
        self.active_override = active.into();
        self
    }
}
