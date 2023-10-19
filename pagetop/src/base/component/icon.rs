use crate::prelude::*;

new_handle!(COMPONENT_BASE_ICON);

#[rustfmt::skip]
#[derive(Default)]
pub struct Icon {
    weight    : Weight,
    renderable: Renderable,
    icon_name : String,
    classes   : OptionClasses,
}

impl ComponentTrait for Icon {
    fn new() -> Self {
        Icon::default().with_classes(ClassesOp::SetDefault, "bi-question-circle-fill")
    }

    fn handle(&self) -> Handle {
        COMPONENT_BASE_ICON
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        cx.set_param::<bool>(PARAM_BASE_INCLUDE_ICONS, true);

        PrepareMarkup::With(html! { i class=[self.classes().get()] {} })
    }
}

impl Icon {
    pub fn with(icon_name: &str) -> Self {
        Icon::new().with_icon_name(icon_name)
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
    pub fn alter_icon_name(&mut self, name: &str) -> &mut Self {
        self.alter_classes(ClassesOp::SetDefault, concat_string!("bi-", name));
        self.icon_name = name.to_owned();
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: impl Into<String>) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    // Icon GETTERS.

    pub fn icon_name(&self) -> &str {
        self.icon_name.as_str()
    }

    pub fn classes(&self) -> &OptionClasses {
        &self.classes
    }
}
