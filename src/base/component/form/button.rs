use crate::prelude::*;

enum ButtonType {Button, Reset, Submit}

pub struct Button {
    renderable : fn() -> bool,
    weight     : i8,
    button_type: ButtonType,
    name       : Option<String>,
    value      : Option<String>,
    autofocus  : Option<String>,
    disabled   : Option<String>,
    template   : String,
}

impl PageComponent for Button {

    fn prepare() -> Self {
        Button {
            renderable : always,
            weight     : 0,
            button_type: ButtonType::Button,
            name       : None,
            value      : None,
            autofocus  : None,
            disabled   : None,
            template   : "default".to_string(),
        }
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> i8 {
        self.weight
    }

    fn default_render(&self, _: &mut PageAssets) -> Markup {
        let (button_type, button_class) = match &self.button_type {
            ButtonType::Button => ("button", "btn btn-primary form-button"),
            ButtonType::Reset  => ("reset",  "btn btn-primary form-reset" ),
            ButtonType::Submit => ("submit", "btn btn-primary form-submit")
        };
        let id_item = match &self.name {
            Some(name) => Some(format!("edit-{}", name)),
            _ => None
        };
        html! {
            button
                type=(button_type)
                id=[&id_item]
                class=(button_class)
                name=[&self.name]
                value=[&self.value]
                autofocus=[&self.autofocus]
                disabled=[&self.disabled]
            {
                @match &self.value {
                    Some(value) => (value),
                    _ => ""
                };
            }
        }
    }
}

impl Button {

    pub fn button(value: &str) -> Self {
        Button::prepare().with_value(value)
    }

    pub fn reset(value: &str) -> Self {
        let mut button = Button::prepare().with_value(value);
        button.button_type = ButtonType::Reset;
        button
    }

    pub fn submit(value: &str) -> Self {
        let mut button = Button::prepare().with_value(value);
        button.button_type = ButtonType::Submit;
        button
    }

    // Button BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.renderable = renderable;
        self
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = util::valid_id(name);
        self
    }

    pub fn with_value(mut self, value: &str) -> Self {
        self.value = util::optional_value(value);
        self
    }

    pub fn autofocus(mut self, toggle: bool) -> Self {
        self.autofocus = match toggle {
            true => Some("autofocus".to_string()),
            false => None
        };
        self
    }

    pub fn disabled(mut self, toggle: bool) -> Self {
        self.disabled = match toggle {
            true => Some("disabled".to_string()),
            false => None
        };
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_string();
        self
    }

    // Button GETTERS.

    pub fn name(&self) -> &str {
        util::assigned_value(&self.name)
    }

    pub fn value(&self) -> &str {
        util::assigned_value(&self.value)
    }

    pub fn has_autofocus(&self) -> bool {
        match &self.autofocus {
            Some(_) => true,
            _ => false
        }
    }

    pub fn is_disabled(&self) -> bool {
        match &self.disabled {
            Some(_) => true,
            _ => false
        }
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
