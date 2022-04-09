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
    classes     : Classes,
    template    : String,
}

impl PageComponent for Date {
    fn new() -> Self {
        Date {
            renderable  : render_always,
            weight      : 0,
            name        : OptAttr::new(),
            value       : OptAttr::new(),
            label       : OptAttr::new(),
            placeholder : OptAttr::new(),
            autofocus   : OptAttr::new(),
            autocomplete: OptAttr::new(),
            disabled    : OptAttr::new(),
            readonly    : OptAttr::new(),
            required    : OptAttr::new(),
            help_text   : OptAttr::new(),
            classes     : Classes::new_with_default("form-item"),
            template    : "default".to_owned(),
        }
        .with_classes("form-type-date", ClassesOp::AddFirst)
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> i8 {
        self.weight
    }

    fn default_render(&self, _: &mut PageAssets) -> Markup {
        let id = match self.name() {
            Some(name) => Some(concat_string!("edit-", name)),
            None => None,
        };
        html! {
            div class=[self.classes()] {
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
        self.alter_renderable(renderable);
        self
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
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

    pub fn with_label(mut self, label: &str) -> Self {
        self.alter_label(label);
        self
    }

    pub fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.alter_placeholder(placeholder);
        self
    }

    pub fn with_autofocus(mut self, toggle: bool) -> Self {
        self.alter_autofocus(toggle);
        self
    }

    pub fn with_autocomplete(mut self, toggle: bool) -> Self {
        self.alter_autocomplete(toggle);
        self
    }

    pub fn with_disabled(mut self, toggle: bool) -> Self {
        self.alter_disabled(toggle);
        self
    }

    pub fn with_readonly(mut self, toggle: bool) -> Self {
        self.alter_readonly(toggle);
        self
    }

    pub fn with_required(mut self, toggle: bool) -> Self {
        self.alter_required(toggle);
        self
    }

    pub fn with_help_text(mut self, help_text: &str) -> Self {
        self.alter_help_text(help_text);
        self
    }

    pub fn with_classes(mut self, classes: &str, op: ClassesOp) -> Self {
        self.alter_classes(classes, op);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Date ALTER.

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_weight(&mut self, weight: i8) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_name(&mut self, name: &str) -> &mut Self {
        self.name.with_value(name);
        self
    }

    pub fn alter_value(&mut self, value: &str) -> &mut Self {
        self.value.with_value(value);
        self
    }

    pub fn alter_label(&mut self, label: &str) -> &mut Self {
        self.label.with_value(label);
        self
    }

    pub fn alter_placeholder(&mut self, placeholder: &str) -> &mut Self {
        self.placeholder.with_value(placeholder);
        self
    }

    pub fn alter_autofocus(&mut self, toggle: bool) -> &mut Self {
        self.autofocus.with_value(match toggle {
            true => "autofocus",
            false => "",
        });
        self
    }

    pub fn alter_autocomplete(&mut self, toggle: bool) -> &mut Self {
        self.autocomplete.with_value(match toggle {
            true => "",
            false => "off",
        });
        self
    }

    pub fn alter_disabled(&mut self, toggle: bool) -> &mut Self {
        self.disabled.with_value(match toggle {
            true => "disabled",
            false => "",
        });
        self
    }

    pub fn alter_readonly(&mut self, toggle: bool) -> &mut Self {
        self.readonly.with_value(match toggle {
            true => "readonly",
            false => "",
        });
        self
    }

    pub fn alter_required(&mut self, toggle: bool) -> &mut Self {
        self.required.with_value(match toggle {
            true => "required",
            false => "",
        });
        self
    }

    pub fn alter_help_text(&mut self, help_text: &str) -> &mut Self {
        self.help_text.with_value(help_text);
        self
    }

    pub fn alter_classes(&mut self, classes: &str, op: ClassesOp) -> &mut Self {
        self.classes.alter(classes, op);
        self
    }

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
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

    pub fn classes(&self) -> &Option<String> {
        self.classes.option()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
