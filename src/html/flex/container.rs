use crate::core::component::Context;
use crate::core::theme::{Breakpoint, Responsive};
use crate::html::flex::{Align, AlignContent, Behavior, ContentJustify, Direction, Gap};
use crate::{AutoDefault, Getters, builder_impl, util};

// **< DisplayFlex >********************************************************************************

// Modo de activación del posicionamiento Flexbox de un contenedor `Flex`. Detalle interno de
// implementación: la API pública sólo expone los constructores `Flex::new()`, `Flex::at()`,
// `Flex::inline()` e `Flex::inline_at()`, nunca esta variante directamente.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
enum DisplayFlex {
    #[default]
    Always,
    AlwaysInline,
    At(Breakpoint),
    InlineAt(Breakpoint),
}

// **< Flex >***************************************************************************************

/// Configuración para el posicionamiento Flexbox en un contenedor.
///
/// Se resuelve como clases CSS generadas dinámicamente (`display`, `flex-direction`, `flex-wrap`,
/// `justify-content`, `align-items`, `align-content`, `gap`), registradas vía
/// [`AssetsOp::AddResponsiveStyle`] en [`ResponsiveStyles`] y renderizadas como reglas en el
/// `<head>` del documento. Son propiedades nativas que no requieren interpretación por parte de los
/// temas, siempre funcionan igual, sin una sola línea de CSS ni de código específico.
///
/// El nombre de cada clase se deriva de la propiedad y el valor que representa (por ejemplo
/// `_flex-direction_row_`), así que dos contenedores con la misma configuración comparten la misma
/// regla generada en vez de duplicarla, y el nombre generado no coincide por accidente con clases
/// de terceros.
///
/// [`AssetsOp::AddResponsiveStyle`]: crate::core::component::AssetsOp::AddResponsiveStyle
/// [`ResponsiveStyles`]: crate::html::ResponsiveStyles
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let actions = Container::new()
///     .with_flex(
///         Flex::new()
///             .with_justify(flex::ContentJustify::End)
///             .with_align(flex::Align::Center)
///             .with_gap(flex::Gap::Both(UnitValue::RelRem(0.5))),
///     )
///     .with_child(Button::submit(Lc::n("Save")))
///     .with_child(Button::plain(Lc::n("Cancel")));
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq, Getters)]
pub struct Flex {
    // Determina si esta configuración debe aplicarse (y con qué variante de `display`) o si
    // `Flex` no está en absoluto configurado. `None` es el estado real de ausencia: lo que tiene
    // un contenedor que nunca ha llamado a `with_flex()`. Sin getter público; `new()`, `at()`,
    // `inline()` e `inline_at()` son la única forma de activarlo.
    #[getters(skip)]
    display: Option<DisplayFlex>,
    /// Devuelve la dirección del eje principal por punto de corte.
    #[getters(copy)]
    direction: Responsive<Direction>,
    /// Devuelve el comportamiento cuando los elementos no caben en una sola línea, por punto de
    /// corte.
    #[getters(copy)]
    wrap: Responsive<Behavior>,
    /// Devuelve la alineación de los elementos en el eje principal, por punto de corte.
    #[getters(copy)]
    justify: Responsive<ContentJustify>,
    /// Devuelve la alineación de los elementos en el eje transversal, por punto de corte.
    #[getters(copy)]
    align: Responsive<Align>,
    /// Devuelve la alineación de las líneas cuando hay más de una, por punto de corte.
    #[getters(copy)]
    align_content: Responsive<AlignContent>,
    /// Devuelve el espaciado entre elementos, por punto de corte.
    #[getters(copy)]
    gap: Responsive<Gap>,
}

#[builder_impl]
impl Flex {
    /// Define una configuración Flex con `display: flex`, sin punto de corte: se aplica siempre.
    pub fn new() -> Self {
        Self {
            display: Some(DisplayFlex::Always),
            ..Default::default()
        }
    }

