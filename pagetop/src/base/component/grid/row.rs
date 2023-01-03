use crate::prelude::*;

pub_handle!(COMPONENT_ROW);

hook_before_render_component!(HOOK_BEFORE_RENDER_ROW, Row);

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

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_renderable(mut self, check: IsRenderable) -> Self {
        self.alter_renderable(check);
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

    pub fn with_column(mut self, column: grid::Column) -> Self {
        self.alter_column(column);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Row ALTER.

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
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

    pub fn columns(&self) -> &ComponentsBundle {
        &self.columns
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
