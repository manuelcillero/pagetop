use pagetop::prelude::*;

use crate::theme::*;

/// Componente para crear un **contenedor de componentes**
/// ([`container`](crate::theme::bs::container)).
///
/// Envuelve un conjunto de componentes en un contenedor establecido que se crea aplicando uno de
/// los tipos definidos en [`container::Kind`](crate::theme::bs::container::Kind).
///
/// Si no contiene elementos, el componente **no se renderiza**.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop_bootsier::theme::*;
///
/// let main = bs::Container::main()
///     .with_id("main-page")
///     .with_width(bs::container::Width::From(token::BreakPoint::LG));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Container {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el tipo semántico del contenedor.
    container_kind: bs::container::Kind,
    /// Devuelve el comportamiento para el ancho del contenedor.
    container_width: bs::container::Width,
    /// Devuelve la lista de componentes (`children`) del contenedor.
    children: Children,
}

#[async_trait]
impl Component for Container {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.props.get_id()
    }

    fn setup(&mut self, _cx: &Context) {
        self.alter_prop(PropsOp::prepend_classes(self.container_width().to_class()));
    }

    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let output = self.children().render(cx).await;
        if output.is_empty() {
            return Ok(html! {});
        }
        let style = match self.container_width() {
            bs::container::Width::FluidMax(w) if w.is_measurable() => {
                Some(util::join!("max-width: ", w.to_string(), ";"))
            }
            _ => None,
        };
        Ok(match self.container_kind() {
            bs::container::Kind::Default => html! {
                div (self.props()) style=[style] {
                    (output)
                }
            },
            bs::container::Kind::Main => html! {
                main (self.props()) style=[style] {
                    (output)
                }
            },
            bs::container::Kind::Header => html! {
                header (self.props()) style=[style] {
                    (output)
                }
            },
            bs::container::Kind::Footer => html! {
                footer (self.props()) style=[style] {
                    (output)
                }
            },
            bs::container::Kind::Section => html! {
                section (self.props()) style=[style] {
                    (output)
                }
            },
            bs::container::Kind::Article => html! {
                article (self.props()) style=[style] {
                    (output)
                }
            },
        })
    }
}

impl Container {
    /// Crea un contenedor de tipo `Main` (`<main>`).
    pub fn main() -> Self {
        Self {
            container_kind: bs::container::Kind::Main,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Header` (`<header>`).
    pub fn header() -> Self {
        Self {
            container_kind: bs::container::Kind::Header,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Footer` (`<footer>`).
    pub fn footer() -> Self {
        Self {
            container_kind: bs::container::Kind::Footer,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Section` (`<section>`).
    pub fn section() -> Self {
        Self {
            container_kind: bs::container::Kind::Section,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Article` (`<article>`).
    pub fn article() -> Self {
        Self {
            container_kind: bs::container::Kind::Article,
            ..Default::default()
        }
    }

    // **< Container BUILDER >**********************************************************************

    /// Establece el identificador único del componente; igual a `with_prop(PropsOp::set_id(id))`.
    #[builder_fn]
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    ///
    /// También acepta clases predefinidas para:
    ///
    /// - Modificar el color de fondo ([`Background`]).
    /// - Definir la apariencia del texto ([`Text`]).
    /// - Establecer bordes ([`Border`]).
    /// - Redondear las esquinas ([`Rounded`]).
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Establece el comportamiento del ancho para el contenedor.
    #[builder_fn]
    pub fn with_width(mut self, width: bs::container::Width) -> Self {
        self.container_width = width;
        self
    }

    /// Añade un nuevo componente al contenedor o modifica la lista de componentes (`children`) con
    /// una operación [`ChildOp`].
    #[builder_fn]
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.children.alter_child(op.into());
        self
    }
}
