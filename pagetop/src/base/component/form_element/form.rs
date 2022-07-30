use crate::prelude::*;

pub_const_handler!(COMPONENT_FORM);

hook_before_render_component!(HOOK_BEFORE_RENDER_FORM, Form);

pub enum FormMethod {
    Get,
    Post,
}

pub struct Form {
    weight    : isize,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    action    : AttributeValue,
    charset   : AttributeValue,
    method    : FormMethod,
    elements  : ComponentsBundle,
    template  : String,
}

impl ComponentTrait for Form {
    fn new() -> Self {
        Form {
            weight    : 0,
            renderable: render_always,
            id        : IdentifierValue::new(),
            classes   : Classes::new_with_default("form"),
            action    : AttributeValue::new(),
            charset   : AttributeValue::new_with_value("UTF-8"),
            method    : FormMethod::Post,
            elements  : ComponentsBundle::new(),
            template  : "default".to_owned(),
        }
    }

    fn handler(&self) -> Handler {
        COMPONENT_FORM
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, context: &PageContext) -> bool {
        (self.renderable)(context)
    }

    fn before_render(&mut self, context: &mut PageContext) {
        before_render_inline(self, context);
    }

    fn default_render(&self, context: &mut PageContext) -> Markup {
        let method = match self.method() {
            FormMethod::Get => None,
            FormMethod::Post => Some("post".to_owned()),
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
    // Form BUILDER.

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.alter_id(id);
        self
    }

    pub fn with_classes(mut self, op: ClassesOp, classes: &str) -> Self {
        self.alter_classes(op, classes);
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

    pub fn with_element(mut self, element: impl ComponentTrait) -> Self {
        self.alter_element(element);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Form ALTER.

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_renderable(&mut self, renderable: Renderable) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.with_value(id);
        self
    }

    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter(op, classes);
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

    pub fn alter_element(&mut self, element: impl ComponentTrait) -> &mut Self {
        self.elements.add(element);
        self
    }

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Form GETTERS.

    pub fn id(&self) -> &IdentifierValue {
        &self.id
    }

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn action(&self) -> &AttributeValue {
        &self.action
    }

    pub fn charset(&self) -> &AttributeValue {
        &self.charset
    }

    pub fn method(&self) -> &FormMethod {
        &self.method
    }

    pub fn elements(&self) -> &ComponentsBundle {
        &self.elements
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
