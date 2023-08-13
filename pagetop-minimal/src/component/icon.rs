use pagetop::prelude::*;

new_handle!(COMPONENT_ICON);

#[rustfmt::skip]
#[derive(Default)]
pub struct Icon {
    weight    : Weight,
    renderable: Renderable,
    icon_name : String,
    classes   : Classes,
}

impl ComponentTrait for Icon {
    fn new() -> Self {
        Icon::default().with_classes(ClassesOp::SetDefault, "bi-question-circle-fill")
    }

    fn handle(&self) -> Handle {
        COMPONENT_ICON
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn before_prepare_component(&mut self, cx: &mut Context) {
        cx.alter(ContextOp::AddStyleSheet(
            StyleSheet::at("/minimal/icons/bootstrap-icons.css").with_version("1.8.2"),
        ));
    }

    fn prepare_component(&self, _: &mut Context) -> PrepareMarkup {
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
        self.icon_name = name.to_owned();
        self.alter_classes(
            ClassesOp::SetDefault,
            concat_string!("bi-", self.icon_name).as_str(),
        );
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    // Icon GETTERS.

    pub fn icon_name(&self) -> &str {
        self.icon_name.as_str()
    }

    pub fn classes(&self) -> &Classes {
        &self.classes
    }
}
