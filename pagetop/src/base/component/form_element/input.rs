use crate::prelude::*;

pub_handle!(COMPONENT_INPUT);

#[derive(Default)]
pub enum InputType {
    #[default]
    Textfield,
    Password,
    Search,
    Email,
    Telephone,
    Url,
}

#[rustfmt::skip]
#[derive(Default)]
pub struct Input {
    weight      : isize,
    renderable  : Renderable,
    classes     : Classes,
    input_type  : InputType,
    name        : IdentifierValue,
    value       : AttributeValue,
    label       : AttributeValue,
    size        : Option<u16>,
    minlength   : Option<u16>,
    maxlength   : Option<u16>,
    placeholder : AttributeValue,
    autofocus   : AttributeValue,
    autocomplete: AttributeValue,
    disabled    : AttributeValue,
    readonly    : AttributeValue,
    required    : AttributeValue,
    help_text   : AttributeValue,
    template    : String,
}

impl ComponentTrait for Input {
    fn new() -> Self {
        Input::default()
            .with_classes(ClassesOp::SetDefault, "form-item")
            .with_classes(ClassesOp::AddFirst, "form-type-textfield")
            .with_size(Some(60))
            .with_maxlength(Some(128))
    }

    fn handle(&self) -> Handle {
        COMPONENT_INPUT
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, context: &PageContext) -> bool {
        (self.renderable.check)(context)
    }

    #[rustfmt::skip]
    fn default_render(&self, _: &mut PageContext) -> Markup {
        let type_input = match self.input_type() {
            InputType::Textfield => "text",
            InputType::Password  => "password",
            InputType::Search    => "search",
            InputType::Email     => "email",
            InputType::Telephone => "tel",
            InputType::Url       => "url",
        };
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
                    type=(type_input)
                    id=[id]
                    class="form-control"
                    name=[self.name().get()]
                    value=[self.value().get()]
                    size=[self.size()]
                    minlength=[self.minlength()]
                    maxlength=[self.maxlength()]
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

impl Input {
    pub fn textfield() -> Self {
        Input::new()
    }

    #[rustfmt::skip]
    pub fn password() -> Self {
        let mut input = Input::new().with_classes(
            ClassesOp::Replace("form-type-textfield"),
            "form-type-password",
        );
        input.input_type = InputType::Password;
        input
    }

    #[rustfmt::skip]
    pub fn search() -> Self {
        let mut input = Input::new().with_classes(
            ClassesOp::Replace("form-type-textfield"),
            "form-type-search",
        );
        input.input_type = InputType::Search;
        input
    }

    #[rustfmt::skip]
    pub fn email() -> Self {
        let mut input = Input::new().with_classes(
            ClassesOp::Replace("form-type-textfield"),
            "form-type-email"
        );
        input.input_type = InputType::Email;
        input
    }

    #[rustfmt::skip]
    pub fn telephone() -> Self {
        let mut input = Input::new().with_classes(
            ClassesOp::Replace("form-type-textfield"),
            "form-type-telephone",
        );
        input.input_type = InputType::Telephone;
        input
    }

    #[rustfmt::skip]
    pub fn url() -> Self {
        let mut input = Input::new().with_classes(
            ClassesOp::Replace("form-type-textfield"),
            "form-type-url"
        );
        input.input_type = InputType::Url;
        input
    }

    // Input BUILDER.

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

    pub fn with_size(mut self, size: Option<u16>) -> Self {
        self.alter_size(size);
        self
    }

    pub fn with_minlength(mut self, minlength: Option<u16>) -> Self {
        self.alter_minlength(minlength);
        self
    }

    pub fn with_maxlength(mut self, maxlength: Option<u16>) -> Self {
        self.alter_maxlength(maxlength);
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

    // Input ALTER.

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
        self.alter_classes(
            ClassesOp::SetDefault,
            concat_string!("form-item form-item-", name).as_str(),
        );
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

    pub fn alter_size(&mut self, size: Option<u16>) -> &mut Self {
        self.size = size;
        self
    }

    pub fn alter_minlength(&mut self, minlength: Option<u16>) -> &mut Self {
        self.minlength = minlength;
        self
    }

    pub fn alter_maxlength(&mut self, maxlength: Option<u16>) -> &mut Self {
        self.maxlength = maxlength;
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

    // Input GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn input_type(&self) -> &InputType {
        &self.input_type
    }

    pub fn name(&self) -> &IdentifierValue {
        &self.name
    }

    pub fn value(&self) -> &AttributeValue {
        &self.value
    }

    pub fn label(&self) -> &AttributeValue {
        &self.label
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
