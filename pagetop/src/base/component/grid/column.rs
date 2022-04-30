use crate::prelude::*;

pub struct Column {
    renderable: fn() -> bool,
    weight    : i8,
    components: PageContainer,
    id        : OptIden,
    classes   : Classes,
    template  : String,
}

impl ComponentTrait for Column {
    fn new() -> Self {
        Column {
            renderable: render_always,
            weight    : 0,
            components: PageContainer::new(),
            id        : OptIden::new(),
            classes   : Classes::new_with_default("col"),
            template  : "default".to_owned(),
        }
    }

    fn name(&self) -> &'static str {
        "GridColumn"
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> i8 {
        self.weight
    }

    fn default_render(&self, assets: &mut PageAssets) -> Markup {
        html! {
            div id=[self.id()] class=[self.classes()] {
                (self.components().render(assets))
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

    // Column CONTAINER.

    pub fn add(mut self, component: impl ComponentTrait) -> Self {
        self.components.add(component);
        self
    }

    pub fn components(&self) -> &PageContainer {
        &self.components
    }

    // Column BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.alter_renderable(renderable);
        self
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
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

    pub fn using_template(mut self, template: &str) -> Self {
        self.alter_template(template);
        self
    }

    // Column ALTER.

    pub fn alter_renderable(&mut self, renderable: fn() -> bool) -> &mut Self {
        self.renderable = renderable;
        self
    }

    pub fn alter_weight(&mut self, weight: i8) -> &mut Self {
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

    pub fn alter_template(&mut self, template: &str) -> &mut Self {
        self.template = template.to_owned();
        self
    }

    // Column GETTERS.

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
