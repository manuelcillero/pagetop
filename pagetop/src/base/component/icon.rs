use crate::prelude::*;

pub const COMPONENT_ICON: &str = "pagetop::component::icon";

pub struct Icon {
    renderable: fn() -> bool,
    weight    : isize,
    icon_name : String,
    classes   : Classes,
}

impl ComponentTrait for Icon {
    fn new() -> Self {
        Icon {
            renderable: render_always,
            weight    : 0,
            icon_name : "question-circle-fill".to_owned(),
            classes   : Classes::new_with_default("bi-question-circle-fill"),
        }
    }

    fn handler(&self) -> &'static str {
        COMPONENT_ICON
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, _: &InContext) -> bool {
        (self.renderable)()
    }

    fn default_render(&self, context: &mut InContext) -> Markup {
        context
            .alter(InContextOp::StyleSheet(AssetsOp::Add(
                StyleSheet::located("/theme/icons/bootstrap-icons.css")
                    .with_version("1.8.2")
            )));

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

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_icon_name(mut self, name: &str) -> Self {
        self.alter_icon_name(name);
        self
    }

    pub fn with_classes(mut self, op: ClassesOp, classes: &str) -> Self {
        self.alter_classes(op, classes);
        self
    }

    // Icon ALTER.

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_icon_name(&mut self, name: &str) -> &mut Self {
        self.icon_name = name.to_owned();
        self.alter_classes(ClassesOp::SetDefault, concat_string!("bi-", self.icon_name).as_str());
        self
    }

    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter(op, classes);
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
