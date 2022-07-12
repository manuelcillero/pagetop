use crate::prelude::*;

pub const ROW_COMPONENT: &str = "pagetop::component::grid::row";

pub struct Row {
    renderable: fn() -> bool,
    weight    : isize,
    id        : IdentifierValue,
    classes   : Classes,
    layout    : InlineLayout,
    columns   : ComponentsBundle,
    template  : String,
}

impl ComponentTrait for Row {
    fn new() -> Self {
        Row {
            renderable: render_always,
            weight    : 0,
            id        : IdentifierValue::new(),
            classes   : Classes::new_with_default("row"),
            layout    : InlineLayout::new(),
            columns   : ComponentsBundle::new(),
            template  : "default".to_owned(),
        }
    }

    fn handler(&self) -> &'static str {
        ROW_COMPONENT
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn default_render(&self, context: &mut InContext) -> Markup {
        html! {
            div id=[self.id().get()] class=[self.classes().get()] style=[self.layout().get()] {
                (self.columns().render(context))
            }
        }
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl Row {

    // Row BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.alter_id(id);
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

    pub fn with_column(mut self, column: grid::Column) -> Self {
        self.alter_column(column);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Row ALTER.

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.with_value(id);
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

    pub fn alter_column(&mut self, column: grid::Column) -> &mut Self {
        self.columns.add(column);
        self
    }

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Row GETTERS.

    pub fn id(&self) -> &IdentifierValue {
        &self.id
    }

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn layout(&self) -> &InlineLayout {
        &self.layout
    }

    pub fn columns(&self) -> &ComponentsBundle {
        &self.columns
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
