use crate::prelude::*;

pub enum ButtonType {Button, Reset, Submit}

pub struct Button {
    renderable : fn() -> bool,
    weight     : i8,
    button_type: ButtonType,
    name       : OptAttr,
    value      : OptAttr,
    autofocus  : OptAttr,
    disabled   : OptAttr,
    classes    : Classes,
    template   : String,
}

impl PageComponent for Button {
    fn new() -> Self {
        Button {
            renderable : always,
            weight     : 0,
            button_type: ButtonType::Button,
            name       : OptAttr::none(),
            value      : OptAttr::none(),
            autofocus  : OptAttr::none(),
            disabled   : OptAttr::none(),
            classes    : Classes::none(),
            template   : "default".to_owned(),
        }
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> i8 {
        self.weight
    }

    fn default_render(&self, _: &mut PageAssets) -> Markup {
        let (button_type, button_class) = match self.button_type() {
            ButtonType::Button => ("button", "btn btn-primary form-button"),
            ButtonType::Reset  => ("reset",  "btn btn-primary form-reset" ),
            ButtonType::Submit => ("submit", "btn btn-primary form-submit"),
        };
        let id = match self.name() {
            Some(name) => Some(concat_string!("edit-", name)),
            _ => None
        };
        html! {
            button
                type=(button_type)
                id=[id]
                class=[self.classes(button_class)]
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
}

impl Button {
    pub fn button(value: &str) -> Self {
        let mut button = Button::new();
        button.with_value(value);
        button
    }

    pub fn reset(value: &str) -> Self {
        let mut button = Button::new();
        button.with_value(value);
        button.button_type = ButtonType::Reset;
        button
    }

    pub fn submit(value: &str) -> Self {
        let mut button = Button::new();
        button.with_value(value);
        button.button_type = ButtonType::Submit;
        button
    }

    // Button BUILDER.

    pub fn with_renderable(&mut self, renderable: fn() -> bool) -> &Self {
        self.renderable = renderable;
        self
    }

    pub fn with_weight(&mut self, weight: i8) -> &Self {
        self.weight = weight;
        self
    }

    pub fn with_name(&mut self, name: &str) -> &Self {
        self.name.with_value(name);
        self
    }

    pub fn with_value(&mut self, value: &str) -> &Self {
        self.value.with_value(value);
        self
    }

    pub fn with_autofocus(&mut self, toggle: bool) -> &Self {
        self.autofocus.with_value(match toggle {
            true => "autofocus",
            false => "",
        });
        self
    }

    pub fn with_disabled(&mut self, toggle: bool) -> &Self {
        self.disabled.with_value(match toggle {
            true => "disabled",
            false => "",
        });
        self
    }

    pub fn set_classes(&mut self, classes: &str) -> &Self {
        self.classes.set_classes(classes);
        self
    }

    pub fn add_classes(&mut self, classes: &str) -> &Self {
        self.classes.add_classes(classes);
        self
    }

    pub fn using_template(&mut self, template: &str) -> &Self {
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

    pub fn classes(&self, default: &str) -> Option<String> {
        self.classes.option(default)
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
