use pagetop::prelude::*;

use crate::theme::form;
use crate::LOCALES_BOOTSIER;

#[derive(AutoDefault, Getters)]
pub struct Input {
    classes: Classes,
    input_type: form::InputType,
    name: AttrName,
    value: AttrValue,
    label: Attr<L10n>,
    help_text: Attr<L10n>,
    #[default(_code = "Attr::<u16>::some(60)")]
    size: Attr<u16>,
    minlength: Attr<u16>,
    #[default(_code = "Attr::<u16>::some(128)")]
    maxlength: Attr<u16>,
    placeholder: AttrValue,
    autocomplete: Attr<form::Autocomplete>,
    autofocus: bool,
    readonly: bool,
    required: bool,
    disabled: bool,
}

impl Component for Input {
    fn new() -> Self {
        Self::default()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            util::join!("form-item form-type-", self.input_type().to_string()),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let id = self.name().get().map(|name| util::join!("edit-", name));
        PrepareMarkup::With(html! {
            div class=[self.classes().get()] {
                @if let Some(label) = self.label().lookup(cx) {
                    label for=[&id] class="form-label" {
                        (label)
                        @if *self.required() {
                            span
                                class="form-required"
                                title=(L10n::t("input_required", &LOCALES_BOOTSIER).using(cx))
                            {
                                "*"
                            }
                        }
                    }
                }
                input
                    type=(self.input_type())
                    id=[id]
                    class="form-control"
                    name=[self.name().get()]
                    value=[self.value().get()]
                    size=[self.size().get()]
                    minlength=[self.minlength().get()]
                    maxlength=[self.maxlength().get()]
                    placeholder=[self.placeholder().get()]
                    autocomplete=[self.autocomplete().get()]
                    autofocus[*self.autofocus()]
                    readonly[*self.readonly()]
                    required[*self.required()]
                    disabled[*self.disabled()] {}
                @if let Some(description) = self.help_text().lookup(cx) {
                    div class="form-text" { (description) }
                }
            }
        })
    }
}

impl Input {
    pub fn textfield() -> Self {
        Input::default()
    }

    pub fn password() -> Self {
        Self {
            input_type: form::InputType::Password,
            ..Default::default()
        }
    }

    pub fn search() -> Self {
        Self {
            input_type: form::InputType::Search,
            ..Default::default()
        }
    }

    pub fn email() -> Self {
        Self {
            input_type: form::InputType::Email,
            ..Default::default()
        }
    }

    pub fn telephone() -> Self {
        Self {
            input_type: form::InputType::Telephone,
            ..Default::default()
        }
    }

    pub fn url() -> Self {
        Self {
            input_type: form::InputType::Url,
            ..Default::default()
        }
    }

    // **< Input BUILDER >**************************************************************************

    /// Modifica la lista de clases CSS aplicadas al `input`.
    #[builder_fn]
    pub fn with_classes(mut self, op: ClassesOp, classes: impl AsRef<str>) -> Self {
        self.classes.alter_classes(op, classes);
        self
    }

    #[builder_fn]
    pub fn with_name(mut self, name: impl AsRef<str>) -> Self {
        self.name.alter_name(name);
        self
    }

    #[builder_fn]
    pub fn with_value(mut self, value: impl AsRef<str>) -> Self {
        self.value.alter_str(value);
        self
    }

    #[builder_fn]
    pub fn with_label(mut self, label: L10n) -> Self {
        self.label.alter_value(label);
        self
    }

    #[builder_fn]
    pub fn with_help_text(mut self, help_text: L10n) -> Self {
        self.help_text.alter_value(help_text);
        self
    }

    #[builder_fn]
    pub fn with_size(mut self, size: Option<u16>) -> Self {
        self.size.alter_opt(size);
        self
    }

    #[builder_fn]
    pub fn with_minlength(mut self, minlength: Option<u16>) -> Self {
        self.minlength.alter_opt(minlength);
        self
    }

    #[builder_fn]
    pub fn with_maxlength(mut self, maxlength: Option<u16>) -> Self {
        self.maxlength.alter_opt(maxlength);
        self
    }

    #[builder_fn]
    pub fn with_placeholder(mut self, placeholder: impl AsRef<str>) -> Self {
        self.placeholder.alter_str(placeholder);
        self
    }

    #[builder_fn]
    pub fn with_autocomplete(mut self, autocomplete: Option<form::Autocomplete>) -> Self {
        self.autocomplete.alter_opt(autocomplete);
        self
    }

    #[builder_fn]
    pub fn with_autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }

    #[builder_fn]
    pub fn with_readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    #[builder_fn]
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    #[builder_fn]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
