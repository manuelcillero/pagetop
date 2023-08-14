use pagetop::prelude::*;

new_handle!(COMPONENT_GRID_COLUMN);

actions_for_component!(Column);

const SIZE_DEFAULT: &str = "col";
const SIZE_1_OF_12: &str = "col-md-1";
const SIZE_2_OF_12: &str = "col-md-2";
const SIZE_3_OF_12: &str = "col-md-3";
const SIZE_4_OF_12: &str = "col-md-4";
const SIZE_5_OF_12: &str = "col-md-5";
const SIZE_6_OF_12: &str = "col-md-6";
const SIZE_7_OF_12: &str = "col-md-7";
const SIZE_8_OF_12: &str = "col-md-8";
const SIZE_9_OF_12: &str = "col-md-9";
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
    weight    : Weight,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    size      : ColumnSize,
    stuff     : PackComponents,
    template  : String,
}

impl ComponentTrait for Column {
    fn new() -> Self {
        Column::default().with_classes(ClassesOp::SetDefault, SIZE_DEFAULT)
    }

    fn handle(&self) -> Handle {
        COMPONENT_GRID_COLUMN
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn before_prepare_component(&mut self, cx: &mut Context) {
        run_actions_before_prepare_column(self, cx);
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div id=[self.id()] class=[self.classes().get()] {
                (self.components().prepare(cx))
            }
        })
    }

    fn after_prepare_component(&mut self, cx: &mut Context) {
        run_actions_after_prepare_column(self, cx);
    }
}

impl Column {
    // Column BUILDER.

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

    pub fn with_component(mut self, component: impl ComponentTrait) -> Self {
        self.stuff.alter(PackOp::Add(ComponentArc::with(component)));
        self
    }

    #[fn_builder]
    pub fn alter_components(&mut self, op: PackOp) -> &mut Self {
        self.stuff.alter(op);
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Column GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn size(&self) -> &ColumnSize {
        &self.size
    }

    pub fn components(&self) -> &PackComponents {
        &self.stuff
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
