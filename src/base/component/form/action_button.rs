use crate::prelude::*;

use std::fmt;

#[derive(AutoDefault)]
pub enum ActionButtonType {
    #[default]
    Submit,
    Reset,
}

#[rustfmt::skip]
impl fmt::Display for ActionButtonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionButtonType::Submit => write!(f, "submit"),
            ActionButtonType::Reset  => write!(f, "reset"),
        }
    }
}

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct ActionButton {
    classes    : OptionClasses,
    button_type: ActionButtonType,
    style      : StyleBase,
    font_size  : FontSize,
    left_icon  : OptionComponent<Icon>,
    right_icon : OptionComponent<Icon>,
    name       : OptionString,
    value      : OptionTranslated,
    autofocus  : OptionString,
    disabled   : OptionString,
}

impl ComponentTrait for ActionButton {
    fn new() -> Self {
        ActionButton::submit()
    }

    fn setup_before_prepare(&mut self, _cx: &mut Context) {
        self.set_classes(
            ClassesOp::Prepend,
            [
                "button__tap".to_string(),
                self.style().to_string(),
                self.font_size().to_string(),
            ]
            .join(" "),
        );
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let id = self.name().get().map(|name| concat_string!("edit-", name));
        PrepareMarkup::With(html! {
            button
                type=(self.button_type().to_string())
                id=[id]
                class=[self.classes().get()]
                name=[self.name().get()]
                value=[self.value().using(cx.langid())]
                autofocus=[self.autofocus().get()]
                disabled=[self.disabled().get()]
            {
                (self.left_icon().render(cx))
                span { (self.value().escaped(cx.langid())) }
                (self.right_icon().render(cx))
            }
        })
    }
}

impl ActionButton {
    pub fn submit() -> Self {
        ActionButton {
            button_type: ActionButtonType::Submit,
            style: StyleBase::Default,
            value: OptionTranslated::new(L10n::l("button_submit")),
            ..Default::default()
        }
    }

    pub fn reset() -> Self {
        ActionButton {
            button_type: ActionButtonType::Reset,
            style: StyleBase::Info,
            value: OptionTranslated::new(L10n::l("button_reset")),
            ..Default::default()
        }
    }

    // Button BUILDER.

    #[fn_builder]
    pub fn set_style(&mut self, style: StyleBase) -> &mut Self {
        self.style = style;
        self
    }

    #[fn_builder]
    pub fn set_font_size(&mut self, font_size: FontSize) -> &mut Self {
        self.font_size = font_size;
        self
    }

    #[fn_builder]
    pub fn set_left_icon(&mut self, icon: Option<Icon>) -> &mut Self {
        self.left_icon.set_value(icon);
        self
    }

    #[fn_builder]
    pub fn set_right_icon(&mut self, icon: Option<Icon>) -> &mut Self {
        self.right_icon.set_value(icon);
        self
    }

    #[fn_builder]
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name.set_value(name);
        self
    }

    #[fn_builder]
    pub fn set_value(&mut self, value: L10n) -> &mut Self {
        self.value.set_value(value);
        self
    }

    #[fn_builder]
    pub fn set_autofocus(&mut self, toggle: bool) -> &mut Self {
        self.autofocus
            .set_value(if toggle { "autofocus" } else { "" });
        self
    }

    #[fn_builder]
    pub fn set_disabled(&mut self, toggle: bool) -> &mut Self {
        self.disabled
            .set_value(if toggle { "disabled" } else { "" });
        self
    }

    // Button GETTERS.

    pub fn button_type(&self) -> &ActionButtonType {
        &self.button_type
    }

    pub fn style(&self) -> &StyleBase {
        &self.style
    }

    pub fn font_size(&self) -> &FontSize {
        &self.font_size
    }

    pub fn left_icon(&self) -> &OptionComponent<Icon> {
        &self.left_icon
    }

    pub fn right_icon(&self) -> &OptionComponent<Icon> {
        &self.right_icon
    }

    pub fn name(&self) -> &OptionString {
        &self.name
    }

    pub fn value(&self) -> &OptionTranslated {
        &self.value
    }

    pub fn autofocus(&self) -> &OptionString {
        &self.autofocus
    }

    pub fn disabled(&self) -> &OptionString {
        &self.disabled
    }
}
