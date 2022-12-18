use crate::prelude::*;

pub_handle!(COMPONENT_BLOCK);

hook_before_render_component!(HOOK_BEFORE_RENDER_BLOCK, Block);

#[rustfmt::skip]
#[derive(Default)]
pub struct Block {
    weight    : isize,
    renderable: Renderable,
    id        : IdentifierValue,
    classes   : Classes,
    title     : AttributeValue,
    components: ComponentsBundle,
    template  : String,
}

impl ComponentTrait for Block {
    fn new() -> Self {
        Block::default().with_classes(ClassesOp::SetDefault, "block")
    }

    fn handle(&self) -> Handle {
        COMPONENT_BLOCK
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, rsx: &RenderResources) -> bool {
        (self.renderable.check)(rsx)
    }

    fn before_render(&mut self, rsx: &mut RenderResources) {
        before_render_inline(self, rsx);
    }

    fn default_render(&self, rsx: &mut RenderResources) -> Markup {
        let id = rsx.required_id::<Block>(self.id());
        html! {
            div id=(id) class=[self.classes().get()] {
                @if let Some(title) = self.title().get() {
                    h2 class="block-title" { (title) }
                }
                div class="block-body" {
                    (self.components().render(rsx))
                }
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

impl Block {
    // Block BUILDER.

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

    pub fn with_title(mut self, title: &str) -> Self {
        self.alter_title(title);
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

    // Block ALTER.

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

    pub fn alter_title(&mut self, title: &str) -> &mut Self {
        self.title.alter_value(title);
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

    // Block GETTERS.

    pub fn id(&self) -> &IdentifierValue {
        &self.id
    }

    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn title(&self) -> &AttributeValue {
        &self.title
    }

    pub fn components(&self) -> &ComponentsBundle {
        &self.components
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
