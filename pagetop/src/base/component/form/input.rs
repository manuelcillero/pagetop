use crate::prelude::*;

pub enum InputType {Email, Password, Search, Telephone, Textfield, Url}

pub struct Input {
    renderable  : fn() -> bool,
    weight      : i8,
    input_type  : InputType,
    name        : OptIden,
    value       : OptAttr,
    label       : OptAttr,
    size        : Option<u16>,
    minlength   : Option<u16>,
    maxlength   : Option<u16>,
    placeholder : OptAttr,
    autofocus   : OptAttr,
    autocomplete: OptAttr,
    disabled    : OptAttr,
    readonly    : OptAttr,
    required    : OptAttr,
    help_text   : OptAttr,
    classes     : Classes,
    template    : String,
}

impl PageComponent for Input {

    fn new() -> Self {
        Input {
            renderable  : always,
            weight      : 0,
            input_type  : InputType::Textfield,
            name        : OptIden::none(),
            value       : OptAttr::none(),
            label       : OptAttr::none(),
            size        : Some(60),
            minlength   : None,
            maxlength   : Some(128),
            placeholder : OptAttr::none(),
            autofocus   : OptAttr::none(),
            autocomplete: OptAttr::none(),
            disabled    : OptAttr::none(),
            readonly    : OptAttr::none(),
            required    : OptAttr::none(),
            help_text   : OptAttr::none(),
            classes     : Classes::none(),
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
        let (type_input, type_class) = match self.input_type() {
            InputType::Email     => ("email",    "form-type-email"),
            InputType::Password  => ("password", "form-type-password"),
            InputType::Search    => ("search",   "form-type-search"),
            InputType::Telephone => ("tel",      "form-type-telephone"),
            InputType::Textfield => ("text",     "form-type-textfield"),
            InputType::Url       => ("url",      "form-type-url")
        };
        let (class, id) = match self.name() {
            Some(name) => (
                concat_string!("form-item form-item-", name, " ", type_class),
                Some(concat_string!("edit-", name))
            ),
            None => (
                concat_string!("form-item ", type_class),
                None
            )
        };
        html! {
            div class=(class) {
                @match self.label() {
                    Some(label) => label class="form-label" for=[&id] {
                        (label) " "
                        @match self.required() {
                            Some(_) => span
                                class="form-required"
                                title="Este campo es obligatorio." { "*" } " ",
                            None => {}
                        }
                    },
                    None => {}
                }
                input
                    type=(type_input)
                    id=[id]
                    class="form-control"
                    name=[self.name()]
                    value=[self.value()]
                    size=[self.size()]
                    minlength=[self.minlength()]
                    maxlength=[self.maxlength()]
                    placeholder=[self.placeholder()]
                    autofocus=[self.autofocus()]
                    autocomplete=[self.autocomplete()]
                    readonly=[self.readonly()]
                    required=[self.required()]
                    disabled=[self.disabled()];
                @match self.help_text() {
                    Some(help_text) => div class="form-text" { (help_text) },
                    None => {}
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

    pub fn with_autofocus(mut self, toggle: bool) -> Self {
        self.autofocus.with_value(match toggle {
            true => "autofocus",
            false => "",
        });
        self
    }

    pub fn with_autocomplete(mut self, toggle: bool) -> Self {
        self.autocomplete.with_value(match toggle {
            true => "",
            false => "off",
        });
        self
    }

    pub fn with_disabled(mut self, toggle: bool) -> Self {
        self.disabled.with_value(match toggle {
            true => "disabled",
            false => "",
        });
        self
    }

    pub fn with_readonly(mut self, toggle: bool) -> Self {
        self.readonly.with_value(match toggle {
            true => "readonly",
            false => "",
        });
        self
    }

    pub fn with_required(mut self, toggle: bool) -> Self {
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

    pub fn set_classes(mut self, classes: &str) -> Self {
        self.classes.set_classes(classes);
        self
    }

    pub fn add_classes(mut self, classes: &str) -> Self {
        self.classes.add_classes(classes);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_owned();
        self
    }

    // Input GETTERS.

    pub fn input_type(&self) -> &InputType {
        &self.input_type
    }

    pub fn name(&self) -> &Option<String> {
        self.name.option()
    }

    pub fn value(&self) -> &Option<String> {
        self.value.option()
    }

    pub fn label(&self) -> &Option<String> {
        self.label.option()
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

    pub fn placeholder(&self) -> &Option<String> {
        self.placeholder.option()
    }

    pub fn autofocus(&self) -> &Option<String> {
        self.autofocus.option()
    }

    pub fn autocomplete(&self) -> &Option<String> {
        self.autocomplete.option()
    }

    pub fn disabled(&self) -> &Option<String> {
        self.disabled.option()
    }

    pub fn readonly(&self) -> &Option<String> {
        self.readonly.option()
    }

    pub fn required(&self) -> &Option<String> {
        self.required.option()
    }

    pub fn help_text(&self) -> &Option<String> {
        self.help_text.option()
    }

    pub fn classes(&self, default: &str) -> Option<String> {
        self.classes.option(default)
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
