use pagetop::prelude::*;

define_handle!(COMPONENT_COLUMN);

action_before_render_component!(ACTION_BEFORE_RENDER_COLUMN, Column);

const SIZE__DEFAULT: &str = "col-md";
const SIZE__1_OF_12: &str = "col-md-1";
const SIZE__2_OF_12: &str = "col-md-2";
const SIZE__3_OF_12: &str = "col-md-3";
const SIZE__4_OF_12: &str = "col-md-4";
const SIZE__5_OF_12: &str = "col-md-5";
const SIZE__6_OF_12: &str = "col-md-6";
const SIZE__7_OF_12: &str = "col-md-7";
const SIZE__8_OF_12: &str = "col-md-8";
const SIZE__9_OF_12: &str = "col-md-9";
const SIZE_10_OF_12: &str = "col-md-10";
const SIZE_11_OF_12: &str = "col-md-11";
const SIZE_12_OF_12: &str = "col-md-12";

#[derive(Default)]
pub enum ColumnSize {
    #[default]
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

#[rustfmt::skip]
#[derive(Default)]
pub struct Column {
    weight    : isize,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    size      : ColumnSize,
    components: ComponentsBundle,
    template  : String,
}

impl ComponentTrait for Column {
    fn new() -> Self {
        Column::default().with_classes(ClassesOp::SetDefault, SIZE__DEFAULT)
    }

    fn handle(&self) -> Handle {
        COMPONENT_COLUMN
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        (self.renderable.check)(rcx)
    }

    fn before_render(&mut self, rcx: &mut RenderContext) {
        before_render_inline(self, rcx);
    }

    fn default_render(&self, rcx: &mut RenderContext) -> Markup {
        html! {
            div id=[self.id().get()] class=[self.classes().get()] {
                (self.components().render(rcx))
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
    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[rustfmt::skip]
    #[fn_builder]
    pub fn alter_size(&mut self, size: ColumnSize) -> &mut Self {
        match size {
            ColumnSize::Default  => self.alter_classes(ClassesOp::SetDefault, SIZE__DEFAULT),
            ColumnSize::Is1of12  => self.alter_classes(ClassesOp::SetDefault, SIZE__1_OF_12),
            ColumnSize::Is2of12  => self.alter_classes(ClassesOp::SetDefault, SIZE__2_OF_12),
            ColumnSize::Is3of12  => self.alter_classes(ClassesOp::SetDefault, SIZE__3_OF_12),
            ColumnSize::Is4of12  => self.alter_classes(ClassesOp::SetDefault, SIZE__4_OF_12),
            ColumnSize::Is5of12  => self.alter_classes(ClassesOp::SetDefault, SIZE__5_OF_12),
            ColumnSize::Is6of12  => self.alter_classes(ClassesOp::SetDefault, SIZE__6_OF_12),
            ColumnSize::Is7of12  => self.alter_classes(ClassesOp::SetDefault, SIZE__7_OF_12),
            ColumnSize::Is8of12  => self.alter_classes(ClassesOp::SetDefault, SIZE__8_OF_12),
            ColumnSize::Is9of12  => self.alter_classes(ClassesOp::SetDefault, SIZE__9_OF_12),
            ColumnSize::Is10of12 => self.alter_classes(ClassesOp::SetDefault, SIZE_10_OF_12),
            ColumnSize::Is11of12 => self.alter_classes(ClassesOp::SetDefault, SIZE_11_OF_12),
            ColumnSize::IsFull   => self.alter_classes(ClassesOp::SetDefault, SIZE_12_OF_12),
        };
        self.size = size;
        self
    }

    #[fn_builder]
    pub fn alter_component(&mut self, component: impl ComponentTrait) -> &mut Self {
        self.components.add(component);
        self
    }

    #[fn_builder]
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
