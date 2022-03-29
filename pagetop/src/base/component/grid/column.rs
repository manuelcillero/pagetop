use crate::prelude::*;

pub struct Column {
    renderable: fn() -> bool,
    weight    : i8,
    id        : OptIden,
    classes   : Classes,
    components: PageContainer,
    template  : String,
}

impl PageComponent for Column {

    fn new() -> Self {
        Column {
            renderable: always,
            weight    : 0,
            id        : OptIden::none(),
            classes   : Classes::some(vec!["col"]),
            components: PageContainer::new(),
            template  : "default".to_owned(),
        }
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
                (self.render_components(assets))
            }
        }
    }
}

impl Column {

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

    pub fn add_classes(mut self, classes: Vec<&str>) -> Self {
        self.classes.add_classes(classes);
        self
    }

    pub fn add(mut self, component: impl PageComponent) -> Self {
        self.components.add(component);
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

    pub fn classes(&self) -> &Option<String> {
        self.classes.option()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }

    // Column EXTRAS.

    pub fn render_components(&self, assets: &mut PageAssets) -> Markup {
        html! { (self.components.render(assets)) }
    }
}

fn always() -> bool {
    true
}
