//! Definiciones para crear contenedores de componentes ([`Container`]).

use crate::prelude::*;

// **< Kind >***************************************************************************************

/// Tipo de contenedor (`Container`).
///
/// Permite aplicar la etiqueta HTML apropiada (`<main>`, `<header>`, etc.) manteniendo una API
/// común a todos los contenedores.
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum Kind {
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

// **< Container >**********************************************************************************

/// Componente para crear un **contenedor de componentes**.
///
/// Envuelve un conjunto de componentes en un contenedor establecido que se crea aplicando uno de
/// los tipos definidos en [`Kind`].
///
/// Si no contiene elementos, el componente **no se renderiza**.
///
/// # Ejemplo
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let main = Container::main().with_id("main-page");
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Container {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el tipo semántico del contenedor.
    kind: Kind,
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

    #[rustfmt::skip]
    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let output = self.children().render(cx).await;
        if output.is_empty() {
            return Ok(html! {});
        }
        Ok(match self.kind() {
            Kind::Default => html! { div     (self.props()) { (output) } },
            Kind::Main    => html! { main    (self.props()) { (output) } },
            Kind::Header  => html! { header  (self.props()) { (output) } },
            Kind::Footer  => html! { footer  (self.props()) { (output) } },
            Kind::Section => html! { section (self.props()) { (output) } },
            Kind::Article => html! { article (self.props()) { (output) } },
        })
    }
}

impl Container {
    /// Crea un contenedor de tipo `Main` (`<main>`).
    pub fn main() -> Self {
        Self {
            kind: Kind::Main,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Header` (`<header>`).
    pub fn header() -> Self {
        Self {
            kind: Kind::Header,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Footer` (`<footer>`).
    pub fn footer() -> Self {
        Self {
            kind: Kind::Footer,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Section` (`<section>`).
    pub fn section() -> Self {
        Self {
            kind: Kind::Section,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Article` (`<article>`).
    pub fn article() -> Self {
        Self {
            kind: Kind::Article,
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
    #[builder_fn]
    pub fn with_prop(mut self, op: PropsOp) -> Self {
        self.props.alter_prop(op);
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
