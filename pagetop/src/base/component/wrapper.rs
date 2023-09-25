use crate::prelude::*;

new_handle!(COMPONENT_WRAPPER);

actions_for_component!(Wrapper);

#[derive(Default)]
pub enum WrapperType {
    #[default]
    Container,
    Header,
    Footer,
    Main,
    Section,
}

#[rustfmt::skip]
#[derive(Default)]
pub struct Wrapper {
    weight       : Weight,
    renderable   : Renderable,
    id           : IdentifierValue,
    classes      : Classes,
    inner_classes: Classes,
    wrapper_type : WrapperType,
    stuff        : ArcComponents,
    template     : String,
}

impl ComponentTrait for Wrapper {
    fn new() -> Self {
        Wrapper::default()
            .with_classes(ClassesOp::SetDefault, "container")
            .with_inner_classes(ClassesOp::SetDefault, "container")
    }

    fn handle(&self) -> Handle {
        COMPONENT_WRAPPER
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

    fn before_prepare_component(&mut self, cx: &mut Context) {
        run_actions_before_prepare_wrapper(self, cx);
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        match self.wrapper_type() {
            WrapperType::Header => PrepareMarkup::With(html! {
                header id=[self.id()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().prepare(cx))
                    }
                }
            }),
            WrapperType::Footer => PrepareMarkup::With(html! {
                footer id=[self.id()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().prepare(cx))
                    }
                }
            }),
            WrapperType::Main => PrepareMarkup::With(html! {
                main id=[self.id()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().prepare(cx))
                    }
                }
            }),
            WrapperType::Section => PrepareMarkup::With(html! {
                section id=[self.id()] class=[self.classes().get()] {
                    div class=[self.inner_classes().get()] {
                        (self.components().prepare(cx))
                    }
                }
            }),
            _ => PrepareMarkup::With(html! {
                div id=[self.id()] class=[self.classes().get()] {
                    (self.components().prepare(cx))
                }
            }),
        }
    }

    fn after_prepare_component(&mut self, cx: &mut Context) {
        run_actions_after_prepare_wrapper(self, cx);
    }
}

impl Wrapper {
    pub fn header() -> Self {
        let mut c = Wrapper::new().with_classes(ClassesOp::SetDefault, "header");
        c.wrapper_type = WrapperType::Header;
        c
    }

    pub fn footer() -> Self {
        let mut c = Wrapper::new().with_classes(ClassesOp::SetDefault, "footer");
        c.wrapper_type = WrapperType::Footer;
        c
    }

    pub fn main() -> Self {
        let mut c = Wrapper::new().with_classes(ClassesOp::SetDefault, "main");
        c.wrapper_type = WrapperType::Main;
        c
    }

    pub fn section() -> Self {
        let mut c = Wrapper::new().with_classes(ClassesOp::SetDefault, "section");
        c.wrapper_type = WrapperType::Section;
        c
    }

    // Wrapper BUILDER.

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
    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_inner_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.inner_classes.alter_value(op, classes);
        self
    }

    pub fn with_component(mut self, component: impl ComponentTrait) -> Self {
        self.stuff.alter(ArcOp::Add(ArcComponent::with(component)));
        self
    }

    #[fn_builder]
    pub fn alter_components(&mut self, op: ArcOp) -> &mut Self {
        self.stuff.alter(op);
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Wrapper GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn inner_classes(&self) -> &Classes {
        &self.inner_classes
    }

    pub fn wrapper_type(&self) -> &WrapperType {
        &self.wrapper_type
    }

    pub fn components(&self) -> &ArcComponents {
        &self.stuff
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
