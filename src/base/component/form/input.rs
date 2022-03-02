use crate::prelude::*;

enum InputType {Email, Password, Search, Telephone, Textfield, Url}

pub struct Input {
    renderable  : fn() -> bool,
    weight      : i8,
    input_type  : InputType,
    name        : Option<String>,
    value       : Option<String>,
    label       : Option<String>,
    size        : Option<u16>,
    minlength   : Option<u16>,
    maxlength   : Option<u16>,
    placeholder : Option<String>,
    autofocus   : Option<String>,
    autocomplete: Option<String>,
    disabled    : Option<String>,
    readonly    : Option<String>,
    required    : Option<String>,
    help_text   : Option<String>,
    template    : String,
}

impl PageComponent for Input {

    fn prepare() -> Self {
        Input {
            renderable  : always,
            weight      : 0,
            input_type  : InputType::Textfield,
            name        : None,
            value       : None,
            label       : None,
            size        : Some(60),
            minlength   : None,
            maxlength   : Some(128),
            placeholder : None,
            autofocus   : None,
            autocomplete: None,
            disabled    : None,
            readonly    : None,
            required    : None,
            help_text   : None,
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
        let (class_item, id_item) = match &self.name {
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
                @if self.label != None {
                    label class="form-label" for=[&id_item] {
                        (self.label()) " "
                        @if self.required != None {
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
                    name=[&self.name]
                    value=[&self.value]
                    size=[self.size]
                    minlength=[self.minlength]
                    maxlength=[self.maxlength]
                    placeholder=[&self.placeholder]
                    autofocus=[&self.autofocus]
                    autocomplete=[&self.autocomplete]
                    readonly=[&self.readonly]
                    required=[&self.required]
                    disabled=[&self.disabled];
                @if self.help_text != None {
                    div class="form-text" {
                        (self.help_text())
                    }
                }
            }
        }
    }
}

impl Input {

    pub fn textfield() -> Self {
        Input::prepare()
    }

    pub fn password() -> Self {
        let mut input = Input::prepare();
        input.input_type = InputType::Password;
        input
    }

    pub fn search() -> Self {
        let mut input = Input::prepare();
        input.input_type = InputType::Search;
        input
    }

    pub fn email() -> Self {
        let mut input = Input::prepare();
        input.input_type = InputType::Email;
        input
    }

    pub fn telephone() -> Self {
        let mut input = Input::prepare();
        input.input_type = InputType::Telephone;
        input
    }

    pub fn url() -> Self {
        let mut input = Input::prepare();
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
        self.name = util::valid_id(name);
        self
    }

    pub fn with_value(mut self, value: &str) -> Self {
        self.value = util::optional_str(value);
        self
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = util::optional_str(label);
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
        self.placeholder = util::optional_str(placeholder);
        self
    }

    pub fn autofocus(mut self, toggle: bool) -> Self {
        self.autofocus = match toggle {
            true => Some("autofocus".to_owned()),
            false => None
        };
        self
    }

    pub fn autocomplete(mut self, toggle: bool) -> Self {
        self.autocomplete = match toggle {
            true => None,
            false => Some("off".to_owned())
        };
        self
    }

    pub fn disabled(mut self, toggle: bool) -> Self {
        self.disabled = match toggle {
            true => Some("disabled".to_owned()),
            false => None
        };
        self
    }

    pub fn readonly(mut self, toggle: bool) -> Self {
        self.readonly = match toggle {
            true => Some("readonly".to_owned()),
            false => None
        };
        self
    }

    pub fn required(mut self, toggle: bool) -> Self {
        self.required = match toggle {
            true => Some("required".to_owned()),
            false => None
        };
        self
    }

    pub fn with_help_text(mut self, help_text: &str) -> Self {
        self.help_text = util::optional_str(help_text);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_owned();
        self
    }

    // Input GETTERS.

    pub fn name(&self) -> &str {
        util::assigned_str(&self.name)
    }

    pub fn value(&self) -> &str {
        util::assigned_str(&self.value)
    }

    pub fn label(&self) -> &str {
        util::assigned_str(&self.label)
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
        util::assigned_str(&self.placeholder)
    }

    pub fn has_autofocus(&self) -> bool {
        match &self.autofocus {
            Some(_) => true,
            _ => false
        }
    }

    pub fn has_autocomplete(&self) -> bool {
        match &self.autocomplete {
            Some(_) => false,
            _ => true
        }
    }

    pub fn is_disabled(&self) -> bool {
        match &self.disabled {
            Some(_) => true,
            _ => false
        }
    }

    pub fn is_readonly(&self) -> bool {
        match &self.readonly {
            Some(_) => true,
            _ => false
        }
    }

    pub fn is_required(&self) -> bool {
        match &self.required {
            Some(_) => true,
            _ => false
        }
    }

    pub fn help_text(&self) -> &str {
        util::assigned_str(&self.help_text)
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
