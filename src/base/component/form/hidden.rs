use crate::prelude::*;

pub struct Hidden {
    renderable  : fn() -> bool,
    weight      : i8,
    name        : Option<String>,
    value       : Option<String>,
    template    : String,
}

impl PageComponent for Hidden {

    fn prepare() -> Self {
        Hidden {
            renderable  : always,
            weight      : 0,
            name        : None,
            value       : None,
            template    : "default".to_string(),
        }
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> i8 {
        self.weight
    }

    fn default_render(&self, _: &mut PageAssets) -> Markup {
        let id_item = match &self.name {
            Some(name) => Some(format!("value-{}", name)),
            _ => None
        };
        html! {
            input
                type="hidden"
                id=[&id_item]
                name=[&self.name]
                value=[&self.value];
        }
    }
}

impl Hidden {

    pub fn set(name: &str, value: &str) -> Self {
        Hidden::prepare().with_name(name).with_value(value)
    }

    // Hidden BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.renderable = renderable;
        self
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = if name.is_empty() {
            None
        } else {
            Some(name.replace(" ", "_"))
        };
        self
    }

    pub fn with_value(mut self, value: &str) -> Self {
        self.value = if value.is_empty() {
            None
        } else {
            Some(value.to_string())
        };
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_string();
        self
    }

    // Hidden GETTERS.

    pub fn name(&self) -> &str {
        match &self.name {
            Some(name) => name.as_str(),
            _ => ""
        }
    }

    pub fn value(&self) -> &str {
        match &self.value {
            Some(value) => value.as_str(),
            _ => ""
        }
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
