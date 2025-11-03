use pagetop::prelude::*;

use crate::prelude::*;

// **< ContainerType >******************************************************************************

/// Tipo de contenedor ([`Container`]).
///
/// Permite aplicar la etiqueta HTML apropiada (`<main>`, `<header>`, etc.) manteniendo una API
/// común a todos los contenedores.
#[derive(AutoDefault)]
pub enum ContainerType {
    /// Contenedor genérico (`<div>`).
    #[default]
    Default,
    /// Contenido principal de la página (`<main>`).
    Main,
    /// Encabezado de la página o de sección (`<header>`).
    Header,
    /// Pie de la página o de sección (`<footer>`).
    Footer,
    /// Sección de contenido (`<section>`).
    Section,
    /// Artículo de contenido (`<article>`).
    Article,
}

// **< ContainerWidth >*****************************************************************************

/// Define el comportamiento para ajustar el ancho de un contenedor ([`Container`]).
#[derive(AutoDefault)]
pub enum ContainerWidth {
    /// Comportamiento por defecto, aplica los anchos máximos predefinidos para cada punto de
    /// ruptura. Por debajo del menor punto de ruptura ocupa el 100% del ancho disponible.
    #[default]
    Default,
    /// Aplica los anchos máximos predefinidos a partir del punto de ruptura indicado. Por debajo de
    /// ese punto de ruptura ocupa el 100% del ancho disponible.
    From(BreakPoint),
    /// Ocupa el 100% del ancho disponible siempre.
    Fluid,
    /// Ocupa el 100% del ancho disponible hasta un ancho máximo explícito.
    FluidMax(UnitValue),
}

// **< Container >**********************************************************************************

/// Componente para crear un **contenedor de componentes**.
///
/// Envuelve el contenido con la etiqueta HTML indicada por [`ContainerType`]. Sólo se renderiza si
/// existen componentes hijos (*children*).
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Container {
    id             : AttrId,
    classes        : AttrClasses,
    container_type : ContainerType,
    container_width: ContainerWidth,
    style_bg       : StyleBg,
    style_text     : StyleText,
    border         : Border,
    rounded        : Rounded,
    children       : Children,
}

impl Component for Container {
    fn new() -> Self {
        Container::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            [
                join_pair!(
                    "container",
                    "-",
                    match self.width() {
                        ContainerWidth::Default => String::new(),
                        ContainerWidth::From(bp) => bp.to_string(),
                        ContainerWidth::Fluid => "fluid".to_string(),
                        ContainerWidth::FluidMax(_) => "fluid".to_string(),
                    }
                ),
                self.style_bg().to_string(),
                self.style_text().to_string(),
                self.border().to_string(),
                self.rounded().to_string(),
            ]
            .join(" "),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let output = self.children().render(cx);
        if output.is_empty() {
            return PrepareMarkup::None;
        }
        let style = match self.width() {
            ContainerWidth::FluidMax(w) if w.is_measurable() => {
                Some(join!("max-width: ", w.to_string(), ";"))
            }
            _ => None,
        };
        match self.container_type() {
            ContainerType::Default => PrepareMarkup::With(html! {
                div id=[self.id()] class=[self.classes().get()] style=[style] {
                    (output)
                }
            }),
            ContainerType::Main => PrepareMarkup::With(html! {
                main id=[self.id()] class=[self.classes().get()] style=[style] {
                    (output)
                }
            }),
            ContainerType::Header => PrepareMarkup::With(html! {
                header id=[self.id()] class=[self.classes().get()] style=[style] {
                    (output)
                }
            }),
            ContainerType::Footer => PrepareMarkup::With(html! {
                footer id=[self.id()] class=[self.classes().get()] style=[style] {
                    (output)
                }
            }),
            ContainerType::Section => PrepareMarkup::With(html! {
                section id=[self.id()] class=[self.classes().get()] style=[style] {
                    (output)
                }
            }),
            ContainerType::Article => PrepareMarkup::With(html! {
                article id=[self.id()] class=[self.classes().get()] style=[style] {
                    (output)
                }
            }),
        }
    }
}

impl Container {
    /// Crea un contenedor de tipo `Main` (`<main>`).
    pub fn main() -> Self {
        Container {
            container_type: ContainerType::Main,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Header` (`<header>`).
    pub fn header() -> Self {
        Container {
            container_type: ContainerType::Header,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Footer` (`<footer>`).
    pub fn footer() -> Self {
        Container {
            container_type: ContainerType::Footer,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Section` (`<section>`).
    pub fn section() -> Self {
        Container {
            container_type: ContainerType::Section,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Article` (`<article>`).
    pub fn article() -> Self {
        Container {
            container_type: ContainerType::Article,
            ..Default::default()
        }
    }

    // **< Container BUILDER >**********************************************************************

    /// Establece el identificador único (`id`) del contenedor.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_value(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al contenedor.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_value(op, classes);
        self
    }

    /// Establece el comportamiento del ancho para el contenedor.
    #[builder_fn]
    pub fn with_width(mut self, width: ContainerWidth) -> Self {
        self.container_width = width;
        self
    }

    /// Establece el estilo del fondo ([`StyleBg`]).
    #[builder_fn]
    pub fn with_style_bg(mut self, style: StyleBg) -> Self {
        self.style_bg = style;
        self
    }

    /// Establece el estilo del texto ([`StyleText`]).
    #[builder_fn]
    pub fn with_style_text(mut self, style: StyleText) -> Self {
        self.style_text = style;
        self
    }

    /// Atajo para definir los estilos de fondo y texto a la vez.
    #[builder_fn]
    pub fn with_styles(mut self, style_bg: StyleBg, style_text: StyleText) -> Self {
        self.style_bg = style_bg;
        self.style_text = style_text;
        self
    }

    /// Establece el borde del contenedor ([`Border`]).
    #[builder_fn]
    pub fn with_border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    /// Establece esquinas redondeadas para el contenedor ([`Rounded`]).
    #[builder_fn]
    pub fn with_rounded(mut self, rounded: Rounded) -> Self {
        self.rounded = rounded;
        self
    }

    /// Añade un nuevo componente hijo al contenedor.
    #[inline]
    pub fn add_child(mut self, component: impl Component) -> Self {
        self.children.add(Child::with(component));
        self
    }

    /// Modifica la lista de componentes (`children`) aplicando una operación [`ChildOp`].
    #[builder_fn]
    pub fn with_child(mut self, op: ChildOp) -> Self {
        self.children.alter_child(op);
        self
    }

    // **< Container GETTERS >**********************************************************************

    /// Devuelve las clases CSS asociadas al contenedor.
    pub fn classes(&self) -> &AttrClasses {
        &self.classes
    }

    /// Devuelve el tipo semántico del contenedor.
    pub fn container_type(&self) -> &ContainerType {
        &self.container_type
    }

    /// Devuelve el comportamiento para el ancho del contenedor.
    pub fn width(&self) -> &ContainerWidth {
        &self.container_width
    }

    /// Devuelve el estilo del fondo del contenedor.
    pub fn style_bg(&self) -> &StyleBg {
        &self.style_bg
    }

    /// Devuelve el estilo del texto del contenedor.
    pub fn style_text(&self) -> &StyleText {
        &self.style_text
    }

    /// Devuelve el borde configurado del contenedor.
    pub fn border(&self) -> &Border {
        &self.border
    }

    /// Devuelve las esquinas redondeadas configuradas para el contenedor.
    pub fn rounded(&self) -> &Rounded {
        &self.rounded
    }

    /// Devuelve la lista de componentes (`children`) del contenedor.
    pub fn children(&self) -> &Children {
        &self.children
    }
}
