use crate::prelude::*;

// **< ItemKind >***********************************************************************************

/// Tipos de [`dropdown::Item`](super::Item) disponibles en un menú desplegable
/// [`Dropdown`](super::Dropdown).
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
    /// Acción ejecutable en la propia página, sin navegación asociada. Inicialmente puede estar
    /// deshabilitado.
    Button { label: Lc, disabled: bool },
    /// Título o encabezado que separa grupos de opciones.
    Header(Lc),
    /// Separador visual entre bloques de elementos.
    Divider,
}

// **< Item >***************************************************************************************

/// Representa un **elemento individual** de un menú desplegable [`Dropdown`](super::Dropdown).
///
/// Cada instancia de [`dropdown::Item`](super::Item) se traduce en un componente visible que
/// puede comportarse como texto, enlace, botón, encabezado o separador, según su [`ItemKind`].
///
/// Permite definir el identificador, las clases de estilo adicionales y el tipo de interacción
/// asociada, manteniendo una interfaz común para renderizar todos los elementos del menú.
///
/// Las etiquetas, los enlaces y los botones pueden llevar un icono al principio y otro al final
/// ([`with_icon_start()`](Self::with_icon_start) y [`with_icon_end()`](Self::with_icon_end)).
///
/// Un enlace no se renderiza si el usuario actual no puede acceder a su ruta (ver [`Endpoint`]).
/// Tampoco se muestra un encabezado si su grupo no tiene ningún elemento visible, ni un separador
/// que no quede entre elementos visibles (ver [`dropdown::Items`](super::Items)).
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Item {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el tipo de elemento representado.
    item_kind: ItemKind,
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

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let start = self.icon_start().render(cx).await;
        let end = self.icon_end().render(cx).await;
        Ok(match self.item_kind() {
            ItemKind::Void => html! {},

            ItemKind::Label(label) => html! {
                li (self.props().unpack(cx)) {
                    span class="dropdown-item-text" {
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
                let route_link = route.resolve(cx);
                let current_path = cx.request().map(|request| request.path());
                let is_current = !*disabled && (current_path == Some(route_link.path()));

                let mut classes = "dropdown-item".to_string();
                if is_current {
                    classes.push_str(" active");
                }
                if *disabled {
                    classes.push_str(" disabled");
                }

                let href = (!*disabled).then_some(route_link);
                let target = (!*disabled && *blank).then_some("_blank");
                let rel = (!*disabled && *blank).then_some("noopener noreferrer");

                let aria_current = (href.is_some() && is_current).then_some("page");
                let aria_disabled = (*disabled).then_some("true");
                let tabindex = disabled.then_some("-1");

                html! {
                    li (self.props().unpack(cx)) {
                        a
                            class=(classes)
                            href=[href]
                            target=[target]
                            rel=[rel]
                            aria-current=[aria_current]
                            aria-disabled=[aria_disabled]
                            tabindex=[tabindex]
                        {
                            (start) (label.using(cx)) (end)
                        }
                    }
                }
            }

            ItemKind::Button { label, disabled } => {
                let mut classes = "dropdown-item".to_string();
                if *disabled {
                    classes.push_str(" disabled");
                }

                let aria_disabled = disabled.then_some("true");
                let disabled_attr = disabled.then_some("disabled");

                html! {
                    li (self.props().unpack(cx)) {
                        button
                            class=(classes)
                            type="button"
                            aria-disabled=[aria_disabled]
                            disabled=[disabled_attr]
                        {
                            (start) (label.using(cx)) (end)
                        }
                    }
                }
            }

            ItemKind::Header(label) => html! {
                li (self.props().unpack(cx)) {
                    h6 class="dropdown-header" {
                        (label.using(cx))
                    }
                }
            },

            ItemKind::Divider => html! {
                li (self.props().unpack(cx)) { hr class="dropdown-divider" {} }
            },
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

    /// Crea un botón de acción local, sin navegación asociada.
    pub fn button(label: Lc) -> Self {
        Self {
            item_kind: ItemKind::Button {
                label,
                disabled: false,
            },
            ..Default::default()
        }
    }

    /// Crea un encabezado para un grupo de elementos dentro del menú.
    pub fn header(label: Lc) -> Self {
        Self {
            item_kind: ItemKind::Header(label),
            ..Default::default()
        }
    }

    /// Crea un separador visual entre bloques de elementos.
    pub fn divider() -> Self {
        Self {
            item_kind: ItemKind::Divider,
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
    /// interacción. Sólo se aplica a los enlaces y los botones; en los demás tipos no tiene efecto.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        if let ItemKind::Link { disabled: d, .. } | ItemKind::Button { disabled: d, .. } =
            &mut self.item_kind
        {
            *d = disabled;
        }
        self
    }

    /// Asigna o quita el icono al principio del elemento, antes de la etiqueta. Si se pasa `None`,
    /// no se mostrará.
    ///
    /// *Principio* y *final* siguen el sentido de la escritura. Se aplica a las etiquetas, los
    /// enlaces y los botones. El icono lleva la clase `icon-start` (`icon-end` el del final) para
    /// que cada tema lo coloque. Basic deja ese hueco libre en los elementos sin icono, para que
    /// todas las etiquetas del menú queden alineadas.
    pub fn with_icon_start(mut self, icon: impl Into<Option<Icon>>) -> Self {
        let icon = icon
            .into()
            .map(|icon| icon.with_prop(PropsOp::add_classes("icon-start")));
        self.icon_start.alter_component(icon);
        self
    }

    /// Asigna o quita el icono al final del elemento, después de la etiqueta. Si se pasa `None`, no
    /// se mostrará. Basic lo lleva junto al borde del menú. Ver
    /// [`with_icon_start()`](Self::with_icon_start).
    pub fn with_icon_end(mut self, icon: impl Into<Option<Icon>>) -> Self {
        let icon = icon
            .into()
            .map(|icon| icon.with_prop(PropsOp::add_classes("icon-end")));
        self.icon_end.alter_component(icon);
        self
    }
}
