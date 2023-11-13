use crate::prelude::*;

#[derive(Default)]
pub enum FormMethod {
    #[default]
    Post,
    Get,
}

#[rustfmt::skip]
#[derive(Default)]
pub struct Form {
    id        : OptionId,
    weight    : Weight,
    renderable: Renderable,
    classes   : OptionClasses,
    action    : OptionString,
    charset   : OptionString,
    method    : FormMethod,
    stuff     : AnyComponents,
    template  : String,
}

impl_handle!(COMPONENT_BASE_FORM for Form);

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
    pub fn alter_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.classes.alter_value(op, classes);
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

    #[rustfmt::skip]
    pub fn with_element(mut self, element: impl ComponentTrait) -> Self {
        self.stuff.alter_value(ArcAnyOp::Add(ArcAnyComponent::new(element)));
        self
    }

    #[fn_builder]
    pub fn alter_elements(&mut self, op: ArcAnyOp) -> &mut Self {
        self.stuff.alter_value(op);
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Form GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn action(&self) -> &OptionString {
        &self.action
    }

    pub fn charset(&self) -> &OptionString {
        &self.charset
    }

    pub fn method(&self) -> &FormMethod {
        &self.method
    }

    pub fn elements(&self) -> &AnyComponents {
        &self.stuff
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
