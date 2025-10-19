use pagetop::prelude::*;

use crate::prelude::*;

/// Tipo de contenedor ([`Container`]).
///
/// Permite aplicar la etiqueta HTML apropiada (`<main>`, `<header>`, etc.) manteniendo una API
/// común a todos los contenedores.
#[rustfmt::skip]
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

/// Componente genérico para crear un contenedor de componentes.
///
/// Envuelve el contenido con la etiqueta HTML indicada por [`ContainerType`]. Sólo se renderiza si
/// existen componentes hijos (*children*).
#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Container {
    id            : AttrId,
    classes       : AttrClasses,
    container_type: ContainerType,
    breakpoint    : BreakPoint,
    children      : Children,
    bg_color      : BgColor,
    text_color    : TextColor,
    border        : Border,
    rounded       : Rounded,
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
                join_pair!("container", "-", self.breakpoint().to_string()),
                self.bg_color().to_string(),
                self.text_color().to_string(),
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
        let style = match self.breakpoint() {
            BreakPoint::FluidMax(w) if w.is_measurable() => {
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

    /// Establece el *punto de ruptura* del contenedor.
    #[builder_fn]
    pub fn with_breakpoint(mut self, bp: BreakPoint) -> Self {
        self.breakpoint = bp;
        self
    }

    /// Añade un nuevo componente hijo al contenedor.
    pub fn add_child(mut self, component: impl Component) -> Self {
        self.children
            .alter_child(ChildOp::Add(Child::with(component)));
        self
    }

    /// Modifica la lista de hijos (`children`) aplicando una operación [`ChildOp`].
    #[builder_fn]
    pub fn with_child(mut self, op: ChildOp) -> Self {
        self.children.alter_child(op);
        self
    }

    /// Establece el color de fondo ([`BgColor`]).
    #[builder_fn]
    pub fn with_bg_color(mut self, color: BgColor) -> Self {
        self.bg_color = color;
        self
    }

    /// Establece el color del texto ([`TextColor`]).
    #[builder_fn]
    pub fn with_text_color(mut self, color: TextColor) -> Self {
        self.text_color = color;
        self
    }

    /// Atajo para definir los colores de fondo y texto a la vez.
    #[builder_fn]
    pub fn with_colors(mut self, bg_color: BgColor, text_color: TextColor) -> Self {
        self.bg_color = bg_color;
        self.text_color = text_color;
        self
    }

    /// Establece el borde del contenedor ([`Border`]).
    #[builder_fn]
    pub fn with_border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    /// Establece esquinas redondeadas para el contenedor.
    #[builder_fn]
    pub fn with_rounded(mut self, rounded: Rounded) -> Self {
        self.rounded = rounded;
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

    /// Devuelve el *punto de ruptura* actualmente configurado.
    pub fn breakpoint(&self) -> &BreakPoint {
        &self.breakpoint
    }

    /// Devuelve la lista de hijos (`children`) del contenedor.
    pub fn children(&self) -> &Children {
        &self.children
    }

    /// Devuelve el color de fondo del contenedor.
    pub fn bg_color(&self) -> &BgColor {
        &self.bg_color
    }

    /// Devuelve el color del texto del contenedor.
    pub fn text_color(&self) -> &TextColor {
        &self.text_color
    }

    /// Devuelve el borde del contenedor.
    pub fn border(&self) -> &Border {
        &self.border
    }

    /// Devuelve las esquinas redondeadas del contenedor.
    pub fn rounded(&self) -> &Rounded {
        &self.rounded
    }
}
