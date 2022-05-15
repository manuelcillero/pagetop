use crate::prelude::*;

pub const HIDDEN_COMPONENT: &str = "pagetop::component::form::hidden";

pub struct Hidden {
    weight: isize,
    name  : OptIden,
    value : OptAttr,
}

impl ComponentTrait for Hidden {
    fn new() -> Self {
        Hidden {
            weight: 0,
            name  : OptIden::new(),
            value : OptAttr::new(),
        }
    }

    fn handler(&self) -> &'static str {
        HIDDEN_COMPONENT
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn default_render(&self, _: &mut Context) -> Markup {
        let id = match self.name() {
            Some(name) => Some(concat_string!("value-", name)),
            _ => None
        };
        html! {
            input type="hidden" id=[id] name=[self.name()] value=[self.value()];
        }
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl Hidden {
    pub fn set(name: &str, value: &str) -> Self {
        Hidden::new().with_name(name).with_value(value)
    }

    // Hidden BUILDER.

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

    // Hidden ALTER.

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

    // Hidden GETTERS.

    pub fn name(&self) -> &Option<String> {
        self.name.option()
    }

    pub fn value(&self) -> &Option<String> {
        self.value.option()
    }
}
