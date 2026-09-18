use crate::html::align;
use crate::html::flex::{Behavior, ContentJustify, Direction};
use crate::prelude::*;

// **< DisplayFlex >********************************************************************************

// Posicionamiento Flexbox del contenedor `Flex`. La API pública sólo expone los constructores
// `Flex::new()`, `Flex::at()`, `Flex::inline()` y `Flex::inline_at()`, nunca la variante en sí.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
enum DisplayFlex {
    #[default]
    Always,
    AlwaysInline,
    At(Breakpoint),
    InlineAt(Breakpoint),
}

// **< Flex >***************************************************************************************

/// Componente que crea un **contenedor Flexbox** para posicionar componentes.
///
/// Es el único componente de PageTop que ofrece posicionamiento Flexbox. Sus hijos pueden ser
/// **cualquier componente**, no sólo otro `Flex`. Para colocarlos dentro del contenedor
/// (crecimiento, reducción, alineación individual, orden, tamaño, desplazamiento) se usa
/// [`flex::FlexItem`] vía `with_prop()`, exactamente igual que en cualquier otro contenedor.
/// `FlexItem` es una característica de [`PropsOp`], no exige ningún envoltorio propio.
///
/// Se aplica con clases CSS generadas dinámicamente (`display`, `flex-direction`, `flex-wrap`,
/// `justify-content`, `align-items`, `align-content`, `gap`, `row-gap`, `column-gap`), registradas
/// vía [`AssetsOp::add_responsive_style()`] en [`ResponsiveStyles`] y renderizadas como reglas en
/// el `<head>` del documento. Son propiedades nativas que no requieren interpretación por parte de
/// los temas, siempre funcionan igual, sin una sola línea de CSS ni de código específico.
///
/// `align`/`align_content`/`gap` usan tipos de [`pagetop::html::align`] ([`align::Items`],
/// [`align::Content`], [`align::Gap`]), compartidos con [`Grid`], mismo CSS y mismo catálogo de
/// valores en los dos. Ver la documentación de [`align`](crate::html::align) para el criterio
/// completo.
///
/// Si no contiene elementos, el componente **no se renderiza**.
///
/// [`AssetsOp::add_responsive_style()`]: crate::core::component::AssetsOp::add_responsive_style
/// [`pagetop::html::align`]: crate::html::align
/// [`ResponsiveStyles`]: crate::html::ResponsiveStyles
/// [`flex::FlexItem`]: crate::html::flex::FlexItem
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let actions = Flex::new()
///     .with_justify(flex::ContentJustify::End)
///     .with_align(align::Items::Center)
///     .with_gap(align::Gap::Both(UnitValue::RelRem(0.5)))
///     .with_child(Button::submit(Lc::n("Save")))
///     .with_child(Button::plain(Lc::n("Cancel")));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Flex {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    // Determina si esta configuración debe aplicarse (y con qué variante de `display`) o si `Flex`
    // no está en absoluto configurado. `None` es el estado real de ausencia: lo que tiene un
    // componente que nunca ha llamado a ninguno de sus constructores explícitos. Sin getter
    // público; `new()`, `at()`, `inline()` e `inline_at()` son la única forma de activarlo.
    #[getters(skip)]
    display: Option<DisplayFlex>,
    /// Devuelve la dirección del eje principal, por punto de corte.
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
    align: Responsive<align::Items>,
    /// Devuelve la alineación de las líneas cuando hay más de una, por punto de corte.
    #[getters(copy)]
    align_content: Responsive<align::Content>,
    /// Devuelve el espaciado entre elementos, por punto de corte.
    #[getters(copy)]
    gap: Responsive<align::Gap>,
    /// Devuelve la lista de componentes (`children`) del contenedor.
    children: Children,
}

#[async_trait]
impl Component for Flex {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let output = self.children().render(cx).await;
        if output.is_empty() {
            return Ok(html! {});
        }
        let mut classes = String::new();
        self.flex_classes(cx, &mut classes);
        let container_props = self.props().unpack_with_classes(cx, classes);
        Ok(html! { div (container_props) { (output) } })
    }
}

#[builder_impl]
impl Flex {
    /// Define una configuración con `display: flex`, sin punto de corte: se aplica siempre.
    pub fn new() -> Self {
        Self {
            display: Some(DisplayFlex::Always),
            ..Default::default()
        }
    }

    /// Define una configuración con `display: flex` que se aplica a partir del punto de corte
    /// indicado.
    pub fn at(bp: Breakpoint) -> Self {
        Self {
            display: Some(DisplayFlex::At(bp)),
            ..Default::default()
        }
    }

    /// Define una configuración con `display: inline-flex`, sin punto de corte: se aplica
    /// siempre.
    pub fn inline() -> Self {
        Self {
            display: Some(DisplayFlex::AlwaysInline),
            ..Default::default()
        }
    }

    /// Define una configuración con `display: inline-flex` que se aplica a partir del punto de
    /// corte indicado.
    pub fn inline_at(bp: Breakpoint) -> Self {
        Self {
            display: Some(DisplayFlex::InlineAt(bp)),
            ..Default::default()
        }
    }

    // **< Flex BUILDER >***************************************************************************

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
    pub fn with_align(mut self, align: align::Items) -> Self {
        self.align = self.align.set(align);
        self
    }

    /// Establece la alineación de los elementos en el eje transversal, a partir del punto de corte
    /// indicado.
    pub fn with_align_at(mut self, bp: Breakpoint, align: align::Items) -> Self {
        self.align = self.align.set_at(bp, align);
        self
    }

    /// Establece la alineación de las líneas cuando hay más de una (ver
    /// [`Content`](crate::html::align::Content)).
    pub fn with_align_content(mut self, align_content: align::Content) -> Self {
        self.align_content = self.align_content.set(align_content);
        self
    }

    /// Establece la alineación de las líneas cuando hay más de una (ver
    /// [`Content`](crate::html::align::Content)), a partir del punto de corte indicado.
    pub fn with_align_content_at(mut self, bp: Breakpoint, align_content: align::Content) -> Self {
        self.align_content = self.align_content.set_at(bp, align_content);
        self
    }

    /// Establece el espaciado entre elementos.
    pub fn with_gap(mut self, gap: align::Gap) -> Self {
        self.gap = self.gap.set(gap);
        self
    }

    /// Establece el espaciado entre elementos, a partir del punto de corte indicado.
    pub fn with_gap_at(mut self, bp: Breakpoint, gap: align::Gap) -> Self {
        self.gap = self.gap.set_at(bp, gap);
        self
    }

    /// Añade un nuevo componente al contenedor o modifica la lista de componentes (`children`) con
    /// una operación [`ChildOp`].
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.children.alter_child(op.into());
        self
    }
}

impl Flex {
    // Calcula las clases CSS *responsive* de este contenedor Flex y las añade a `classes`,
    // separadas con un espacio de las que ya hubiera, para poder compartir un único acumulador
    // con `FlexItem::apply()` (ver `Props::unpack_with_classes()`) sin cadenas intermedias.
    #[rustfmt::skip]
    fn flex_classes(&self, cx: &mut Context, classes: &mut String) {
        // Sin `display` no hay contenedor Flex: el resto de propiedades (`flex-direction`, `gap`,
        // etc.) no tienen ningún efecto en CSS sin `display: flex`/`inline-flex`, así que ni se
        // generan.
        let Some(display) = self.display else {
            return;
        };

        use crate::html::responsive::{apply, responsive_class, styles, value_to_token};

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
