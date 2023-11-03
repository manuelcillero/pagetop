use crate::prelude::*;

#[rustfmt::skip]
#[derive(Default)]
pub struct Icon {
    weight    : Weight,
    renderable: Renderable,
    classes   : OptionClasses,
    font_size : FontSize,
    icon_name : String,
}

impl_handle!(COMPONENT_BASE_ICON for Icon);

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

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        if self.icon_name().is_empty() {
            return PrepareMarkup::None;
        }
        cx.set_param::<bool>(PARAM_BASE_INCLUDE_ICONS, true);
        PrepareMarkup::With(html! { i class=[self.classes().get()] {} })
    }
}

impl Icon {
    pub fn with(icon_name: &str) -> Self {
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
    pub fn alter_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_font_size(&mut self, font_size: FontSize) -> &mut Self {
        self.classes.alter_value(
            ClassesOp::Replace(self.font_size.to_string()),
            font_size.to_string(),
        );
        self.font_size = font_size;
        self
    }

    #[fn_builder]
    pub fn alter_icon_name(&mut self, name: &str) -> &mut Self {
        self.classes.alter_value(
            ClassesOp::Replace(concat_string!("bi-", self.icon_name)),
            concat_string!("bi-", name),
        );
        self.icon_name = name.to_owned();
        self
    }

    // Icon GETTERS.

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }

    pub fn font_size(&self) -> &FontSize {
        &self.font_size
    }

    pub fn icon_name(&self) -> &str {
        self.icon_name.as_str()
    }
}
