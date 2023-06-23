use pagetop::prelude::*;

use_handle!(COMPONENT_FORM);

action_before_prepare_component!(ACTION_BEFORE_PREPARE_FORM for Form);

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

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        (self.renderable.check)(rcx)
    }

    fn before_prepare(&mut self, rcx: &mut RenderContext) {
        run_actions_before_prepare_component(self, rcx);
    }

    fn prepare_component(&self, rcx: &mut RenderContext) -> PrepareMarkup {
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
                div { (self.elements().prepare(rcx)) }
            }
        })
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

    #[fn_builder]
    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
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

    pub fn with_element(mut self, element: impl ComponentTrait) -> Self {
        self.elements.alter_bundle(BundleOp::Add, element);
        self
    }

    pub fn alter_elements(&mut self, op: BundleOp, element: impl ComponentTrait) -> &mut Self {
        self.elements.alter_bundle(op, element);
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Form GETTERS.

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