    /// Define una configuración Flex con `display: flex` que se aplica a partir del punto de corte
    /// indicado.
    pub fn at(bp: Breakpoint) -> Self {
        Self {
            display: Some(DisplayFlex::At(bp)),
            ..Default::default()
        }
    }

    /// Define una configuración Flex con `display: inline-flex`, sin punto de corte: se aplica
    /// siempre.
    pub fn inline() -> Self {
        Self {
            display: Some(DisplayFlex::AlwaysInline),
            ..Default::default()
        }
    }

    /// Define una configuración Flex con `display: inline-flex` que se aplica a partir del punto de
    /// corte indicado.
    pub fn inline_at(bp: Breakpoint) -> Self {
        Self {
            display: Some(DisplayFlex::InlineAt(bp)),
            ..Default::default()
        }
    }

    // **< Flex BUILDER >***************************************************************************

    /// Establece la dirección del eje principal.
    pub fn with_direction(mut self, dir: Direction) -> Self {
        self.direction = self.direction.set(dir);
        self
    }

    /// Establece la dirección del eje principal a partir del punto de corte indicado.
    pub fn with_direction_at(mut self, bp: Breakpoint, dir: Direction) -> Self {
        self.direction = self.direction.set_at(bp, dir);
        self
    }

    /// Establece el comportamiento cuando los elementos no caben en una sola línea.
    pub fn with_wrap(mut self, wrap: Behavior) -> Self {
        self.wrap = self.wrap.set(wrap);
        self
    }

    /// Establece el comportamiento cuando los elementos no caben en una sola línea, a partir del
    /// punto de corte indicado.
    pub fn with_wrap_at(mut self, bp: Breakpoint, wrap: Behavior) -> Self {
        self.wrap = self.wrap.set_at(bp, wrap);
        self
    }

    /// Establece la alineación de los elementos en el eje principal.
    pub fn with_justify(mut self, justify: ContentJustify) -> Self {
        self.justify = self.justify.set(justify);
        self
    }

    /// Establece la alineación de los elementos en el eje principal, a partir del punto de corte
    /// indicado.
    pub fn with_justify_at(mut self, bp: Breakpoint, justify: ContentJustify) -> Self {
        self.justify = self.justify.set_at(bp, justify);
        self
    }

    /// Establece la alineación de los elementos en el eje transversal.
    pub fn with_align(mut self, align: Align) -> Self {
        self.align = self.align.set(align);
        self
    }

    /// Establece la alineación de los elementos en el eje transversal, a partir del punto de corte
    /// indicado.
    pub fn with_align_at(mut self, bp: Breakpoint, align: Align) -> Self {
        self.align = self.align.set_at(bp, align);
        self
    }

    /// Establece la alineación de las líneas cuando hay más de una (ver [`AlignContent`]).
    pub fn with_align_content(mut self, align_content: AlignContent) -> Self {
        self.align_content = self.align_content.set(align_content);
        self
    }

    /// Establece la alineación de las líneas cuando hay más de una (ver [`AlignContent`]), a partir
    /// del punto de corte indicado.
    pub fn with_align_content_at(mut self, bp: Breakpoint, align_content: AlignContent) -> Self {
        self.align_content = self.align_content.set_at(bp, align_content);
        self
    }

    /// Establece el espaciado entre elementos.
    pub fn with_gap(mut self, gap: Gap) -> Self {
        self.gap = self.gap.set(gap);
        self
    }

    /// Establece el espaciado entre elementos, a partir del punto de corte indicado.
    pub fn with_gap_at(mut self, bp: Breakpoint, gap: Gap) -> Self {
        self.gap = self.gap.set_at(bp, gap);
        self
    }
}

