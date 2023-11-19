use crate::prelude::*;
use crate::CrateHandle;

#[derive(SmartDefault)]
pub enum WrapperType {
    #[default]
    Container,
    Header,
    Footer,
    Main,
    Section,
}

#[rustfmt::skip]
#[derive(ComponentClasses, CrateHandle, SmartDefault)]
pub struct Wrapper {
    id           : OptionId,
    weight       : Weight,
    renderable   : Renderable,
    classes      : OptionClasses,
    inner_classes: OptionClasses,
    wrapper_type : WrapperType,
    stuff        : AnyComponents,
    template     : String,
}

impl ComponentTrait for Wrapper {
    fn new() -> Self {
        Wrapper::default()
            .with_classes(ClassesOp::Add, "container")
            .with_inner_classes(ClassesOp::Add, "container")
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        match self.wrapper_type() {
            WrapperType::Header => PrepareMarkup::With(html! {
                header id=[self.id()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().render(cx))
                    }
                }
            }),
            WrapperType::Footer => PrepareMarkup::With(html! {
                footer id=[self.id()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().render(cx))
                    }
                }
            }),
            WrapperType::Main => PrepareMarkup::With(html! {
                main id=[self.id()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().render(cx))
                    }
                }
            }),
            WrapperType::Section => PrepareMarkup::With(html! {
                section id=[self.id()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().render(cx))
                    }
                }
            }),
            _ => PrepareMarkup::With(html! {
                div id=[self.id()] class=[self.classes().get()] {
                    (self.components().render(cx))
                }
            }),
        }
    }
}

impl Wrapper {
    pub fn header() -> Self {
        let mut c = Wrapper::default()
            .with_classes(ClassesOp::Add, "header")
            .with_inner_classes(ClassesOp::Add, "container");
        c.wrapper_type = WrapperType::Header;
        c
    }

    pub fn footer() -> Self {
        let mut c = Wrapper::default()
            .with_classes(ClassesOp::Add, "footer")
            .with_inner_classes(ClassesOp::Add, "container");
        c.wrapper_type = WrapperType::Footer;
        c
    }

    pub fn main() -> Self {
        let mut c = Wrapper::default()
            .with_classes(ClassesOp::Add, "main")
            .with_inner_classes(ClassesOp::Add, "container");
        c.wrapper_type = WrapperType::Main;
        c
    }

    pub fn section() -> Self {
        let mut c = Wrapper::default()
            .with_classes(ClassesOp::Add, "section")
            .with_inner_classes(ClassesOp::Add, "container");
        c.wrapper_type = WrapperType::Section;
        c
    }

    // Wrapper BUILDER.

    #[fn_builder]
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: FnIsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_inner_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.inner_classes.alter_value(op, classes);
        self
    }

    #[rustfmt::skip]
    pub fn add_component(mut self, component: impl ComponentTrait) -> Self {
        self.stuff.alter_value(ArcAnyOp::Add(ArcAnyComponent::new(component)));
        self
    }

    #[fn_builder]
    pub fn alter_components(&mut self, op: ArcAnyOp) -> &mut Self {
        self.stuff.alter_value(op);
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Wrapper GETTERS.

    pub fn inner_classes(&self) -> &OptionClasses {
        &self.inner_classes
    }

    pub fn wrapper_type(&self) -> &WrapperType {
        &self.wrapper_type
    }

    pub fn components(&self) -> &AnyComponents {
        &self.stuff
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
