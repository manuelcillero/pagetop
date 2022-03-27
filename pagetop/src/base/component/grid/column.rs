use crate::prelude::*;

pub struct Column {
    renderable: fn() -> bool,
    weight    : i8,
    id        : OptionId,
    components: PageContainer,
    template  : String,
}

impl PageComponent for Column {

    fn new() -> Self {
        Column {
            renderable: always,
            weight    : 0,
            id        : OptionId::none(),
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
            div id=[&self.id.option()] class="col" {
                (self.components.render(assets))
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

    pub fn add(mut self, component: impl PageComponent) -> Self {
        self.components.add(component);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_owned();
        self
    }

    // Column GETTERS.

    pub fn id(&self) -> &str {
        self.id.value()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
