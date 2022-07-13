use crate::prelude::*;

pub const ICON_COMPONENT: &str = "pagetop::component::icon";

pub struct Icon {
    renderable: fn() -> bool,
    weight    : isize,
    icon_name : String,
    classes   : Classes,
    layout    : InlineLayout,
}

impl ComponentTrait for Icon {
    fn new() -> Self {
        Icon {
            renderable: render_always,
            weight    : 0,
            icon_name : "question-circle-fill".to_owned(),
            classes   : Classes::new(),
            layout    : InlineLayout::new(),
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

    fn before_render(&mut self, context: &mut InContext) {
        context
            .with_stylesheet(AssetsOp::Add(
                StyleSheet::located("/theme/icons/bootstrap-icons.css")
                    .with_version("1.8.2")
            ));

        self.alter_classes(concat_string!("bi-", self.icon_name()).as_str(), ClassesOp::SetDefault);
    }

    fn default_render(&self, _context: &mut InContext) -> Markup {
        html! { i class=[self.classes().get()] style=[self.layout().get()] {}; }
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

    pub fn with_layout(mut self, layout: &[LayoutSet]) -> Self {
        self.alter_layout(layout);
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
        self.icon_name = name.to_owned();
        self
    }

    pub fn alter_classes(&mut self, classes: &str, op: ClassesOp) -> &mut Self {
        self.classes.alter(classes, op);
        self
    }

    pub fn alter_layout(&mut self, layout: &[LayoutSet]) -> &mut Self {
        self.layout.set(layout);
        self
    }

    // Icon GETTERS.

    pub fn icon_name(&self) -> &str {
        self.icon_name.as_str()
    }

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn layout(&self) -> &InlineLayout {
        &self.layout
    }
}
