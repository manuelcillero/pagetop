use pagetop::prelude::*;

use crate::LOCALES_BOOTSIER;
use crate::theme::*;

/// Componente para crear un **panel lateral deslizante** ([`offcanvas`](crate::theme::bs::offcanvas)).
///
/// Útil para navegación, filtros, formularios o menús contextuales. Incluye las siguientes
/// características principales:
///
/// - Puede mostrar una capa de fondo para centrar la atención del usuario en el panel
///   ([`with_backdrop()`](Self::with_backdrop)); o puede bloquear el desplazamiento del documento
///   principal ([`with_body_scroll()`](Self::with_body_scroll)).
/// - Se puede configurar el borde de la ventana desde el que se desliza el panel
///   ([`with_placement()`](Self::with_placement)).
/// - Encabezado con título ([`with_title()`](Self::with_title)) y **botón de cierre** integrado.
/// - Puede cambiar su comportamiento a partir de un punto de ruptura
///   ([`with_breakpoint()`](Self::with_breakpoint)).
/// - Asocia título y controles de accesibilidad a un identificador único y expone atributos
///   adecuados para lectores de pantalla y navegación por teclado.
///
/// Si no contiene elementos, el componente **no se renderiza**.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
/// use pagetop_bootsier::theme::*;
///
/// let panel = bs::Offcanvas::new()
///     .with_id("offcanvas_example")
///     .with_title(L10n::n("Offcanvas title"))
///     .with_placement(bs::offcanvas::Placement::End)
///     .with_backdrop(bs::offcanvas::Backdrop::Enabled)
///     .with_body_scroll(bs::offcanvas::BodyScroll::Enabled)
///     .with_visibility(bs::offcanvas::Visibility::Default)
///     .with_child(bs::Dropdown::new()
///         .with_title(L10n::n("Menu"))
///         .with_item(bs::dropdown::Item::label(L10n::n("Label")))
///         .with_item(bs::dropdown::Item::link_blank(L10n::n("Doc"), |_| "https://docs.rs".into()))
///         .with_item(bs::dropdown::Item::link(L10n::n("Sign out"), |_| "/signout".into()))
///     );
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Offcanvas {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el título del panel.
    title: L10n,
    /// Devuelve el punto de ruptura configurado para cambiar el comportamiento del panel.
    breakpoint: token::BreakPoint,
    /// Devuelve el comportamiento configurado para la capa de fondo.
    backdrop: bs::offcanvas::Backdrop,
    /// Indica si la página principal puede desplazarse mientras el panel está abierto.
    body_scroll: bs::offcanvas::BodyScroll,
    /// Devuelve la posición de inicio del panel.
    placement: bs::offcanvas::Placement,
    /// Devuelve el estado inicial del panel.
    visibility: bs::offcanvas::Visibility,
    /// Devuelve la lista de componentes (`children`) del panel.
    children: Children,
}

#[async_trait]
impl Component for Offcanvas {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, cx: &Context) {
        // Asegura que el panel tiene un identificador único.
        self.alter_prop(PropsOp::ensure_id(cx.build_id::<Self>(1)));

        // Clases CSS por defecto para el panel.
        self.alter_prop(PropsOp::prepend_classes({
            let mut classes = "offcanvas".to_string();
            self.breakpoint().push_to(&mut classes, "offcanvas", "");
            self.placement().push_to(&mut classes);
            self.visibility().push_to(&mut classes);
            classes
        }));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(self.render_offcanvas(cx, None).await)
    }
}

impl Offcanvas {
    // **< Offcanvas BUILDER >**********************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece el título del encabezado.
    #[builder_fn]
    pub fn with_title(mut self, title: L10n) -> Self {
        self.title = title;
        self
    }

    /// Establece el punto de ruptura a partir del cual cambia el comportamiento del panel.
    ///
    /// - **Por debajo** de ese tamaño de pantalla, el componente actúa como panel deslizante
    ///   ([`Offcanvas`]).
    /// - **Por encima**, el contenido del panel se muestra tal cual, integrado en la página.
    ///
    /// Por ejemplo, con `BreakPoint::Lg`, será *offcanvas* en móviles y tabletas, y visible
    /// directamente en pantallas grandes. Por defecto usa `BreakPoint::None` para que sea
    /// *offcanvas* siempre.
    #[builder_fn]
    pub fn with_breakpoint(mut self, bp: token::BreakPoint) -> Self {
        self.breakpoint = bp;
        self
    }

    /// Ajusta la capa de fondo del panel para definir su comportamiento al hacer clic fuera del
    /// panel.
    #[builder_fn]
    pub fn with_backdrop(mut self, backdrop: bs::offcanvas::Backdrop) -> Self {
        self.backdrop = backdrop;
        self
    }

    /// Permite o bloquea el desplazamiento de la página principal mientras el panel está abierto.
    #[builder_fn]
    pub fn with_body_scroll(mut self, scrolling: bs::offcanvas::BodyScroll) -> Self {
        self.body_scroll = scrolling;
        self
    }

    /// Indica desde qué borde de la ventana entra y se ancla el panel.
    #[builder_fn]
    pub fn with_placement(mut self, placement: bs::offcanvas::Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Fija el estado inicial del panel (oculto o visible al cargar).
    #[builder_fn]
    pub fn with_visibility(mut self, visibility: bs::offcanvas::Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    /// Añade un nuevo componente al panel o modifica la lista de componentes (`children`) con una
    /// operación [`ChildOp`].
    #[builder_fn]
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.children.alter_child(op.into());
        self
    }

    // **< Offcanvas HELPERS >**********************************************************************

    pub(crate) async fn render_offcanvas(
        &self,
        cx: &mut Context,
        extra: Option<&Children>,
    ) -> Markup {
        let body = self.children().render(cx).await;
        let body_extra = if let Some(c) = extra {
            c.render(cx).await
        } else {
            html! {}
        };
        if body.is_empty() && body_extra.is_empty() {
            return html! {};
        }

        // `setup()` garantiza que habrá un `id` antes de renderizar.
        let id = self.id().unwrap();
        let id_label = util::join!(id, "-label");
        let id_target = util::join!("#", id);

        let body_scroll = self.body_scroll().opt_str();
        let backdrop = self.backdrop().opt_str();

        let title = self.title().using(cx);

        html! {
            div
                (self.props())
                tabindex="-1"
                data-bs-scroll=[body_scroll]
                data-bs-backdrop=[backdrop]
                aria-labelledby=(id_label)
            {
                div class="offcanvas-header" {
                    @if !title.is_empty() {
                        h5 id=(&id_label) class="offcanvas-title" { (title) }
                    }
                    button
                        type="button"
                        class="btn-close"
                        data-bs-dismiss="offcanvas"
                        data-bs-target=(id_target)
                        aria-label=[L10n::t("offcanvas_close", &LOCALES_BOOTSIER).lookup(cx)]
                    {}
                }
                div class="offcanvas-body" {
                    (body)
                    (body_extra)
                }
            }
        }
    }
}
