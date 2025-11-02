use pagetop::prelude::*;

use crate::prelude::*;
use crate::LOCALES_BOOTSIER;

/// Componente para crear un **panel lateral deslizante** con contenidos adicionales.
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
/// Ver ejemplo en el módulo [`offcanvas`].
/// Si no contiene elementos, el componente **no se renderiza**.
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Offcanvas {
    id        : AttrId,
    classes   : AttrClasses,
    title     : L10n,
    breakpoint: BreakPoint,
    backdrop  : offcanvas::Backdrop,
    scrolling : offcanvas::BodyScroll,
    placement : offcanvas::Placement,
    visibility: offcanvas::Visibility,
    children  : Children,
}

impl Component for Offcanvas {
    fn new() -> Self {
        Offcanvas::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    #[rustfmt::skip]
    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            [
                self.breakpoint().to_class("offcanvas"),
                match self.placement() {
                    offcanvas::Placement::Start  => "offcanvas-start",
                    offcanvas::Placement::End    => "offcanvas-end",
                    offcanvas::Placement::Top    => "offcanvas-top",
                    offcanvas::Placement::Bottom => "offcanvas-bottom",
                }.to_string(),
                match self.visibility() {
                    offcanvas::Visibility::Default => "",
                    offcanvas::Visibility::Show    => "show",
                }.to_string(),
            ]
            .join(" "),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(self.render_offcanvas(cx, None))
    }
}

impl Offcanvas {
    // **< Offcanvas BUILDER >**********************************************************************

    /// Establece el identificador único (`id`) del panel.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_value(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al panel.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_value(op, classes);
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
    /// Por ejemplo, con `BreakPoint::LG`, será *offcanvas* en móviles y tabletas, y visible
    /// directamente en pantallas grandes. Por defecto usa `BreakPoint::None` para que sea
    /// *offcanvas* siempre.
    #[builder_fn]
    pub fn with_breakpoint(mut self, bp: BreakPoint) -> Self {
        self.breakpoint = bp;
        self
    }

    /// Ajusta la capa de fondo del panel para definir su comportamiento al hacer clic fuera del
    /// panel.
    #[builder_fn]
    pub fn with_backdrop(mut self, backdrop: offcanvas::Backdrop) -> Self {
        self.backdrop = backdrop;
        self
    }

    /// Permite o bloquea el desplazamiento de la página principal mientras el panel está abierto.
    #[builder_fn]
    pub fn with_body_scroll(mut self, scrolling: offcanvas::BodyScroll) -> Self {
        self.scrolling = scrolling;
        self
    }

    /// Indica desde qué borde de la ventana entra y se ancla el panel.
    #[builder_fn]
    pub fn with_placement(mut self, placement: offcanvas::Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Fija el estado inicial del panel (oculto o visible al cargar).
    #[builder_fn]
    pub fn with_visibility(mut self, visibility: offcanvas::Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    /// Añade un nuevo componente hijo al panel.
    #[inline]
    pub fn add_child(mut self, child: impl Component) -> Self {
        self.children.add(Child::with(child));
        self
    }

    /// Modifica la lista de componentes (`children`) aplicando una operación [`ChildOp`].
    #[builder_fn]
    pub fn with_children(mut self, op: ChildOp) -> Self {
        self.children.alter_child(op);
        self
    }

    // **< Offcanvas GETTERS >**********************************************************************

    /// Devuelve las clases CSS asociadas al panel.
    pub fn classes(&self) -> &AttrClasses {
        &self.classes
    }

    /// Devuelve el título del panel.
    pub fn title(&self) -> &L10n {
        &self.title
    }

    /// Devuelve el punto de ruptura configurado para cambiar el comportamiento del panel.
    pub fn breakpoint(&self) -> &BreakPoint {
        &self.breakpoint
    }

    /// Devuelve el comportamiento configurado para la capa de fondo.
    pub fn backdrop(&self) -> &offcanvas::Backdrop {
        &self.backdrop
    }

    /// Indica si la página principal puede desplazarse mientras el panel está abierto.
    pub fn body_scroll(&self) -> &offcanvas::BodyScroll {
        &self.scrolling
    }

    /// Devuelve la posición de inicio del panel.
    pub fn placement(&self) -> &offcanvas::Placement {
        &self.placement
    }

    /// Devuelve el estado inicial del panel.
    pub fn visibility(&self) -> &offcanvas::Visibility {
        &self.visibility
    }

    /// Devuelve la lista de componentes (`children`) del panel.
    pub fn children(&self) -> &Children {
        &self.children
    }

    // **< Offcanvas HELPERS >**********************************************************************

    pub(crate) fn render_offcanvas(&self, cx: &mut Context, extra: Option<&Children>) -> Markup {
        let body = self.children().render(cx);
        let body_extra = extra.map(|c| c.render(cx)).unwrap_or_else(|| html! {});
        if body.is_empty() && body_extra.is_empty() {
            return html! {};
        }

        let id = cx.required_id::<Self>(self.id());
        let id_label = join!(id, "-label");
        let id_target = join!("#", id);

        let body_scroll = match self.body_scroll() {
            offcanvas::BodyScroll::Disabled => None,
            offcanvas::BodyScroll::Enabled => Some("true"),
        };

        let backdrop = match self.backdrop() {
            offcanvas::Backdrop::Disabled => Some("false"),
            offcanvas::Backdrop::Enabled => None,
            offcanvas::Backdrop::Static => Some("static"),
        };

        let title = self.title().using(cx);

        html! {
            div
                id=(id)
                class=[self.classes().get()]
                tabindex="-1"
                data-bs-scroll=[body_scroll]
                data-bs-backdrop=[backdrop]
                aria-labelledby=(id_label)
            {
                div class="offcanvas-header" {
                    @if !title.is_empty() {
                        h5 class="offcanvas-title" id=(id_label) { (title) }
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
