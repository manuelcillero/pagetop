use crate::prelude::*;

pub const ICON_COMPONENT: &str = "pagetop::component::icon";

pub struct Icon {
    renderable: fn() -> bool,
    weight    : isize,
    classes   : Classes,
    spaces   : Spaces,
}

impl ComponentTrait for Icon {
    fn new() -> Self {
        Icon {
            renderable: render_always,
            weight    : 0,
            classes   : Classes::new_with_default("bi-question-circle-fill"),
            spaces   : Spaces::new(),
        }
    }

    fn handler(&self) -> &'static str {
        ICON_COMPONENT
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn default_render(&self, context: &mut InContext) -> Markup {
        context
            .add_stylesheet(StyleSheet::with_source(
                "/theme/icons/bootstrap-icons.css?ver=1.8.2"
            ));

        html! { i class=[self.classes().get()] style=[self.spaces().get()] {}; }
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

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_icon_name(mut self, name: &str) -> Self {
        self.alter_icon_name(name);
        self
    }

    pub fn with_classes(mut self, classes: &str, op: ClassesOp) -> Self {
        self.alter_classes(classes, op);
        self
    }

    pub fn with_spaces(mut self, spaces: &[SpaceSet]) -> Self {
        self.alter_spaces(spaces);
        self
    }

    // Icon ALTER.

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_icon_name(&mut self, name: &str) -> &mut Self {
        self.classes.alter(concat_string!("bi-", name).as_str(), ClassesOp::SetDefault);
        self
    }

    pub fn alter_classes(&mut self, classes: &str, op: ClassesOp) -> &mut Self {
        self.classes.alter(classes, op);
        self
    }

    pub fn alter_spaces(&mut self, spaces: &[SpaceSet]) -> &mut Self {
        self.spaces.add(spaces);
        self
    }

    // Icon GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn spaces(&self) -> &Spaces {
        &self.spaces
    }
}
