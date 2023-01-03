use crate::prelude::*;

pub_handle!(COMPONENT_HIDDEN);

#[rustfmt::skip]
#[derive(Default)]
pub struct Hidden {
    weight: isize,
    name  : NameValue,
    value : AttributeValue,
}

impl ComponentTrait for Hidden {
    fn new() -> Self {
        Hidden::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_HIDDEN
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn default_render(&self, _: &mut RenderContext) -> Markup {
        let id = self.name().get().map(|name| concat_string!("value-", name));
        html! {
            input type="hidden" id=[id] name=[self.name().get()] value=[self.value().get()];
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
        self.name.alter_value(name);
        self
    }

    pub fn alter_value(&mut self, value: &str) -> &mut Self {
        self.value.alter_value(value);
        self
    }

    // Hidden GETTERS.

    pub fn name(&self) -> &NameValue {
        &self.name
    }

    pub fn value(&self) -> &AttributeValue {
        &self.value
    }
}
