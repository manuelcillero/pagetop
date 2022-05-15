use crate::prelude::*;

pub const BUTTON_COMPONENT: &str = "pagetop::component::form::button";

pub enum ButtonType {Button, Reset, Submit}

pub struct Button {
    renderable : fn() -> bool,
    weight     : isize,
    button_type: ButtonType,
    name       : OptAttr,
    value      : OptAttr,
    autofocus  : OptAttr,
    disabled   : OptAttr,
    classes    : Classes,
    template   : String,
}

impl ComponentTrait for Button {
    fn new() -> Self {
        Button {
            renderable : render_always,
            weight     : 0,
            button_type: ButtonType::Button,
            name       : OptAttr::new(),
            value      : OptAttr::new(),
            autofocus  : OptAttr::new(),
            disabled   : OptAttr::new(),
            classes    : Classes::new_with_default("btn btn-primary"),
            template   : "default".to_owned(),
        }
        .with_classes("form-button", ClassesOp::AddFirst)
    }

    fn handler(&self) -> &'static str {
        BUTTON_COMPONENT
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn default_render(&self, _: &mut InContext) -> Markup {
        let button_type = match self.button_type() {
            ButtonType::Button => "button",
            ButtonType::Reset  => "reset",
            ButtonType::Submit => "submit",
        };
        let id = match self.name() {
            Some(name) => Some(concat_string!("edit-", name)),
            _ => None
        };
        html! {
            button
                type=(button_type)
                id=[id]
                class=[self.classes()]
                name=[self.name()]
                value=[self.value()]
                autofocus=[self.autofocus()]
                disabled=[self.disabled()]
            {
                @match self.value() {
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
            .with_classes("form-reset", ClassesOp::Replace("form-button"))
            .with_value(value);
        button.button_type = ButtonType::Reset;
        button
    }

    pub fn submit(value: &str) -> Self {
        let mut button = Button::new()
            .with_classes("form-submit", ClassesOp::Replace("form-button"))
            .with_value(value);
        button.button_type = ButtonType::Submit;
        button
    }

    // Button BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
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

    pub fn with_classes(mut self, classes: &str, op: ClassesOp) -> Self {
        self.alter_classes(classes, op);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Button ALTER.

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
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

    pub fn alter_classes(&mut self, classes: &str, op: ClassesOp) -> &mut Self {
        self.classes.alter(classes, op);
        self
    }

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Button GETTERS.

    pub fn button_type(&self) -> &ButtonType {
        &self.button_type
    }

    pub fn name(&self) -> &Option<String> {
        self.name.option()
    }

    pub fn value(&self) -> &Option<String> {
        self.value.option()
    }

    pub fn autofocus(&self) -> &Option<String> {
        self.autofocus.option()
    }

    pub fn disabled(&self) -> &Option<String> {
        self.disabled.option()
    }

    pub fn classes(&self) -> &Option<String> {
        self.classes.option()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
