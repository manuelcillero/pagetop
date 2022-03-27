use crate::prelude::*;

enum InputType {Email, Password, Search, Telephone, Textfield, Url}

pub struct Input {
    renderable  : fn() -> bool,
    weight      : i8,
    input_type  : InputType,
    name        : OptionId,
    value       : OptionAttr,
    label       : OptionAttr,
    size        : Option<u16>,
    minlength   : Option<u16>,
    maxlength   : Option<u16>,
    placeholder : OptionAttr,
    autofocus   : OptionAttr,
    autocomplete: OptionAttr,
    disabled    : OptionAttr,
    readonly    : OptionAttr,
    required    : OptionAttr,
    help_text   : OptionAttr,
    template    : String,
}

impl PageComponent for Input {

    fn new() -> Self {
        Input {
            renderable  : always,
            weight      : 0,
            input_type  : InputType::Textfield,
            name        : OptionId::none(),
            value       : OptionAttr::none(),
            label       : OptionAttr::none(),
            size        : Some(60),
            minlength   : None,
            maxlength   : Some(128),
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
        let (input_type, class_type) = match &self.input_type {
            InputType::Email     => ("email",    "form-type-email"),
            InputType::Password  => ("password", "form-type-password"),
            InputType::Search    => ("search",   "form-type-search"),
            InputType::Telephone => ("tel",      "form-type-telephone"),
            InputType::Textfield => ("text",     "form-type-textfield"),
            InputType::Url       => ("url",      "form-type-url")
        };
        let (class_item, id_item) = match &self.name.option() {
            Some(name) => (
                format!("form-item form-item-{} {}", name, class_type),
                Some(format!("edit-{}", name))
            ),
            None => (
                format!("form-item {}", class_type),
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
                    type=(input_type)
                    id=[&id_item]
                    class="form-control"
                    name=[&self.name.option()]
                    value=[&self.value.option()]
                    size=[self.size]
                    minlength=[self.minlength]
                    maxlength=[self.maxlength]
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

impl Input {

    pub fn textfield() -> Self {
        Input::new()
    }

    pub fn password() -> Self {
        let mut input = Input::new();
        input.input_type = InputType::Password;
        input
    }

    pub fn search() -> Self {
        let mut input = Input::new();
        input.input_type = InputType::Search;
        input
    }

    pub fn email() -> Self {
        let mut input = Input::new();
        input.input_type = InputType::Email;
        input
    }

    pub fn telephone() -> Self {
        let mut input = Input::new();
        input.input_type = InputType::Telephone;
        input
    }

    pub fn url() -> Self {
        let mut input = Input::new();
        input.input_type = InputType::Url;
        input
    }

    // Input BUILDER.

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

    pub fn with_size(mut self, size: Option<u16>) -> Self {
        self.size = size;
        self
    }

    pub fn with_minlength(mut self, minlength: Option<u16>) -> Self {
        self.minlength = minlength;
        self
    }

    pub fn with_maxlength(mut self, maxlength: Option<u16>) -> Self {
        self.maxlength = maxlength;
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

    // Input GETTERS.

    pub fn name(&self) -> &str {
        self.name.value()
    }

    pub fn value(&self) -> &str {
        self.value.value()
    }

    pub fn label(&self) -> &str {
        self.label.value()
    }

    pub fn size(&self) -> Option<u16> {
        self.size
    }

    pub fn minlength(&self) -> Option<u16> {
        self.minlength
    }

    pub fn maxlength(&self) -> Option<u16> {
        self.maxlength
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
