use pagetop::prelude::*;

use crate::prelude::*;

/// Componente para crear un **contenedor de componentes**.
///
/// Envuelve un contenido con la etiqueta HTML indicada por [`container::Kind`]. Sólo se renderiza
/// si existen componentes hijos (*children*).
#[derive(AutoDefault, Getters)]
pub struct Container {
    #[getters(skip)]
    id: AttrId,
    /// Devuelve las clases CSS asociadas al contenedor.
    classes: AttrClasses,
    /// Devuelve el tipo semántico del contenedor.
    container_kind: container::Kind,
    /// Devuelve el comportamiento para el ancho del contenedor.
    container_width: container::Width,
    /// Devuelve la lista de componentes (`children`) del contenedor.
    children: Children,
}

impl Component for Container {
    fn new() -> Self {
        Self::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(ClassesOp::Prepend, self.container_width().to_class());
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let output = self.children().render(cx);
        if output.is_empty() {
            return PrepareMarkup::None;
        }
        let style = match self.container_width() {
            container::Width::FluidMax(w) if w.is_measurable() => {
                Some(util::join!("max-width: ", w.to_string(), ";"))
            }
            _ => None,
        };
        match self.container_kind() {
            container::Kind::Default => PrepareMarkup::With(html! {
                div id=[self.id()] class=[self.classes().get()] style=[style] {
                    (output)
                }
            }),
            container::Kind::Main => PrepareMarkup::With(html! {
                main id=[self.id()] class=[self.classes().get()] style=[style] {
                    (output)
                }
            }),
            container::Kind::Header => PrepareMarkup::With(html! {
                header id=[self.id()] class=[self.classes().get()] style=[style] {
                    (output)
                }
            }),
            container::Kind::Footer => PrepareMarkup::With(html! {
                footer id=[self.id()] class=[self.classes().get()] style=[style] {
                    (output)
                }
            }),
            container::Kind::Section => PrepareMarkup::With(html! {
                section id=[self.id()] class=[self.classes().get()] style=[style] {
                    (output)
                }
            }),
            container::Kind::Article => PrepareMarkup::With(html! {
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
        Self {
            container_kind: container::Kind::Main,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Header` (`<header>`).
    pub fn header() -> Self {
        Self {
            container_kind: container::Kind::Header,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Footer` (`<footer>`).
    pub fn footer() -> Self {
        Self {
            container_kind: container::Kind::Footer,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Section` (`<section>`).
    pub fn section() -> Self {
        Self {
            container_kind: container::Kind::Section,
            ..Default::default()
        }
    }

    /// Crea un contenedor de tipo `Article` (`<article>`).
    pub fn article() -> Self {
        Self {
            container_kind: container::Kind::Article,
            ..Default::default()
        }
    }

    // **< Container BUILDER >**********************************************************************

    /// Establece el identificador único (`id`) del contenedor.
    #[builder_fn]
    pub fn with_id(mut self, id: impl AsRef<str>) -> Self {
        self.id.alter_id(id);
        self
    }

    /// Modifica la lista de clases CSS aplicadas al contenedor.
    ///
    /// También acepta clases predefinidas para:
    ///
    /// - Modificar el color de fondo ([`classes::Background`]).
    /// - Definir la apariencia del texto ([`classes::Text`]).
    /// - Establecer bordes ([`classes::Border`]).
    /// - Redondear las esquinas ([`classes::Rounded`]).
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_value(op, classes);
        self
    }

    /// Establece el comportamiento del ancho para el contenedor.
    #[builder_fn]
    pub fn with_width(mut self, width: container::Width) -> Self {
        self.container_width = width;
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
}
