use crate::html::flex::props_container::{
    Align, AlignContent, Behavior, ContentJustify, Direction, Gap,
};
use crate::html::props::{Props, PropsOp};
use crate::{AutoDefault, Getters, builder_impl};

// **< Flex >***************************************************************************************

/// Configuración para el posicionamiento Flexbox en un contenedor.
///
/// Se resuelve con estilos en línea (`display`, `flex-direction`, `flex-wrap`, `justify-content`,
/// `align-items`, `align-content`, `gap`), nunca como clases CSS. Son propiedades estándar que no
/// requieren interpretación por parte de los temas, siempre funcionan igual, sin una sola
/// línea de CSS ni de código específico.
///
/// Esto tiene además una consecuencia práctica; los estilos en línea tienen la especificidad más
/// alta que existe en CSS, salvo `!important`. Ningún *framework* CSS de terceros, ni el CSS de la
/// propia aplicación, puede sobrescribir por accidente lo que `Flex` aplica. Es un mecanismo
/// autosuficiente que funciona igual conviva con quien conviva en la misma página, sin coordinar
/// nombres de clase ni orden de carga de hojas de estilo con nadie.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let actions = Container::new()
///     .with_flex(
///         Flex::row()
///             .with_justify(flex::ContentJustify::End)
///             .with_align(flex::Align::Center)
///             .with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))),
///     )
///     .with_child(Button::submit(Lc::n("Save")))
///     .with_child(Button::plain(Lc::n("Cancel")));
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq, Getters)]
pub struct Flex {
    /// Devuelve la dirección del eje principal.
    #[getters(copy)]
    direction: Direction,
    /// Devuelve el comportamiento cuando los elementos no caben en una sola línea.
    #[getters(copy)]
    wrap: Behavior,
    /// Devuelve la alineación de los elementos en el eje principal.
    #[getters(copy)]
    justify: ContentJustify,
    /// Devuelve la alineación de los elementos en el eje transversal.
    #[getters(copy)]
    align: Align,
    /// Devuelve la alineación de las líneas cuando hay más de una.
    #[getters(copy)]
    align_content: AlignContent,
    /// Devuelve el espaciado entre elementos.
    #[getters(copy)]
    gap: Gap,
}

#[builder_impl]
impl Flex {
    /// Crea una configuración Flex para disponer los elementos en fila (comportamiento por
    /// defecto).
    pub fn row() -> Self {
        Self::default()
    }

    /// Crea una configuración Flex para disponer los elementos en columna.
    pub fn column() -> Self {
        Self {
            direction: Direction::Column,
            ..Default::default()
        }
    }

    // **< Flex BUILDER >***************************************************************************

    /// Establece la dirección del eje principal.
    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Establece el comportamiento cuando los elementos no caben en una sola línea.
    pub fn with_wrap(mut self, wrap: Behavior) -> Self {
        self.wrap = wrap;
        self
    }

    /// Establece la alineación de los elementos en el eje principal.
    pub fn with_justify(mut self, justify: ContentJustify) -> Self {
        self.justify = justify;
        self
    }

    /// Establece la alineación de los elementos en el eje transversal.
    pub fn with_align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Establece la alineación de las líneas cuando hay más de una (ver [`AlignContent`]).
    pub fn with_align_content(mut self, align_content: AlignContent) -> Self {
        self.align_content = align_content;
        self
    }

    /// Establece el espaciado entre elementos.
    pub fn with_gap(mut self, gap: Gap) -> Self {
        self.gap = gap;
        self
    }
}

impl Flex {
    /// Aplica esta configuración a un [`Props`] como declaraciones de estilo en línea.
    ///
    /// Es el método recomendado para que un componente adopte `Flex`: concentra en un único sitio
    /// la traducción de la configuración a estilos, para no repetirla en cada componente que la
    /// use. Precedente: [`Container`](crate::base::component::Container) lo aplica sobre su
    /// propio `Props`; [`Navbar`](crate::base::component::Navbar), sobre el `Props` de su área de
    /// contenido.
    pub fn apply_to(self, props: &mut Props) {
        props.alter_prop(PropsOp::add_style("display", "flex"));
        for (property, value) in [
            ("flex-direction", self.direction.value()),
            ("flex-wrap", self.wrap.value()),
            ("justify-content", self.justify.value()),
            ("align-items", self.align.value()),
            ("align-content", self.align_content.value()),
        ] {
            props.alter_prop(PropsOp::add_style(property, value));
        }
        for (property, value) in self.gap.styles() {
            props.alter_prop(PropsOp::add_style(property, value));
        }
    }
}
