use crate::prelude::*;

pub_const_handler!(COMPONENT_BUTTON);

#[derive(Default)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

#[rustfmt::skip]
#[derive(Default)]
pub struct Button {
    weight     : isize,
    renderable : Renderable,
    classes    : Classes,
    button_type: ButtonType,
    name       : AttributeValue,
    value      : AttributeValue,
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

    fn handler(&self) -> Handler {
        COMPONENT_BUTTON
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, context: &PageContext) -> bool {
        (self.renderable.check)(context)
    }

    fn default_render(&self, _: &mut PageContext) -> Markup {
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
                value=[self.value().get()]
                autofocus=[self.autofocus().get()]
                disabled=[self.disabled().get()]
            {
                @match self.value().get() {
                    Some(value) => { (value) },
                    None => {},
                }
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
    pub fn new_with_value(value: &str) -> Self {
        Button::new().with_value(value)
    }

    pub fn submit(value: &str) -> Self {
        let mut button = Button::new()
            .with_classes(ClassesOp::Replace("form-button"), "form-submit")
            .with_value(value);
        button.button_type = ButtonType::Submit;
        button
    }

    pub fn reset(value: &str) -> Self {
        let mut button = Button::new()
            .with_classes(ClassesOp::Replace("form-button"), "form-reset")
            .with_value(value);
        button.button_type = ButtonType::Reset;
        button
    }

    // Button BUILDER.

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_renderable(mut self, check: IsRenderable) -> Self {
        self.alter_renderable(check);
        self
    }

    pub fn with_classes(mut self, op: ClassesOp, classes: &str) -> Self {
        self.alter_classes(op, classes);
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.alter_name(name);
        self
    }

    pub fn with_value(mut self, value: &str) -> Self {
        self.alter_value(value);
        self
    }

    pub fn with_autofocus(mut self, toggle: bool) -> Self {
        self.alter_autofocus(toggle);
        self
    }

    pub fn with_disabled(mut self, toggle: bool) -> Self {
        self.alter_disabled(toggle);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Button ALTER.

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    pub fn alter_name(&mut self, name: &str) -> &mut Self {
        self.name.alter_value(name);
        self
    }

    pub fn alter_value(&mut self, value: &str) -> &mut Self {
        self.value.alter_value(value);
        self
    }

    pub fn alter_autofocus(&mut self, toggle: bool) -> &mut Self {
        self.autofocus.alter_value(match toggle {
            true => "autofocus",
            false => "",
        });
        self
    }

    pub fn alter_disabled(&mut self, toggle: bool) -> &mut Self {
        self.disabled.alter_value(match toggle {
            true => "disabled",
            false => "",
        });
        self
    }

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

    pub fn value(&self) -> &AttributeValue {
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
