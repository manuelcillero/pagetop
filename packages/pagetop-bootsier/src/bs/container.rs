use pagetop::prelude::*;

use crate::bs::BreakPoint;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub enum ContainerType {
    #[default]
    Default,  // Contenedor genérico
    Main,     // Contenido principal
    Header,   // Encabezado
    Footer,   // Pie
    Section,  // Sección específica de contenido
    Article,  // Artículo dentro de una sección
}

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Container {
    id            : OptionId,
    classes       : OptionClasses,
    container_type: ContainerType,
    breakpoint    : BreakPoint,
    children      : Children,
}

impl ComponentTrait for Container {
    fn new() -> Self {
        Container::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            trio_string!("container", "-", self.breakpoint().to_string()),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let output = self.children().render(cx);
        if output.is_empty() {
            return PrepareMarkup::None;
        }
        let style = if let BreakPoint::FluidMax(max_width) = self.breakpoint() {
            Some(join_string!("max-width: ", max_width.to_string(), ";"))
        } else {
            None
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
    pub fn main() -> Self {
        Container {
            container_type: ContainerType::Main,
            ..Default::default()
        }
    }

    pub fn header() -> Self {
        Container {
            container_type: ContainerType::Header,
            ..Default::default()
        }
    }

    pub fn footer() -> Self {
        Container {
            container_type: ContainerType::Footer,
            ..Default::default()
        }
    }

    pub fn section() -> Self {
        Container {
            container_type: ContainerType::Section,
            ..Default::default()
        }
    }

    pub fn article() -> Self {
        Container {
            container_type: ContainerType::Article,
            ..Default::default()
        }
    }

    // Container BUILDER.

    #[fn_builder]
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl Into<String>) -> Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn with_breakpoint(mut self, bp: BreakPoint) -> Self {
        self.breakpoint = bp;
        self
    }

    pub fn with_child(mut self, child: impl ComponentTrait) -> Self {
        self.children.add(ChildComponent::with(child));
        self
    }

    #[fn_builder]
    pub fn with_children(mut self, op: ChildOp) -> Self {
        self.children.alter_child(op);
        self
    }

    // Container GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn container_type(&self) -> &ContainerType {
        &self.container_type
    }

    pub fn breakpoint(&self) -> &BreakPoint {
        &self.breakpoint
    }

    pub fn children(&self) -> &Children {
        &self.children
    }
}
