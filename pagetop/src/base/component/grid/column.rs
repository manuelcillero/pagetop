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
    spaces    : Spaces,
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
            spaces    : Spaces::new(),
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
            ColumnSize::Default  => self.alter_classes("col-sm",    ClassesOp::SetDefault),
            ColumnSize::Is1of12  => self.alter_classes("col-sm-1",  ClassesOp::SetDefault),
            ColumnSize::Is2of12  => self.alter_classes("col-sm-2",  ClassesOp::SetDefault),
            ColumnSize::Is3of12  => self.alter_classes("col-sm-3",  ClassesOp::SetDefault),
            ColumnSize::Is4of12  => self.alter_classes("col-sm-4",  ClassesOp::SetDefault),
            ColumnSize::Is5of12  => self.alter_classes("col-sm-5",  ClassesOp::SetDefault),
            ColumnSize::Is6of12  => self.alter_classes("col-sm-6",  ClassesOp::SetDefault),
            ColumnSize::Is7of12  => self.alter_classes("col-sm-7",  ClassesOp::SetDefault),
            ColumnSize::Is8of12  => self.alter_classes("col-sm-8",  ClassesOp::SetDefault),
            ColumnSize::Is9of12  => self.alter_classes("col-sm-9",  ClassesOp::SetDefault),
            ColumnSize::Is10of12 => self.alter_classes("col-sm-10", ClassesOp::SetDefault),
            ColumnSize::Is11of12 => self.alter_classes("col-sm-11", ClassesOp::SetDefault),
            ColumnSize::IsFull   => self.alter_classes("col-sm-12", ClassesOp::SetDefault),
        };
    }

    fn default_render(&self, context: &mut InContext) -> Markup {
        html! {
            div id=[self.id().get()] class=[self.classes().get()] style=[self.spaces().get()] {
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

    pub fn with_classes(mut self, classes: &str, op: ClassesOp) -> Self {
        self.alter_classes(classes, op);
        self
    }

    pub fn with_spaces(mut self, spaces: &[SpaceSet]) -> Self {
        self.alter_spaces(spaces);
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

    pub fn alter_classes(&mut self, classes: &str, op: ClassesOp) -> &mut Self {
        self.classes.alter(classes, op);
        self
    }

    pub fn alter_spaces(&mut self, spaces: &[SpaceSet]) -> &mut Self {
        self.spaces.add(spaces);
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

    pub fn spaces(&self) -> &Spaces {
        &self.spaces
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
