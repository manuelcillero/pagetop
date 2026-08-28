use pagetop::prelude::*;

// **< Item >***************************************************************************************

/// Elemento de navegación individual en la barra lateral de AdminLTE.
///
/// Renderiza un `<li class="nav-item">` con un enlace `<a class="nav-link">`, un icono de
/// Bootstrap Icons y una etiqueta localizable. Si la ruta del ítem coincide con la del *request*
/// actual, el enlace se marca como activo con la clase `active`.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::bs::sidebar;
///
/// let item = sidebar::Item::link(
///     Lc::n("Usuarios"),
///     "/users",
///     "people",
/// );
/// ```
#[derive(AutoDefault, Clone, Getters)]
pub struct Item {
    /// Devuelve el texto localizable del ítem.
    label: Lc,
    /// Devuelve la ruta de destino en función del contexto.
    #[getters(skip)]
    route: Option<Route>,
    /// Devuelve el nombre del icono de Bootstrap Icons (sin el prefijo `bi-`).
    icon: CowStr,
}

#[async_trait]
impl Component for Item {
    fn new() -> Self {
        Self::default()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let Some(route) = self.route.as_ref() else {
            return Ok(html! {});
        };

        let route_link = route.resolve(cx);
        let current_path = cx.request().map(|r| r.path());
        let is_active = current_path == Some(route_link.path());

        let link_class = if is_active {
            "nav-link active"
        } else {
            "nav-link"
        };
        let aria_current = is_active.then_some("page");
        let icon_class = util::join!("nav-icon bi bi-", self.icon());

        Ok(html! {
            li class="nav-item" {
                a href=(route_link) class=(link_class) aria-current=[aria_current] {
                    i class=(icon_class) {}
                    p { (self.label().using(cx)) }
                }
            }
        })
    }
}

impl Item {
    /// Crea un ítem de navegación con etiqueta, ruta e icono.
    ///
    /// * `label` - Texto localizable del ítem.
    /// * `route` - Ruta de destino, resuelta según el contexto (ver [`Route`]).
    /// * `icon` - Nombre del icono de Bootstrap Icons sin el prefijo `bi-` (p. ej. `"people"`).
    pub fn link(label: Lc, route: impl Into<Route>, icon: impl Into<CowStr>) -> Self {
        Self {
            label,
            route: Some(route.into()),
            icon: icon.into(),
        }
    }

    // **< Item BUILDER >***************************************************************************

    /// Establece el texto localizable del ítem.
    #[builder_fn]
    pub fn with_label(mut self, label: Lc) -> Self {
        self.label = label;
        self
    }

    /// Establece la ruta de destino del ítem.
    #[builder_fn]
    pub fn with_route(mut self, route: impl Into<Option<Route>>) -> Self {
        self.route = route.into();
        self
    }

    /// Establece el nombre del icono de Bootstrap Icons (sin el prefijo `bi-`).
    #[builder_fn]
    pub fn with_icon(mut self, icon: impl Into<CowStr>) -> Self {
        self.icon = icon.into();
        self
    }
}
