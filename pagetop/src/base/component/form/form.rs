use crate::prelude::*;

pub const FORM_COMPONENT: &str = "pagetop::component::form";

pub enum FormMethod {Get, Post}

pub struct Form {
    renderable: fn() -> bool,
    weight    : isize,
    elements  : ComponentsBundle,
    action    : AttributeValue,
    charset   : AttributeValue,
    method    : FormMethod,
    id        : IdentifierValue,
    classes   : Classes,
    template  : String,
}

impl ComponentTrait for Form {
    fn new() -> Self {
        Form {
            renderable: render_always,
            weight    : 0,
            elements  : ComponentsBundle::new(),
            action    : AttributeValue::new(),
            charset   : AttributeValue::new_with_value("UTF-8"),
            method    : FormMethod::Post,
            id        : IdentifierValue::new(),
            classes   : Classes::new_with_default("form"),
            template  : "default".to_owned(),
        }
    }

    fn handler(&self) -> &'static str {
        FORM_COMPONENT
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn default_render(&self, context: &mut InContext) -> Markup {
        let method = match self.method() {
            FormMethod::Get => None,
            FormMethod::Post => Some("post".to_owned())
        };
        html! {
            form
                id=[self.id().get()]
                class=[self.classes().get()]
                action=[self.action().get()]
                method=[method]
                accept-charset=[self.charset().get()]
            {
                div { (self.elements().render(context)) }
            }
        }
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl Form {

    // Form CONTAINER.

    pub fn add(mut self, element: impl ComponentTrait) -> Self {
        self.elements.add(element);
        self
    }

    pub fn elements(&self) -> &ComponentsBundle {
        &self.elements
    }

    // Form BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_action(mut self, action: &str) -> Self {
        self.alter_action(action);
        self
    }

    pub fn with_charset(mut self, charset: &str) -> Self {
        self.alter_charset(charset);
        self
    }

    pub fn with_method(mut self, method: FormMethod) -> Self {
        self.alter_method(method);
        self
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.alter_id(id);
        self
    }

    pub fn with_classes(mut self, classes: &str, op: ClassesOp) -> Self {
        self.alter_classes(classes, op);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Form ALTER.

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_action(&mut self, action: &str) -> &mut Self {
        self.action.with_value(action);
        self
    }

    pub fn alter_charset(&mut self, charset: &str) -> &mut Self {
        self.charset.with_value(charset);
        self
    }

    pub fn alter_method(&mut self, method: FormMethod) -> &mut Self {
        self.method = method;
        self
    }

    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.with_value(id);
        self
    }

    pub fn alter_classes(&mut self, classes: &str, op: ClassesOp) -> &mut Self {
        self.classes.alter(classes, op);
        self
    }

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Form GETTERS.

    pub fn action(&self) -> &AttributeValue {
        &self.action
    }

    pub fn charset(&self) -> &AttributeValue {
        &self.charset
    }

    pub fn method(&self) -> &FormMethod {
        &self.method
    }

    pub fn id(&self) -> &IdentifierValue {
        &self.id
    }

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
