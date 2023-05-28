use pagetop::prelude::*;

define_handle!(COMPONENT_BUTTON);

#[derive(Default)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

type ButtonValue = OneComponent<L10n>;

#[rustfmt::skip]
#[derive(Default)]
pub struct Button {
    weight     : isize,
    renderable : Renderable,
    classes    : Classes,
    button_type: ButtonType,
    name       : AttributeValue,
    value      : ButtonValue,
    autofocus  : AttributeValue,
    disabled   : AttributeValue,
    template   : String,
}

impl ComponentTrait for Button {
    fn new() -> Self {
        Button::default()
            .with_classes(ClassesOp::SetDefault, "btn btn-primary")
            .with_classes(ClassesOp::AddFirst, "form-button")
    }

    fn handle(&self) -> Handle {
        COMPONENT_BUTTON
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        (self.renderable.check)(rcx)
    }

    fn default_render(&self, rcx: &mut RenderContext) -> Markup {
        let button_type = match self.button_type() {
            ButtonType::Button => "button",
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
        };
        let id = self.name().get().map(|name| concat_string!("edit-", name));
        html! {
            button
                type=(button_type)
                id=[id]
                class=[self.classes().get()]
                name=[self.name().get()]
                value=(self.value().render(rcx))
                autofocus=[self.autofocus().get()]
                disabled=[self.disabled().get()]
            {
                (self.value().render(rcx))
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

impl Button {
    pub fn with(value: L10n) -> Self {
        Button::new().with_value(value)
    }

    pub fn submit(value: L10n) -> Self {
        let mut button = Button::new()
            .with_classes(ClassesOp::Replace("form-button"), "form-submit")
            .with_value(value);
        button.button_type = ButtonType::Submit;
        button
    }

    pub fn reset(value: L10n) -> Self {
        let mut button = Button::new()
            .with_classes(ClassesOp::Replace("form-button"), "form-reset")
            .with_value(value);
        button.button_type = ButtonType::Reset;
        button
    }

    // Button BUILDER.

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
    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_name(&mut self, name: &str) -> &mut Self {
        self.name.alter_value(name);
        self
    }

    #[fn_builder]
    pub fn alter_value(&mut self, value: L10n) -> &mut Self {
        self.value.set(value);
        self
    }

    #[fn_builder]
    pub fn alter_autofocus(&mut self, toggle: bool) -> &mut Self {
        self.autofocus.alter_value(match toggle {
            true => "autofocus",
            false => "",
        });
        self
    }

    #[fn_builder]
    pub fn alter_disabled(&mut self, toggle: bool) -> &mut Self {
        self.disabled.alter_value(match toggle {
            true => "disabled",
            false => "",
        });
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Button GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn button_type(&self) -> &ButtonType {
        &self.button_type
    }

    pub fn name(&self) -> &AttributeValue {
        &self.name
    }

    pub fn value(&self) -> &ButtonValue {
        &self.value
    }

    pub fn autofocus(&self) -> &AttributeValue {
        &self.autofocus
    }

    pub fn disabled(&self) -> &AttributeValue {
        &self.disabled
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
