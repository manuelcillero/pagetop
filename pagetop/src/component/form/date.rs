use crate::prelude::*;

pub struct Date {
    renderable  : fn() -> bool,
    weight      : i8,
    name        : Option<String>,
    value       : Option<String>,
    label       : Option<String>,
    placeholder : Option<String>,
    autofocus   : Option<String>,
    autocomplete: Option<String>,
    disabled    : Option<String>,
    readonly    : Option<String>,
    required    : Option<String>,
    help_text   : Option<String>,
    template    : String,
}

impl PageComponent for Date {

    fn prepare() -> Self {
        Date {
            renderable  : always,
            weight      : 0,
            name        : None,
            value       : None,
            label       : None,
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
        let (class_item, id_item) = match &self.name {
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
                    type="date"
                    id=[&id_item]
                    class="form-control"
                    name=[&self.name]
                    value=[&self.value]
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

    // Date GETTERS.

    pub fn name(&self) -> &str {
        util::assigned_str(&self.name)
    }

    pub fn value(&self) -> &str {
        util::assigned_str(&self.value)
    }

    pub fn label(&self) -> &str {
        util::assigned_str(&self.label)
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
