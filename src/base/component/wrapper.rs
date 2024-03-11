use crate::prelude::*;

#[derive(AutoDefault)]
pub enum WrapperType {
    #[default]
    Container,
    Header,
    Footer,
    Main,
    Section,
}

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct Wrapper {
    id          : OptionId,
    weight      : Weight,
    renderable  : Renderable,
    classes     : OptionClasses,
    wrapper_type: WrapperType,
    mixed       : MixedComponents,
}

impl ComponentTrait for Wrapper {
    fn new() -> Self {
        Wrapper::default().with_classes(ClassesOp::Add, "wrapper__container")
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
            WrapperType::Container => PrepareMarkup::With(html! {
                div id=[self.id()] class=[self.classes().get()] {
                    div class="wrapper__content" {
                        (self.components().render(cx))
                    }
                }
            }),
            WrapperType::Main => PrepareMarkup::With(html! {
                main id=[self.id()] class=[self.classes().get()] {
                    div class="wrapper__content" {
                        (self.components().render(cx))
                    }
                }
            }),
            WrapperType::Section => PrepareMarkup::With(html! {
                section id=[self.id()] class=[self.classes().get()] {
                    div class="wrapper__content" {
                        (self.components().render(cx))
                    }
                }
            }),
            WrapperType::Header => PrepareMarkup::With(html! {
                header id=[self.id()] class=[self.classes().get()] {
                    div class="wrapper__content" {
                        (self.components().render(cx))
                    }
                }
            }),
            WrapperType::Footer => PrepareMarkup::With(html! {
                footer id=[self.id()] class=[self.classes().get()] {
                    div class="wrapper__content" {
                        (self.components().render(cx))
                    }
                }
            }),
        }
    }
}

impl Wrapper {
    pub fn main() -> Self {
        let mut c = Wrapper::default().with_classes(ClassesOp::Add, "main__container");
        c.wrapper_type = WrapperType::Main;
        c
    }

    pub fn section() -> Self {
        let mut c = Wrapper::default().with_classes(ClassesOp::Add, "section__container");
        c.wrapper_type = WrapperType::Section;
        c
    }

    pub fn header() -> Self {
        let mut c = Wrapper::default().with_classes(ClassesOp::Add, "header__container");
        c.wrapper_type = WrapperType::Header;
        c
    }

    pub fn footer() -> Self {
        let mut c = Wrapper::default().with_classes(ClassesOp::Add, "footer__container");
        c.wrapper_type = WrapperType::Footer;
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
    pub fn alter_components(&mut self, op: MixedOp) -> &mut Self {
        self.mixed.alter_value(op);
        self
    }

    #[rustfmt::skip]
    pub fn add_component(mut self, component: impl ComponentTrait) -> Self {
        self.mixed.alter_value(MixedOp::Add(AnyComponent::with(component)));
        self
    }

    // Wrapper GETTERS.

    pub fn wrapper_type(&self) -> &WrapperType {
        &self.wrapper_type
    }

    pub fn components(&self) -> &MixedComponents {
        &self.mixed
    }
}
