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

// **< Width >**************************************************************************************

/// Define cómo se comporta el ancho de un contenedor ([`Container`]).
///
/// Cada variante se traduce en una clase CSS (`container`, `container-{nombre}` o
/// `container-fluid`), con el nombre que el tema activo da al punto de corte, y el tema decide qué
/// anchos máximos le corresponden. [`FluidMax`](Self::FluidMax) añade además un `max-width` en
/// línea.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Width {
    /// Aplica los anchos máximos predefinidos por el tema para cada punto de corte. Por debajo del
    /// menor punto de corte ocupa el 100% del ancho disponible.
    Responsive,
    /// Aplica los anchos máximos predefinidos a partir del punto de corte indicado. Por debajo de
    /// ese punto de corte ocupa el 100% del ancho disponible.
    From(Breakpoint),
    /// Ocupa el 100% del ancho disponible siempre.
    Fluid,
    /// Ocupa el 100% del ancho disponible hasta un ancho máximo explícito.
    FluidMax(UnitValue),
}

// **< Container >**********************************************************************************

/// Componente para crear un **contenedor de componentes**.
///
/// Envuelve un conjunto de componentes en un contenedor establecido que se crea aplicando uno de
/// los tipos definidos en [`Kind`]. Opcionalmente, su ancho se controla con [`Width`] mediante
/// [`with_width()`](Self::with_width); sin él, el contenedor no añade ninguna clase de ancho.
///
/// Si no contiene elementos, el componente **no se renderiza**.
///
/// # Ejemplos
///
/// ```rust,no_run
/// use pagetop::prelude::*;
///
/// let main = Container::main().with_id("main-page");
/// ```
///
/// Contenedor centrado que ocupa todo el ancho hasta un máximo explícito:
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// let page = Container::new().with_width(container::Width::FluidMax(UnitValue::RelRem(75.0)));
/// ```
#[derive(AutoDefault, Clone, Debug, Getters)]
pub struct Container {
    /// Devuelve identificador, clases CSS, atributos HTML y valores extra del componente.
    props: Props,
    /// Devuelve el tipo semántico del contenedor.
    kind: Kind,
    /// Devuelve el comportamiento del ancho, si se ha fijado.
    #[getters(copy)]
    width: Option<Width>,
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

    fn setup(&mut self, cx: &mut Context) {
        if let Some(width) = self.width() {
            let class: CowStr = match width {
                Width::Responsive => "container".into(),
                Width::From(bp) => match bp.resolved(cx) {
                    Some(entry) => util::join!("container-", entry.name).into(),
                    None => "container".into(),
                },
                Width::Fluid | Width::FluidMax(_) => "container-fluid".into(),
            };
            self.alter_prop(PropsOp::prepend_classes(class));
            if let Width::FluidMax(max) = width
                && max.is_measurable()
            {
                self.alter_prop(PropsOp::add_style("max-width", max.to_string()));
            }
        }
    }

    #[rustfmt::skip]
    async fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        let output = self.children().render(cx).await;
        if output.is_empty() {
            return Ok(html! {});
        }
        let container_props = self.props().unpack(cx);
        Ok(match self.kind() {
            Kind::Default => html! { div     (container_props) { (output) } },
            Kind::Main    => html! { main    (container_props) { (output) } },
            Kind::Header  => html! { header  (container_props) { (output) } },
            Kind::Footer  => html! { footer  (container_props) { (output) } },
            Kind::Section => html! { section (container_props) { (output) } },
            Kind::Article => html! { article (container_props) { (output) } },
        })
    }
}

#[builder_impl]
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
    pub fn with_id(mut self, id: impl Into<CowStr>) -> Self {
        self.props.alter_id(id);
        self
    }

    /// Modifica identificador, clases CSS, atributos HTML o valores extra del componente.
    pub fn with_prop(mut self, op: impl Into<PropsOp>) -> Self {
        self.props.alter_prop(op);
        self
    }

    /// Define el comportamiento del ancho del contenedor. Con `None` no se añade ninguna clase de
    /// ancho y cada tema decide cómo tratarlo.
    ///
    /// Ver [`Width`] para las variantes disponibles.
    pub fn with_width(mut self, width: impl Into<Option<Width>>) -> Self {
        self.width = width.into();
        self
    }

    /// Añade un nuevo componente al contenedor o modifica la lista de componentes (`children`) con
    /// una operación [`ChildOp`].
    pub fn with_child(mut self, op: impl Into<ChildOp>) -> Self {
        self.children.alter_child(op.into());
        self
    }
}
