use pagetop::prelude::*;

use crate::prelude::*;
use crate::LOCALES_BOOTSIER;

use std::fmt;

// **< OffcanvasPlacement >*************************************************************************

/// Posición de aparición del panel **deslizante** ([`Offcanvas`]).
///
/// Define desde qué borde de la ventana entra y se ancla el panel.
#[derive(AutoDefault)]
pub enum OffcanvasPlacement {
    /// Opción por defecto, desde el borde inicial según dirección de lectura (respetando LTR/RTL).
    #[default]
    Start,
    /// Desde el borde final según dirección de lectura (respetando LTR/RTL).
    End,
    /// Desde la parte superior.
    Top,
    /// Desde la parte inferior.
    Bottom,
}

#[rustfmt::skip]
impl fmt::Display for OffcanvasPlacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OffcanvasPlacement::Start  => f.write_str("offcanvas-start"),
            OffcanvasPlacement::End    => f.write_str("offcanvas-end"),
            OffcanvasPlacement::Top    => f.write_str("offcanvas-top"),
            OffcanvasPlacement::Bottom => f.write_str("offcanvas-bottom"),
        }
    }
}

// **< OffcanvasVisibility >************************************************************************

/// Estado inicial del panel ([`Offcanvas`]).
#[derive(AutoDefault)]
pub enum OffcanvasVisibility {
    /// El panel **permanece oculto** desde el principio.
    #[default]
    Default,
    /// El panel **se muestra abierto** al cargar.
    Show,
}

impl fmt::Display for OffcanvasVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OffcanvasVisibility::Default => Ok(()),
            OffcanvasVisibility::Show => f.write_str("show"),
        }
    }
}

// **< OffcanvasBodyScroll >************************************************************************

/// Controla si la página principal puede **desplazarse** al abrir el panel ([`Offcanvas`]).
#[derive(AutoDefault)]
pub enum OffcanvasBodyScroll {
    /// Opción por defecto, la página principal se **bloquea** centrando la interacción en el panel.
    #[default]
    Disabled,
    /// **Permite** el desplazamiento de la página principal.
    Enabled,
}

// **< OffcanvasBackdrop >**************************************************************************

/// Comportamiento de la **capa de fondo** (*backdrop*) del panel ([`Offcanvas`]) al desplegarse.
#[derive(AutoDefault)]
pub enum OffcanvasBackdrop {
    /// **Sin capa** de fondo; la página principal permanece visible e interactiva.
    Disabled,
    /// Opción por defecto, se **oscurece** el fondo; un clic fuera del panel suele cerrarlo.
    #[default]
    Enabled,
    /// Se muestra capa de fondo pero **no** se cierra al pulsar fuera (útil cuando se requiere
    /// completar una acción antes de salir).
    Static,
}

// **< Offcanvas >**********************************************************************************

/// Panel lateral **deslizante** para contenido complementario.
///
/// Útil para navegación, filtros, formularios o menús contextuales. Incluye las siguientes
/// características principales:
///
/// - **Entrada configurable desde un borde** de la ventana.
/// - **Encabezado con título** y **botón de cierre** integrado.
/// - **Accesibilidad**: asocia título y controles a un identificador único y expone atributos
///   adecuados para lectores de pantalla y navegación por teclado.
/// - **Opcionalmente** bloquea el desplazamiento del documento y/o muestra una capa de fondo para
///   centrar la atención del usuario.
/// - **Responsive**: puede cambiar su comportamiento según el punto de ruptura indicado.
/// - **No se renderiza** si no tiene contenido.
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Offcanvas {
    id        : AttrId,
    classes   : AttrClasses,
    title     : L10n,
    breakpoint: BreakPoint,
    placement : OffcanvasPlacement,
    visibility: OffcanvasVisibility,
    scrolling : OffcanvasBodyScroll,
    backdrop  : OffcanvasBackdrop,
    children  : Children,
}

impl Component for Offcanvas {
    fn new() -> Self {
        Offcanvas::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            [
                self.breakpoint().to_class("offcanvas"),
                self.placement().to_string(),
                self.visibility().to_string(),
            ]
            .join(" "),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let body = self.children().render(cx);
        if body.is_empty() {
            return PrepareMarkup::None;
        }

        let id = cx.required_id::<Self>(self.id());
        let id_label = join!(id, "-label");
        let id_target = join!("#", id);

        let body_scroll = match self.body_scroll() {
            OffcanvasBodyScroll::Disabled => None,
            OffcanvasBodyScroll::Enabled => Some("true"),
        };

        let backdrop = match self.backdrop() {
            OffcanvasBackdrop::Disabled => Some("false"),
            OffcanvasBackdrop::Enabled => None,
            OffcanvasBackdrop::Static => Some("static"),
        };

        let title = self.title().using(cx);

        PrepareMarkup::With(html! {
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
                        aria-label=[L10n::t("close", &LOCALES_BOOTSIER).lookup(cx)]
                    {}
                }
                div class="offcanvas-body" {
                    (body)
                }
            }
        })
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

    /// Establece el **título** del encabezado.
    #[builder_fn]
    pub fn with_title(mut self, title: L10n) -> Self {
        self.title = title;
        self
    }

    /// Configura el **punto de ruptura** para activar el comportamiento responsive del panel.
    #[builder_fn]
    pub fn with_breakpoint(mut self, bp: BreakPoint) -> Self {
        self.breakpoint = bp;
        self
    }

    /// Indica la **posición** desde la que entra el panel.
    #[builder_fn]
    pub fn with_placement(mut self, placement: OffcanvasPlacement) -> Self {
        self.placement = placement;
        self
    }

    /// Fija el **estado inicial** del panel (oculto o visible al cargar).
    #[builder_fn]
    pub fn with_visibility(mut self, visibility: OffcanvasVisibility) -> Self {
        self.visibility = visibility;
        self
    }

    /// Permite o bloquea el **desplazamiento** de la página principal mientras el panel está
    /// abierto.
    #[builder_fn]
    pub fn with_body_scroll(mut self, scrolling: OffcanvasBodyScroll) -> Self {
        self.scrolling = scrolling;
        self
    }

    /// Ajusta la **capa de fondo** del panel para definir su comportamiento al interactuar fuera.
    #[builder_fn]
    pub fn with_backdrop(mut self, backdrop: OffcanvasBackdrop) -> Self {
        self.backdrop = backdrop;
        self
    }

    /// Añade un nuevo componente hijo al panel.
    pub fn add_child(mut self, child: impl Component) -> Self {
        self.children.add(Child::with(child));
        self
    }

    /// Modifica la lista de hijos (`children`) aplicando una operación [`ChildOp`].
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

    /// Devuelve el título del panel como [`L10n`].
    pub fn title(&self) -> &L10n {
        &self.title
    }

    /// Devuelve el punto de ruptura configurado.
    pub fn breakpoint(&self) -> &BreakPoint {
        &self.breakpoint
    }

    /// Devuelve la posición del panel.
    pub fn placement(&self) -> &OffcanvasPlacement {
        &self.placement
    }

    /// Devuelve el estado inicial del panel.
    pub fn visibility(&self) -> &OffcanvasVisibility {
        &self.visibility
    }

    /// Indica si la página principal puede desplazarse mientras el panel está abierto.
    pub fn body_scroll(&self) -> &OffcanvasBodyScroll {
        &self.scrolling
    }

    /// Devuelve la configuración de la capa de fondo.
    pub fn backdrop(&self) -> &OffcanvasBackdrop {
        &self.backdrop
    }

    /// Devuelve la lista de hijos (`children`) del panel.
    pub fn children(&self) -> &Children {
        &self.children
    }
}
