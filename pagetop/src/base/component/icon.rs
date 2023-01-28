use crate::prelude::*;

pub_handle!(COMPONENT_ICON);

#[rustfmt::skip]
#[derive(Default)]
pub struct Icon {
    weight    : isize,
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

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        (self.renderable.check)(rcx)
    }

    fn before_render(&mut self, rcx: &mut RenderContext) {
        rcx.alter(ContextOp::AddStyleSheet(
            StyleSheet::located("/theme/icons/bootstrap-icons.css").with_version("1.8.2"),
        ));
    }

    fn default_render(&self, _: &mut RenderContext) -> Markup {
        html! { i class=[self.classes().get()] {}; }
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl Icon {
    pub fn with(icon_name: &str) -> Self {
        Icon::new().with_icon_name(icon_name)
    }

    // Icon BUILDER.

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
