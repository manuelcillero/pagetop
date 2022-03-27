use crate::prelude::*;

pub struct Date {
    renderable  : fn() -> bool,
    weight      : i8,
    name        : OptionAttr,
    value       : OptionAttr,
    label       : OptionAttr,
    placeholder : OptionAttr,
    autofocus   : OptionAttr,
    autocomplete: OptionAttr,
    disabled    : OptionAttr,
    readonly    : OptionAttr,
    required    : OptionAttr,
    help_text   : OptionAttr,
    template    : String,
}

impl PageComponent for Date {

    fn new() -> Self {
        Date {
            renderable  : always,
            weight      : 0,
            name        : OptionAttr::none(),
            value       : OptionAttr::none(),
            label       : OptionAttr::none(),
            placeholder : OptionAttr::none(),
            autofocus   : OptionAttr::none(),
            autocomplete: OptionAttr::none(),
            disabled    : OptionAttr::none(),
            readonly    : OptionAttr::none(),
            required    : OptionAttr::none(),
            help_text   : OptionAttr::none(),
            template    : "default".to_owned(),
        }
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> i8 {
        self.weight
    }

    fn default_render(&self, _: &mut PageAssets) -> Markup {
        let (class_item, id_item) = match self.name.option() {
            Some(name) => (
                format!("form-item form-item-{} form-type-date", name),
                Some(format!("edit-{}", name))
            ),
            None => (
                "form-item form-type-date".to_owned(),
                None
            )
        };
        html! {
            div class=(class_item) {
                @if self.label.has_value() {
                    label class="form-label" for=[&id_item] {
                        (self.label.value()) " "
                        @if self.required.has_value() {
                            span
                                class="form-required"
                                title="Este campo es obligatorio."
                            {
                                "*"
                            } " "
                        }
                    }
                }
                input
                    type="date"
                    id=[&id_item]
                    class="form-control"
                    name=[&self.name.option()]
                    value=[&self.value.option()]
                    placeholder=[&self.placeholder.option()]
                    autofocus=[&self.autofocus.option()]
                    autocomplete=[&self.autocomplete.option()]
                    readonly=[&self.readonly.option()]
                    required=[&self.required.option()]
                    disabled=[&self.disabled.option()];
                @if self.help_text.has_value() {
                    div class="form-text" {
                        (self.help_text.value())
                    }
                }
            }
        }
    }
}

impl Date {

    // Date BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.renderable = renderable;
        self
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name.with_value(name);
        self
    }

    pub fn with_value(mut self, value: &str) -> Self {
        self.value.with_value(value);
        self
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label.with_value(label);
        self
    }

    pub fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder.with_value(placeholder);
        self
    }

    pub fn autofocus(mut self, toggle: bool) -> Self {
        self.autofocus.with_value(match toggle {
            true => "autofocus",
            false => "",
        });
        self
    }

    pub fn autocomplete(mut self, toggle: bool) -> Self {
        self.autocomplete.with_value(match toggle {
            true => "",
            false => "off",
        });
        self
    }

    pub fn disabled(mut self, toggle: bool) -> Self {
        self.disabled.with_value(match toggle {
            true => "disabled",
            false => "",
        });
        self
    }

    pub fn readonly(mut self, toggle: bool) -> Self {
        self.readonly.with_value(match toggle {
            true => "readonly",
            false => "",
        });
        self
    }

    pub fn required(mut self, toggle: bool) -> Self {
        self.required.with_value(match toggle {
            true => "required",
            false => "",
        });
        self
    }

    pub fn with_help_text(mut self, help_text: &str) -> Self {
        self.help_text.with_value(help_text);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_owned();
        self
    }

    // Date GETTERS.

    pub fn name(&self) -> &str {
        self.name.value()
    }

    pub fn value(&self) -> &str {
        self.value.value()
    }

    pub fn label(&self) -> &str {
        self.label.value()
    }

    pub fn placeholder(&self) -> &str {
        self.placeholder.value()
    }

    pub fn has_autofocus(&self) -> bool {
        self.autofocus.has_value()
    }

    pub fn has_autocomplete(&self) -> bool {
        !self.autocomplete.has_value()
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled.has_value()
    }

    pub fn is_readonly(&self) -> bool {
        self.readonly.has_value()
    }

    pub fn is_required(&self) -> bool {
        self.required.has_value()
    }

    pub fn help_text(&self) -> &str {
        self.help_text.value()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
