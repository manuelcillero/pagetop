use crate::prelude::*;

pub struct Hidden {
    weight    : i8,
    name      : Option<String>,
    value     : Option<String>,
}

impl PageComponent for Hidden {

    fn prepare() -> Self {
        Hidden {
            weight    : 0,
            name      : None,
            value     : None,
        }
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

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = util::valid_id(name);
        self
    }

    pub fn with_value(mut self, value: &str) -> Self {
        self.value = util::valid_str(value);
        self
    }

    // Hidden GETTERS.

    pub fn name(&self) -> &str {
        util::assigned_str(&self.name)
    }

    pub fn value(&self) -> &str {
        util::assigned_str(&self.value)
    }
}