impl Flex {
    /// Combina esta configuración con otra `Flex`, campo a campo, o la resetea a los valores por
    /// defecto si se pasa `None`.
    ///
    /// Cada campo de `flex` que tenga un valor sustituye al correspondiente de `self`; los que
    /// estén a `None` dejan intacto el valor ya presente en `self`. Así, sucesivas llamadas pueden
    /// ir completando o sobrescribiendo campos concretos sin necesidad de repetir los ya
    /// establecidos. Es el método recomendado para que un contenedor propio adopte `Flex` de forma
    /// incremental (ver [`Container::with_flex()`](crate::base::component::Container::with_flex)
    /// como referencia de uso).
    pub fn merge(mut self, flex: impl Into<Option<Flex>>) -> Self {
        let Some(flex) = flex.into() else {
            return Flex::default();
        };
        self.display = flex.display.or(self.display);
        self.direction = self.direction.merge(flex.direction);
        self.wrap = self.wrap.merge(flex.wrap);
        self.justify = self.justify.merge(flex.justify);
        self.align = self.align.merge(flex.align);
        self.align_content = self.align_content.merge(flex.align_content);
        self.gap = self.gap.merge(flex.gap);
        self
    }

    /// Aplica esta configuración a un [`Props`] como declaraciones de estilo en línea.
    ///
    /// Es el método recomendado para que un componente adopte `Flex`: concentra en un único sitio
    /// la traducción de la configuración a estilos, para no repetirla en cada componente que la
    /// use. Precedente: [`Container`](crate::base::component::Container) lo aplica sobre su
    /// propio `Props`; [`Navbar`](crate::base::component::Navbar), sobre el `Props` de su área de
    /// contenido.
    ///
    /// Las clases generadas se añaden a `classes`, separadas con un espacio de las que ya hubiera,
    /// para poder compartir un único acumulador con [`FlexItem::apply()`](super::FlexItem::apply)
    /// sin cadenas intermedias.
    #[rustfmt::skip]
    pub(crate) fn apply(self, cx: &mut Context, classes: &mut String) {
        // Sin `display` no hay contenedor Flex: el resto de facetas (`flex-direction`, `gap`...)
        // no tienen ningún efecto en CSS sin `display: flex`/`inline-flex`, así que ni se generan.
        let Some(display) = self.display else {
            return;
        };

        use super::{apply, responsive_class, styles, value_to_token};

        let (prefix, value) = match display {
            DisplayFlex::Always
                | DisplayFlex::At(_) => ("_flex_", "flex"),
            DisplayFlex::AlwaysInline
                | DisplayFlex::InlineAt(_) => ("_inline-flex_", "inline-flex"),
        };
        let entry = match display {
            DisplayFlex::At(bp) | DisplayFlex::InlineAt(bp) => bp.resolved(cx),
            _ => None,
        };
        let class = match entry {
            None => prefix.into(),
            Some(entry) => util::join!(prefix, entry.name, "_").into(),
        };
        styles(cx, classes, entry, class, "display", value.into());

        apply!(cx, classes, self.direction, "_flex-direction_", "flex-direction");
        apply!(cx, classes, self.wrap, "_flex-wrap_", "flex-wrap");
        apply!(cx, classes, self.justify, "_flex-justify_", "justify-content");
        apply!(cx, classes, self.align, "_flex-align-items_", "align-items");
        apply!(cx, classes, self.align_content, "_flex-align-content_", "align-content");
        for (bp, gap) in self.gap.by_breakpoint() {
            let entry = bp.resolved(cx);
            for (property, value) in gap.styles().into_iter().flatten() {
                // El prefijo de `gap` no es literal (depende de la propiedad), así que se compone
                // aquí en un único `join!` en vez de pasar por `responsive_class!`.
                let token = value_to_token(&value);
                let class = match entry {
                    None => util::join!("_flex-", property, "_", token, "_"),
                    Some(e) => util::join!("_flex-", property, "_", token, "_", e.name, "_"),
                };
                styles(cx, classes, entry, class.into(), property, value);
            }
        }
    }
}
