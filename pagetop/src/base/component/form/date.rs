use crate::prelude::*;

pub struct Date {
    renderable  : fn() -> bool,
    weight      : i8,
    name        : OptAttr,
    value       : OptAttr,
    label       : OptAttr,
    placeholder : OptAttr,
    autofocus   : OptAttr,
    autocomplete: OptAttr,
    disabled    : OptAttr,
    readonly    : OptAttr,
    required    : OptAttr,
    help_text   : OptAttr,
    template    : String,
}

impl PageComponent for Date {

    fn new() -> Self {
        Date {
            renderable  : always,
            weight      : 0,
            name        : OptAttr::none(),
            value       : OptAttr::none(),
            label       : OptAttr::none(),
            placeholder : OptAttr::none(),
            autofocus   : OptAttr::none(),
            autocomplete: OptAttr::none(),
            disabled    : OptAttr::none(),
            readonly    : OptAttr::none(),
            required    : OptAttr::none(),
            help_text   : OptAttr::none(),
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
        let (class, id) = match self.name() {
            Some(name) => (
                concat_string!("form-item form-item-", name, " form-type-date"),
                Some(concat_string!("edit-", name))
            ),
            None => (
                "form-item form-type-date".to_owned(),
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
                    type="date"
                    id=[id]
                    class="form-control"
                    name=[self.name()]
                    value=[self.value()]
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

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_owned();
        self
    }

    // Date GETTERS.

    pub fn name(&self) -> &Option<String> {
        self.name.option()
    }

    pub fn value(&self) -> &Option<String> {
        self.value.option()
    }

    pub fn label(&self) -> &Option<String> {
        self.label.option()
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

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
