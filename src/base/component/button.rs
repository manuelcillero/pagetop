use crate::prelude::*;

#[derive(AutoDefault)]
pub enum ButtonTarget {
    #[default]
    Default,
    Blank,
    Parent,
    Top,
    Context(String),
}

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct Button {
    id        : OptionId,
    classes   : OptionClasses,
    style     : StyleBase,
    font_size : FontSize,
    left_icon : OptionComponent<Icon>,
    right_icon: OptionComponent<Icon>,
    href      : OptionString,
    html      : OptionTranslated,
    target    : ButtonTarget,
}

impl ComponentTrait for Button {
    fn new() -> Self {
        Button::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
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

    #[rustfmt::skip]
    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let target = match &self.target() {
            ButtonTarget::Default => None,
            ButtonTarget::Blank   => Some("_blank"),
            ButtonTarget::Parent  => Some("_parent"),
            ButtonTarget::Top     => Some("_top"),
            ButtonTarget::Context(name) => Some(name.as_str()),
        };
        PrepareMarkup::With(html! {
            a
                id=[self.id()]
                class=[self.classes().get()]
                href=[self.href().get()]
                target=[target]
            {
                (self.left_icon().render(cx))
                span { (self.html().escaped(cx.langid())) }
                (self.right_icon().render(cx))
            }
        })
    }
}

impl Button {
    pub fn anchor(href: impl Into<String>, html: L10n) -> Self {
        Button::default().with_href(href).with_html(html)
    }

    // Button BUILDER.

    #[fn_builder]
    pub fn set_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.set_value(id);
        self
    }

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
    pub fn set_href(&mut self, href: impl Into<String>) -> &mut Self {
        self.href.set_value(href);
        self
    }

    #[fn_builder]
    pub fn set_html(&mut self, html: L10n) -> &mut Self {
        self.html.set_value(html);
        self
    }

    #[fn_builder]
    pub fn set_target(&mut self, target: ButtonTarget) -> &mut Self {
        self.target = target;
        self
    }

    // Button GETTERS.

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

    pub fn href(&self) -> &OptionString {
        &self.href
    }

    pub fn html(&self) -> &OptionTranslated {
        &self.html
    }

    pub fn target(&self) -> &ButtonTarget {
        &self.target
    }
}
