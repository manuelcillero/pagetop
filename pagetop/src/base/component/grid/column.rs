use crate::prelude::*;

pub const COLUMN_COMPONENT: &str = "pagetop::component::grid::column";

pub enum ColumnSize {
    Default,
    Is1of12,
    Is2of12,
    Is3of12,
    Is4of12,
    Is5of12,
    Is6of12,
    Is7of12,
    Is8of12,
    Is9of12,
    Is10of12,
    Is11of12,
    IsFull,
}
pub struct Column {
    renderable: fn() -> bool,
    weight    : isize,
    id        : IdentifierValue,
    classes   : Classes,
    size      : ColumnSize,
    components: ComponentsBundle,
    template  : String,
}

impl ComponentTrait for Column {
    fn new() -> Self {
        Column {
            renderable: render_always,
            weight    : 0,
            id        : IdentifierValue::new(),
            classes   : Classes::new(),
            size      : ColumnSize::Default,
            components: ComponentsBundle::new(),
            template  : "default".to_owned(),
        }
    }

    fn handler(&self) -> &'static str {
        COLUMN_COMPONENT
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn before_render(&mut self, _context: &mut InContext) {
        match self.size() {
            ColumnSize::Default  => self.alter_classes(ClassesOp::SetDefault, "col-md"),
            ColumnSize::Is1of12  => self.alter_classes(ClassesOp::SetDefault, "col-md-1"),
            ColumnSize::Is2of12  => self.alter_classes(ClassesOp::SetDefault, "col-md-2"),
            ColumnSize::Is3of12  => self.alter_classes(ClassesOp::SetDefault, "col-md-3"),
            ColumnSize::Is4of12  => self.alter_classes(ClassesOp::SetDefault, "col-md-4"),
            ColumnSize::Is5of12  => self.alter_classes(ClassesOp::SetDefault, "col-md-5"),
            ColumnSize::Is6of12  => self.alter_classes(ClassesOp::SetDefault, "col-md-6"),
            ColumnSize::Is7of12  => self.alter_classes(ClassesOp::SetDefault, "col-md-7"),
            ColumnSize::Is8of12  => self.alter_classes(ClassesOp::SetDefault, "col-md-8"),
            ColumnSize::Is9of12  => self.alter_classes(ClassesOp::SetDefault, "col-md-9"),
            ColumnSize::Is10of12 => self.alter_classes(ClassesOp::SetDefault, "col-md-10"),
            ColumnSize::Is11of12 => self.alter_classes(ClassesOp::SetDefault, "col-md-11"),
            ColumnSize::IsFull   => self.alter_classes(ClassesOp::SetDefault, "col-md-12"),
        };
    }

    fn default_render(&self, context: &mut InContext) -> Markup {
        html! {
            div id=[self.id().get()] class=[self.classes().get()] {
                (self.components().render(context))
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

impl Column {

    // Column BUILDER.

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

    pub fn with_classes(mut self, op: ClassesOp, classes: &str) -> Self {
        self.alter_classes(op, classes);
        self
    }

    pub fn with_size(mut self, size: ColumnSize) -> Self {
        self.alter_size(size);
        self
    }

    pub fn with_component(mut self, component: impl ComponentTrait) -> Self {
        self.alter_component(component);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Column ALTER.

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

    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter(op, classes);
        self
    }

    pub fn alter_size(&mut self, size: ColumnSize) -> &mut Self {
        self.size = size;
        self
    }

    pub fn alter_component(&mut self, component: impl ComponentTrait) -> &mut Self {
        self.components.add(component);
        self
    }
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Column GETTERS.

    pub fn id(&self) -> &IdentifierValue {
        &self.id
    }

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn size(&self) -> &ColumnSize {
        &self.size
    }

    pub fn components(&self) -> &ComponentsBundle {
        &self.components
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
