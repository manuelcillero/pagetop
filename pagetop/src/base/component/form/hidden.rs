use crate::prelude::*;

#[rustfmt::skip]
#[derive(Default)]
pub struct Hidden {
    weight: Weight,
    name  : OptionName,
    value : OptionString,
}

impl_handle!(COMPONENT_BASE_HIDDEN for Hidden);

impl ComponentTrait for Hidden {
    fn new() -> Self {
        Hidden::default()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        let id = self.name().get().map(|name| concat_string!("value-", name));
        PrepareMarkup::With(html! {
            input type="hidden" id=[id] name=[self.name().get()] value=[self.value().get()] {}
        })
    }
}

impl Hidden {
    pub fn set(name: &str, value: &str) -> Self {
        Hidden::default().with_name(name).with_value(value)
    }

    // Hidden BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_builder]
    pub fn alter_name(&mut self, name: &str) -> &mut Self {
        self.name.alter_value(name);
        self
    }

    #[fn_builder]
    pub fn alter_value(&mut self, value: &str) -> &mut Self {
        self.value.alter_value(value);
        self
    }

    // Hidden GETTERS.

    pub fn name(&self) -> &OptionName {
        &self.name
    }

    pub fn value(&self) -> &OptionString {
        &self.value
    }
}
