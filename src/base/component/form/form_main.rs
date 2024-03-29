use crate::prelude::*;

#[derive(AutoDefault)]
pub enum FormMethod {
    #[default]
    Post,
    Get,
}

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct Form {
    id        : OptionId,
    weight    : Weight,
    renderable: Renderable,
    classes   : OptionClasses,
    action    : OptionString,
    charset   : OptionString,
    method    : FormMethod,
    mixed     : MixedComponents,
}

impl ComponentTrait for Form {
    fn new() -> Self {
        Form::default()
            .with_classes(ClassesOp::Add, "form")
            .with_charset("UTF-8")
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
        let method = match self.method() {
            FormMethod::Post => Some("post".to_owned()),
            FormMethod::Get => None,
        };
        PrepareMarkup::With(html! {
            form
                id=[self.id()]
                class=[self.classes().get()]
                action=[self.action().get()]
                method=[method]
                accept-charset=[self.charset().get()]
            {
                div { (self.elements().render(cx)) }
            }
        })
    }
}

impl Form {
    // Form BUILDER.

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
    pub fn alter_action(&mut self, action: &str) -> &mut Self {
        self.action.alter_value(action);
        self
    }

    #[fn_builder]
    pub fn alter_charset(&mut self, charset: &str) -> &mut Self {
        self.charset.alter_value(charset);
        self
    }

    #[fn_builder]
    pub fn alter_method(&mut self, method: FormMethod) -> &mut Self {
        self.method = method;
        self
    }

    #[fn_builder]
    pub fn alter_elements(&mut self, op: AnyOp) -> &mut Self {
        self.mixed.alter_value(op);
        self
    }

    #[rustfmt::skip]
    pub fn add_element(mut self, element: impl ComponentTrait) -> Self {
        self.mixed.alter_value(AnyOp::Add(AnyComponent::with(element)));
        self
    }

    // Form GETTERS.

    pub fn action(&self) -> &OptionString {
        &self.action
    }

    pub fn charset(&self) -> &OptionString {
        &self.charset
    }

    pub fn method(&self) -> &FormMethod {
        &self.method
    }

    pub fn elements(&self) -> &MixedComponents {
        &self.mixed
    }
}
