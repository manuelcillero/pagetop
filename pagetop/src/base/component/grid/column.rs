use crate::prelude::*;

pub struct Column {
    renderable: fn() -> bool,
    weight    : i8,
    components: PageContainer,
    id        : OptIden,
    classes   : Classes,
    template  : String,
}

impl PageComponent for Column {
    fn new() -> Self {
        Column {
            renderable: always,
            weight    : 0,
            components: PageContainer::new(),
            id        : OptIden::none(),
            classes   : Classes::none(),
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
            div id=[self.id()] class=[self.classes("col")] {
                (self.components().render(assets))
            }
        }
    }
}

impl Column {

    // Column CONTAINER.

    pub fn add(mut self, component: impl PageComponent) -> Self {
        self.components.add(component);
        self
    }

    pub fn components(&self) -> &PageContainer {
        &self.components
    }

    // Column BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.renderable = renderable;
        self
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.id.with_value(id);
        self
    }

    pub fn set_classes(mut self, classes: &str) -> Self {
        self.classes.set_classes(classes);
        self
    }

    pub fn add_classes(mut self, classes: &str) -> Self {
        self.classes.add_classes(classes);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_owned();
        self
    }

    // Column GETTERS.

    pub fn id(&self) -> &Option<String> {
        self.id.option()
    }

    pub fn classes(&self, default: &str) -> Option<String> {
        self.classes.option(default)
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
