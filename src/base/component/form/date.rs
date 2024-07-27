use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct Date {
    classes     : OptionClasses,
    name        : OptionString,
    value       : OptionString,
    label       : OptionString,
    placeholder : OptionString,
    autofocus   : OptionString,
    autocomplete: OptionString,
    disabled    : OptionString,
    readonly    : OptionString,
    required    : OptionString,
    help_text   : OptionString,
}

impl ComponentTrait for Date {
    fn new() -> Self {
        Date::default().with_classes(ClassesOp::Add, "form-item form-type-date")
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        let id = self.name().get().map(|name| concat_string!("edit-", name));
        PrepareMarkup::With(html! {
            div class=[self.classes().get()] {
                @if let Some(label) = self.label().get() {
                    label class="form-label" for=[&id] {
                        (label) " "
                        @if self.required().get().is_some() {
                            span
                                class="form-required"
                                title="Este campo es obligatorio." { "*" } " "
                        }
                    }
                }
                input
                    type="date"
                    id=[id]
                    class="form-control"
                    name=[self.name().get()]
                    value=[self.value().get()]
                    placeholder=[self.placeholder().get()]
                    autofocus=[self.autofocus().get()]
                    autocomplete=[self.autocomplete().get()]
                    readonly=[self.readonly().get()]
                    required=[self.required().get()]
                    disabled=[self.disabled().get()] {}
                @if let Some(help_text) = self.help_text().get() {
                    div class="form-text" { (help_text) }
                }
            }
        })
    }
}

impl Date {
    // Date BUILDER.

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

    #[fn_builder]
    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label.set_value(label);
        self
    }

    #[fn_builder]
    pub fn set_placeholder(&mut self, placeholder: &str) -> &mut Self {
        self.placeholder.set_value(placeholder);
        self
    }

    #[fn_builder]
    pub fn set_autofocus(&mut self, toggle: bool) -> &mut Self {
        self.autofocus
            .set_value(if toggle { "autofocus" } else { "" });
        self
    }

    #[fn_builder]
    pub fn set_autocomplete(&mut self, toggle: bool) -> &mut Self {
        self.autocomplete.set_value(if toggle { "" } else { "off" });
        self
    }

    #[fn_builder]
    pub fn set_disabled(&mut self, toggle: bool) -> &mut Self {
        self.disabled
            .set_value(if toggle { "disabled" } else { "" });
        self
    }

    #[fn_builder]
    pub fn set_readonly(&mut self, toggle: bool) -> &mut Self {
        self.readonly
            .set_value(if toggle { "readonly" } else { "" });
        self
    }

    #[fn_builder]
    pub fn set_required(&mut self, toggle: bool) -> &mut Self {
        self.required
            .set_value(if toggle { "required" } else { "" });
        self
    }

    #[fn_builder]
    pub fn set_help_text(&mut self, help_text: &str) -> &mut Self {
        self.help_text.set_value(help_text);
        self
    }

    // Date GETTERS.

    pub fn name(&self) -> &OptionString {
        &self.name
    }

    pub fn value(&self) -> &OptionString {
        &self.value
    }

    pub fn label(&self) -> &OptionString {
        &self.label
    }

    pub fn placeholder(&self) -> &OptionString {
        &self.placeholder
    }

    pub fn autofocus(&self) -> &OptionString {
        &self.autofocus
    }

    pub fn autocomplete(&self) -> &OptionString {
        &self.autocomplete
    }

    pub fn disabled(&self) -> &OptionString {
        &self.disabled
    }

    pub fn readonly(&self) -> &OptionString {
        &self.readonly
    }

    pub fn required(&self) -> &OptionString {
        &self.required
    }

    pub fn help_text(&self) -> &OptionString {
        &self.help_text
    }
}
