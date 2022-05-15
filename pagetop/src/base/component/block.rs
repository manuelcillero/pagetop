use crate::prelude::*;

pub const BLOCK_COMPONENT: &str = "pagetop::component::block";

pub struct Block {
    renderable: fn() -> bool,
    weight    : isize,
    components: ComponentsBundle,
    title     : OptAttr,
    id        : OptIden,
    classes   : Classes,
    template  : String,
}

impl ComponentTrait for Block {
    fn new() -> Self {
        Block {
            renderable: render_always,
            weight    : 0,
            components: ComponentsBundle::new(),
            title     : OptAttr::new(),
            id        : OptIden::new(),
            classes   : Classes::new_with_default("block"),
            template  : "default".to_owned(),
        }
    }

    fn handler(&self) -> &'static str {
        BLOCK_COMPONENT
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn default_render(&self, context: &mut InContext) -> Markup {
        let id = context.required_id::<Block>(self.id());
        html! {
            div id=(id) class=[self.classes()] {
                @match self.title() {
                    Some(title) => h2 class="block-title" { (title) },
                    None => {}
                }
                div class="block-body" {
                    (self.components().render(context))
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

    // Block CONTAINER.

    pub fn add(mut self, component: impl ComponentTrait) -> Self {
        self.components.add(component);
        self
    }

    pub fn components(&self) -> &ComponentsBundle {
        &self.components
    }

    // Block BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.alter_weight(weight);
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.alter_title(title);
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

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Block ALTER.

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
        self
    }

    pub fn alter_title(&mut self, title: &str) -> &mut Self {
        self.title.with_value(title);
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

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Block GETTERS.

    pub fn title(&self) -> &Option<String> {
        self.title.option()
    }

    pub fn id(&self) -> &Option<String> {
        self.id.option()
    }

    pub fn classes(&self) -> &Option<String> {
        self.classes.option()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}
