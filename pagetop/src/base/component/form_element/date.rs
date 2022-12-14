use crate::prelude::*;

pub_handle!(COMPONENT_DATE);

#[rustfmt::skip]
#[derive(Default)]
pub struct Date {
    weight      : isize,
    renderable  : Renderable,
    classes     : Classes,
    name        : AttributeValue,
    value       : AttributeValue,
    label       : AttributeValue,
    placeholder : AttributeValue,
    autofocus   : AttributeValue,
    autocomplete: AttributeValue,
    disabled    : AttributeValue,
    readonly    : AttributeValue,
    required    : AttributeValue,
    help_text   : AttributeValue,
    template    : String,
}

impl ComponentTrait for Date {
    fn new() -> Self {
        Date::default()
            .with_classes(ClassesOp::SetDefault, "form-item")
            .with_classes(ClassesOp::AddFirst, "form-type-date")
    }

    fn handle(&self) -> Handle {
        COMPONENT_DATE
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rsx: &RenderResources) -> bool {
        (self.renderable.check)(rsx)
    }

    fn default_render(&self, _: &mut RenderResources) -> Markup {
        let id = self.name().get().map(|name| concat_string!("edit-", name));
        html! {
            div class=[self.classes().get()] {
                @if let Some(label) = self.label().get() {
                    label class="form-label" for=[&id] {
                        (label) " "
                        @if self.required().get().is_some() {
                            span
                                class="form-required"
                                title="Este campo es obligatorio." { "*" } " ";
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
                    disabled=[self.disabled().get()];
                @if let Some(help_text) = self.help_text().get() {
                    div class="form-text" { (help_text) }
                }
            }
        }
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl Date {
    // Date BUILDER.

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_renderable(mut self, check: IsRenderable) -> Self {
        self.alter_renderable(check);
        self
    }

    pub fn with_classes(mut self, op: ClassesOp, classes: &str) -> Self {
        self.alter_classes(op, classes);
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

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Date ALTER.

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    pub fn alter_name(&mut self, name: &str) -> &mut Self {
        self.name.alter_value(name);
        self
    }

    pub fn alter_value(&mut self, value: &str) -> &mut Self {
        self.value.alter_value(value);
        self
    }

    pub fn alter_label(&mut self, label: &str) -> &mut Self {
        self.label.alter_value(label);
        self
    }

    pub fn alter_placeholder(&mut self, placeholder: &str) -> &mut Self {
        self.placeholder.alter_value(placeholder);
        self
    }

    pub fn alter_autofocus(&mut self, toggle: bool) -> &mut Self {
        self.autofocus.alter_value(match toggle {
            true => "autofocus",
            false => "",
        });
        self
    }

    pub fn alter_autocomplete(&mut self, toggle: bool) -> &mut Self {
        self.autocomplete.alter_value(match toggle {
            true => "",
            false => "off",
        });
        self
    }

    pub fn alter_disabled(&mut self, toggle: bool) -> &mut Self {
        self.disabled.alter_value(match toggle {
            true => "disabled",
            false => "",
        });
        self
    }

    pub fn alter_readonly(&mut self, toggle: bool) -> &mut Self {
        self.readonly.alter_value(match toggle {
            true => "readonly",
            false => "",
        });
        self
    }

    pub fn alter_required(&mut self, toggle: bool) -> &mut Self {
        self.required.alter_value(match toggle {
            true => "required",
            false => "",
        });
        self
    }

    pub fn alter_help_text(&mut self, help_text: &str) -> &mut Self {
        self.help_text.alter_value(help_text);
        self
    }

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Date GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn name(&self) -> &AttributeValue {
        &self.name
    }

    pub fn value(&self) -> &AttributeValue {
        &self.value
    }

    pub fn label(&self) -> &AttributeValue {
        &self.label
    }

    pub fn placeholder(&self) -> &AttributeValue {
        &self.placeholder
    }

    pub fn autofocus(&self) -> &AttributeValue {
        &self.autofocus
    }

    pub fn autocomplete(&self) -> &AttributeValue {
        &self.autocomplete
    }

    pub fn disabled(&self) -> &AttributeValue {
        &self.disabled
    }

    pub fn readonly(&self) -> &AttributeValue {
        &self.readonly
    }

    pub fn required(&self) -> &AttributeValue {
        &self.required
    }

    pub fn help_text(&self) -> &AttributeValue {
        &self.help_text
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
