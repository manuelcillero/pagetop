use crate::prelude::*;

use_handle!(COMPONENT_BLOCK);

#[rustfmt::skip]
#[derive(Default)]
pub struct Block {
    weight    : isize,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    title     : AttributeValue,
    components: PackComponents,
    template  : String,
}

impl ComponentTrait for Block {
    fn new() -> Self {
        Block::default().with_classes(ClassesOp::SetDefault, "block")
    }

    fn handle(&self) -> Handle {
        COMPONENT_BLOCK
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn before_prepare_component(&mut self, cx: &mut Context) {
        actions::block::run_actions_before_prepare_component(self, cx);
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let id = cx.required_id::<Block>(self.id());
        PrepareMarkup::With(html! {
            div id=(id) class=[self.classes().get()] {
                @if let Some(title) = self.title().get() {
                    h2 class="block-title" { (title) }
                }
                div class="block-body" {
                    (self.components().prepare(cx))
                }
            }
        })
    }

    fn after_prepare_component(&mut self, cx: &mut Context) {
        actions::block::run_actions_after_prepare_component(self, cx);
    }

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl Block {
    // Block BUILDER.

    #[fn_builder]
    pub fn alter_id(&mut self, id: &str) -> &mut Self {
        self.id.alter_value(id);
        self
    }

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
    pub fn alter_classes(&mut self, op: ClassesOp, classes: &str) -> &mut Self {
        self.classes.alter_value(op, classes);
        self
    }

    #[fn_builder]
    pub fn alter_title(&mut self, title: &str) -> &mut Self {
        self.title.alter_value(title);
        self
    }

    pub fn with_component(mut self, component: impl ComponentTrait) -> Self {
        self.components.alter_pack(PackOp::Add, component);
        self
    }

    pub fn alter_components(&mut self, op: PackOp, component: impl ComponentTrait) -> &mut Self {
        self.components.alter_pack(op, component);
        self
    }

    #[fn_builder]
    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Block GETTERS.

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn title(&self) -> &AttributeValue {
        &self.title
    }

    pub fn components(&self) -> &PackComponents {
        &self.components
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
