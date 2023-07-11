use pagetop::prelude::*;

use_handle!(COMPONENT_INPUT);

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

type InputLabel = OneComponent<L10n>;
type InputHelpText = OneComponent<L10n>;

#[rustfmt::skip]
#[derive(Default)]
pub struct Input {
    weight      : isize,
    renderable  : Renderable,
    classes     : Classes,
    input_type  : InputType,
    name        : NameValue,
    value       : AttributeValue,
    label       : InputLabel,
    size        : Option<u16>,
    minlength   : Option<u16>,
    maxlength   : Option<u16>,
    placeholder : AttributeValue,
    autofocus   : AttributeValue,
    autocomplete: AttributeValue,
    disabled    : AttributeValue,
    readonly    : AttributeValue,
    required    : AttributeValue,
    help_text   : InputHelpText,
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

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    #[rustfmt::skip]
    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let type_input = match self.input_type() {
            InputType::Textfield => "text",
            InputType::Password  => "password",
            InputType::Search    => "search",
            InputType::Email     => "email",
            InputType::Telephone => "tel",
            InputType::Url       => "url",
        };
        let id = self.name().get().map(|name| concat_string!("edit-", name));
        let label = self.label().prepare(cx);
        let description = self.help_text().prepare(cx);
        PrepareMarkup::With(html! {
            div class=[self.classes().get()] {
                @if !label.is_empty() {
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
                @if !description.is_empty() {
                    div class="form-text" { (description) }
                }
            }
        })
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

    #[fn_builder]
    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_name(&mut self, name: &str) -> &mut Self {
        self.name.alter_value(name);
        self.alter_classes(
            ClassesOp::SetDefault,
            concat_string!("form-item form-item-", name).as_str(),
        );
        self
    }

    #[fn_builder]
    pub fn alter_value(&mut self, value: &str) -> &mut Self {
        self.value.alter_value(value);
        self
    }

    #[fn_builder]
    pub fn alter_label(&mut self, label: L10n) -> &mut Self {
        self.label.set(label);
        self
    }

    #[fn_builder]
    pub fn alter_size(&mut self, size: Option<u16>) -> &mut Self {
        self.size = size;
        self
    }

    #[fn_builder]
    pub fn alter_minlength(&mut self, minlength: Option<u16>) -> &mut Self {
        self.minlength = minlength;
        self
    }

    #[fn_builder]
    pub fn alter_maxlength(&mut self, maxlength: Option<u16>) -> &mut Self {
        self.maxlength = maxlength;
        self
    }

    #[fn_builder]
    pub fn alter_placeholder(&mut self, placeholder: &str) -> &mut Self {
        self.placeholder.alter_value(placeholder);
        self
    }

    #[fn_builder]
    pub fn alter_autofocus(&mut self, toggle: bool) -> &mut Self {
        self.autofocus.alter_value(match toggle {
            true => "autofocus",
            false => "",
        });
        self
    }

    #[fn_builder]
    pub fn alter_autocomplete(&mut self, toggle: bool) -> &mut Self {
        self.autocomplete.alter_value(match toggle {
            true => "",
            false => "off",
        });
        self
    }

    #[fn_builder]
    pub fn alter_disabled(&mut self, toggle: bool) -> &mut Self {
        self.disabled.alter_value(match toggle {
            true => "disabled",
            false => "",
        });
        self
    }

    #[fn_builder]
    pub fn alter_readonly(&mut self, toggle: bool) -> &mut Self {
        self.readonly.alter_value(match toggle {
            true => "readonly",
            false => "",
        });
        self
    }

    #[fn_builder]
    pub fn alter_required(&mut self, toggle: bool) -> &mut Self {
        self.required.alter_value(match toggle {
            true => "required",
            false => "",
        });
        self
    }

    #[fn_builder]
    pub fn alter_help_text(&mut self, help_text: L10n) -> &mut Self {
        self.help_text.set(help_text);
        self
    }

    #[fn_builder]
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

    pub fn name(&self) -> &NameValue {
        &self.name
    }

    pub fn value(&self) -> &AttributeValue {
        &self.value
    }

    pub fn label(&self) -> &InputLabel {
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

    pub fn help_text(&self) -> &InputHelpText {
        &self.help_text
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
