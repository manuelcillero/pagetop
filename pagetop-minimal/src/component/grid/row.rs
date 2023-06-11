use pagetop::prelude::*;

use crate::component::grid;

define_handle!(COMPONENT_ROW);

action_before_render_component!(ACTION_BEFORE_RENDER_ROW for Row);

#[rustfmt::skip]
#[derive(Default)]
pub struct Row {
    weight    : isize,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    columns   : ComponentsBundle,
    template  : String,
}

impl ComponentTrait for Row {
    fn new() -> Self {
        Row::default().with_classes(ClassesOp::SetDefault, "row")
    }

    fn handle(&self) -> Handle {
        COMPONENT_ROW
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rcx: &RenderContext) -> bool {
        (self.renderable.check)(rcx)
    }

    fn before_render(&mut self, rcx: &mut RenderContext) {
        run_actions_before_render_component(self, rcx);
    }

    fn default_render(&self, rcx: &mut RenderContext) -> Markup {
        html! {
            div id=[self.id()] class=[self.classes().get()] {
                (self.columns().render(rcx))
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

    pub fn with_column(mut self, column: grid::Column) -> Self {
        self.columns.alter_bundle(BundleOp::Add, column);
        self
    }

    pub fn alter_columns(&mut self, op: BundleOp, column: grid::Column) -> &mut Self {
        self.columns.alter_bundle(op, column);
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Row GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn columns(&self) -> &ComponentsBundle {
        &self.columns
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
