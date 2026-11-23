use crate::prelude::*;

/// Componente para crear un **recuadro de acceso** (*tile*) a una sección.
///
/// Es un enlace grande con un título, una descripción breve opcional y un icono, pensado para la
/// portada de una aplicación o de una sección: varios recuadros dentro de un [`Grid`] forman un
/// panel de accesos. Cada uno lleva un color de acento ([`Accent`]) que lo distingue de los
/// demás y, opcionalmente, una imagen de fondo que el tema difumina bajo una capa de ese color
/// para que el texto se lea siempre bien. La imagen es decorativa: el enlace se nombra con el
/// título y la descripción.
///
/// Un recuadro no se renderiza si el usuario actual no puede acceder a su ruta (ver [`Endpoint`]),
/// así que un panel de accesos muestra a cada uno sólo lo que puede abrir. Para mostrar un acceso
/// que aún no se puede usar, se deshabilita con [`with_disabled()`](Self::with_disabled), y para
/// abrirlo en una nueva ventana o pestaña se crea con [`Tile::link_blank()`].
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let panel = Grid::new()
///     .with_columns(grid::Tracks::repeat(1, grid::AxisTrack::Fraction(1.0)))
///     .with_columns_at(
///         Breakpoint::Md,
///         grid::Tracks::repeat(2, grid::AxisTrack::Fraction(1.0)),
///     )
///     .with_columns_at(
///         Breakpoint::Lg,
///         grid::Tracks::repeat(3, grid::AxisTrack::Fraction(1.0)),
///     )
///     .with_gap(align::Gap::Both(UnitValue::RelRem(1.5)))
///     .with_child(
///         Tile::link(Lc::n("Orders"), "/orders")
///             .with_description(Lc::n("Check and process the orders"))
///             .with_icon(Icon::with(icon::Core::ChevronRight))
///             .with_image(image::Source::plain("/assets/img/orders.jpg"))
///             .with_accent(Accent::Orange),
///     );
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Tile {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve la ruta a la que lleva el recuadro.
    route: Route,
    /// Devuelve el título del recuadro.
    title: Lc,
    /// Devuelve la descripción breve del recuadro (puede ser [`Lc::none()`]).
    description: Lc,
    /// Devuelve el icono del recuadro (si lo hay).
    icon: Embed<Icon>,
    /// Devuelve la imagen de fondo del recuadro (si la hay).
    image: Embed<Image>,
    /// Devuelve el color de acento del recuadro.
    #[getters(copy)]
    accent: Accent,
    /// Devuelve si el recuadro se abre en una nueva ventana o pestaña.
    blank: bool,
    /// Devuelve si el recuadro está deshabilitado.
    disabled: bool,
}

#[async_trait]
impl Component for Tile {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        self.route().is_accessible(cx)
    }

    fn setup(&mut self, cx: &mut Context) {
        self.alter_prop(PropsOp::prepend_classes(util::join!(
            "tile accent-",
            self.accent().as_str()
        )));
        // Un `<a>` sin `href` (ver `prepare()`) no tiene un estado deshabilitado nativo.
        if *self.disabled() {
            self.alter_prop(PropsOp::add_classes("disabled"));
        }
        // Por `Props` y no en el `<a>` de `prepare()`: un atributo escrito en el elemento omite el
        // mismo atributo de `Props`, y se perdería un `target` añadido con `with_prop()`.
        if *self.blank() && !*self.disabled() && self.route().try_resolve(cx).is_some() {
            self.alter_prop(PropsOp::set("target", "_blank"));
            self.alter_prop(PropsOp::set("rel", "noopener noreferrer"));
        }
        if let Some(image) = self.image.get_mut() {
            image.alter_prop(PropsOp::add_classes("tile-image"));
            image.alter_alternative(Lc::none());
        }
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let disabled = *self.disabled();
        // Deshabilitado, sin `href`: evita resolver la ruta para nada.
        let href = (!disabled).then(|| self.route().try_resolve(cx)).flatten();
        let image = self.image().render(cx).await;
        let icon = self.icon().render(cx).await;
        Ok(html! {
            a
                href=[href]
                aria-disabled=[disabled.then_some("true")]
                (self.props().unpack(cx))
            {
                (image)
                span class="tile-body" {
                    @if self.icon().get().is_some() {
                        span class="tile-icon" { (icon) }
                    }
                    span class="tile-title" { (self.title().using(cx)) }
                    @if !self.description().is_none() {
                        span class="tile-description" { (self.description().using(cx)) }
                    }
                }
            }
        })
    }
}

#[builder_impl]
impl Tile {
    /// Crea un recuadro con el título indicado que lleva a la ruta dada.
    pub fn link(title: Lc, route: impl Into<Route>) -> Self {
        Self {
            route: route.into(),
            title,
            ..Default::default()
        }
    }

    /// Crea un recuadro con el título indicado que abre la ruta dada en una nueva ventana o
    /// pestaña.
    pub fn link_blank(title: Lc, route: impl Into<Route>) -> Self {
        Self {
            blank: true,
            ..Self::link(title, route)
        }
    }

    // **< Tile BUILDER >***************************************************************************

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

    /// Establece la ruta a la que lleva el recuadro.
    pub fn with_route(mut self, route: impl Into<Route>) -> Self {
        self.route = route.into();
        self
    }

    /// Establece el título del recuadro.
    pub fn with_title(mut self, title: Lc) -> Self {
        self.title = title;
        self
    }

    /// Establece la descripción breve que se muestra bajo el título; [`Lc::none()`] la quita.
    pub fn with_description(mut self, description: Lc) -> Self {
        self.description = description;
        self
    }

    /// Establece el icono que se muestra sobre el título, o lo quita con `None`.
    pub fn with_icon(mut self, icon: impl Into<Option<Icon>>) -> Self {
        self.icon.alter_component(icon);
        self
    }

    /// Establece la imagen de fondo del recuadro, o la quita con `None`.
    ///
    /// Admite directamente un [`image::Source`](crate::base::component::image::Source). La imagen
    /// es decorativa: se renderiza sin texto alternativo, aunque la [`Image`] lo tenga, y el tema
    /// la recorta para cubrir el recuadro. Sin imagen, el fondo es sólo el color de acento.
    pub fn with_image(mut self, image: impl Into<Option<Image>>) -> Self {
        self.image.alter_component(image);
        self
    }

    /// Establece el color de acento del recuadro.
    pub fn with_accent(mut self, accent: Accent) -> Self {
        self.accent = accent;
        self
    }

    /// Establece si el recuadro está deshabilitado: se muestra atenuado y sin enlace.
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
