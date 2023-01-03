use crate::prelude::*;

pub_handle!(COMPONENT_FORM);

hook_before_render_component!(HOOK_BEFORE_RENDER_FORM, Form);

#[derive(Default)]
pub enum FormMethod {
    #[default]
    Post,
    Get,
}

#[rustfmt::skip]
#[derive(Default)]
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
        Form::default()
            .with_classes(ClassesOp::SetDefault, "form")
            .with_charset("UTF-8")
    }

    fn handle(&self) -> Handle {
        COMPONENT_FORM
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        (self.renderable.check)(rcx)
    }

    fn before_render(&mut self, rcx: &mut RenderContext) {
        before_render_inline(self, rcx);
    }

    fn default_render(&self, rcx: &mut RenderContext) -> Markup {
        let method = match self.method() {
            FormMethod::Post => Some("post".to_owned()),
            FormMethod::Get => None,
        };
        html! {
            form
                id=[self.id().get()]
                class=[self.classes().get()]
                action=[self.action().get()]
                method=[method]
                accept-charset=[self.charset().get()]
            {
                div { (self.elements().render(rcx)) }
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

    pub fn with_renderable(mut self, check: IsRenderable) -> Self {
        self.alter_renderable(check);
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

    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    pub fn alter_action(&mut self, action: &str) -> &mut Self {
        self.action.alter_value(action);
        self
    }

    pub fn alter_charset(&mut self, charset: &str) -> &mut Self {
        self.charset.alter_value(charset);
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
