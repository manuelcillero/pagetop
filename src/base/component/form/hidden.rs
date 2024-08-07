use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Hidden {
    name  : OptionName,
    value : OptionString,
}

impl ComponentTrait for Hidden {
    fn new() -> Self {
        Hidden::default()
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
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name.set_value(name);
        self
    }

    #[fn_builder]
    pub fn set_value(&mut self, value: &str) -> &mut Self {
        self.value.set_value(value);
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
