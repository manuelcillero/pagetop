use crate::prelude::*;

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct Icon {
    weight    : Weight,
    renderable: Renderable,
    classes   : OptionClasses,
    icon_name : OptionString,
    font_size : FontSize,
}

impl ComponentTrait for Icon {
    fn new() -> Self {
        Icon::default()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    #[rustfmt::skip]
    fn setup_before_prepare(&mut self, cx: &mut Context) {
        if let Some(icon_name) = self.icon_name().get() {
            self.prepend_classes(
                concat_string!("bi-", icon_name, " ", self.font_size().to_string()),
            );
            cx.set_param::<bool>(PARAM_BASE_INCLUDE_ICONS, true);
        }
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        match self.icon_name().get() {
            None => PrepareMarkup::None,
            _ => PrepareMarkup::With(html! { i class=[self.classes().get()] {} }),
        }
    }
}

impl Icon {
    pub fn with(icon_name: impl Into<String>) -> Self {
        Icon::default().with_icon_name(icon_name)
    }

    // Icon BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: FnIsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_icon_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.icon_name.alter_value(name);
        self
    }

    #[fn_builder]
    pub fn alter_font_size(&mut self, font_size: FontSize) -> &mut Self {
        self.font_size = font_size;
        self
    }

    // Icon GETTERS.

    pub fn icon_name(&self) -> &OptionString {
        &self.icon_name
    }

    pub fn font_size(&self) -> &FontSize {
        &self.font_size
    }
}
