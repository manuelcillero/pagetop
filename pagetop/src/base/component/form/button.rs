use crate::prelude::*;

pub const COMPONENT_BUTTON: &str = "pagetop::component::form::button";

pub enum ButtonType {
    Button,
    Reset,
    Submit,
}

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
        Button {
            weight     : 0,
            renderable : render_always,
            classes    : Classes::new_with_default("btn btn-primary"),
            button_type: ButtonType::Button,
            name       : AttributeValue::new(),
            value      : AttributeValue::new(),
            autofocus  : AttributeValue::new(),
            disabled   : AttributeValue::new(),
            template   : "default".to_owned(),
        }
        .with_classes(ClassesOp::AddFirst, "form-button")
    }

    fn handler(&self) -> &'static str {
        COMPONENT_BUTTON
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, context: &InContext) -> bool {
        (self.renderable)(context)
    }

    fn default_render(&self, _: &mut InContext) -> Markup {
        let button_type = match self.button_type() {
            ButtonType::Button => "button",
            ButtonType::Reset  => "reset",
            ButtonType::Submit => "submit",
        };
        let id = match self.name().get() {
            Some(name) => Some(concat_string!("edit-", name)),
            _ => None,
        };
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
    pub fn button(value: &str) -> Self {
        Button::new().with_value(value)
    }

    pub fn reset(value: &str) -> Self {
        let mut button = Button::new()
            .with_classes(ClassesOp::Replace("form-button"), "form-reset")
            .with_value(value);
        button.button_type = ButtonType::Reset;
        button
    }

    pub fn submit(value: &str) -> Self {
        let mut button = Button::new()
            .with_classes(ClassesOp::Replace("form-button"), "form-submit")
            .with_value(value);
        button.button_type = ButtonType::Submit;
        button
    }

    // Button BUILDER.

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_renderable(mut self, renderable: Renderable) -> Self {
        self.alter_renderable(renderable);
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

    pub fn alter_renderable(&mut self, renderable: Renderable) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter(op, classes);
        self
    }

    pub fn alter_name(&mut self, name: &str) -> &mut Self {
        self.name.with_value(name);
        self
    }

    pub fn alter_value(&mut self, value: &str) -> &mut Self {
        self.value.with_value(value);
        self
    }

    pub fn alter_autofocus(&mut self, toggle: bool) -> &mut Self {
        self.autofocus.with_value(match toggle {
            true => "autofocus",
            false => "",
        });
        self
    }

    pub fn alter_disabled(&mut self, toggle: bool) -> &mut Self {
        self.disabled.with_value(match toggle {
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
