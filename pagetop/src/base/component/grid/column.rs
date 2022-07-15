use crate::prelude::*;

pub const COMPONENT_COLUMN: &str = "pagetop::component::grid::column";

const SIZE_DEFAULT:  &str = "col-md";
const SIZE_1_OF_12:  &str = "col-md-1";
const SIZE_2_OF_12:  &str = "col-md-2";
const SIZE_3_OF_12:  &str = "col-md-3";
const SIZE_4_OF_12:  &str = "col-md-4";
const SIZE_5_OF_12:  &str = "col-md-5";
const SIZE_6_OF_12:  &str = "col-md-6";
const SIZE_7_OF_12:  &str = "col-md-7";
const SIZE_8_OF_12:  &str = "col-md-8";
const SIZE_9_OF_12:  &str = "col-md-9";
const SIZE_10_OF_12: &str = "col-md-10";
const SIZE_11_OF_12: &str = "col-md-11";
const SIZE_12_OF_12: &str = "col-md-12";

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
            classes   : Classes::new_with_default(SIZE_DEFAULT),
            size      : ColumnSize::Default,
            components: ComponentsBundle::new(),
            template  : "default".to_owned(),
        }
    }

    fn handler(&self) -> &'static str {
        COMPONENT_COLUMN
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, _: &InContext) -> bool {
        (self.renderable)()
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

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
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

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
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
        match size {
            ColumnSize::Default  => self.alter_classes(ClassesOp::SetDefault, SIZE_DEFAULT),
            ColumnSize::Is1of12  => self.alter_classes(ClassesOp::SetDefault, SIZE_1_OF_12),
            ColumnSize::Is2of12  => self.alter_classes(ClassesOp::SetDefault, SIZE_2_OF_12),
            ColumnSize::Is3of12  => self.alter_classes(ClassesOp::SetDefault, SIZE_3_OF_12),
            ColumnSize::Is4of12  => self.alter_classes(ClassesOp::SetDefault, SIZE_4_OF_12),
            ColumnSize::Is5of12  => self.alter_classes(ClassesOp::SetDefault, SIZE_5_OF_12),
            ColumnSize::Is6of12  => self.alter_classes(ClassesOp::SetDefault, SIZE_6_OF_12),
            ColumnSize::Is7of12  => self.alter_classes(ClassesOp::SetDefault, SIZE_7_OF_12),
            ColumnSize::Is8of12  => self.alter_classes(ClassesOp::SetDefault, SIZE_8_OF_12),
            ColumnSize::Is9of12  => self.alter_classes(ClassesOp::SetDefault, SIZE_9_OF_12),
            ColumnSize::Is10of12 => self.alter_classes(ClassesOp::SetDefault, SIZE_10_OF_12),
            ColumnSize::Is11of12 => self.alter_classes(ClassesOp::SetDefault, SIZE_11_OF_12),
            ColumnSize::IsFull   => self.alter_classes(ClassesOp::SetDefault, SIZE_12_OF_12),
        };
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
