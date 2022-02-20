use crate::prelude::*;

enum ContainerType { Column, Row, Wrapper }

pub struct Container {
    renderable: fn() -> bool,
    weight    : i8,
    id        : String,
    container : ContainerType,
    components: PageContainer,
    template  : String,
}

impl PageComponent for Container {

    fn prepare() -> Self {
        Container {
            renderable: always,
            weight    : 0,
            id        : "".to_string(),
            container : ContainerType::Wrapper,
            components: PageContainer::new(),
            template  : "default".to_string(),
        }
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> i8 {
        self.weight
    }

    fn default_render(&self, assets: &mut PageAssets) -> Markup {
        let classes = match self.container {
            ContainerType::Wrapper => "wrapper",
            ContainerType::Row     => "row",
            ContainerType::Column  => "col",
        };
        html! {
            div class=(classes) {
                (self.components.render(assets))
            }
        }
    }
}

impl Container {

    pub fn row() -> Self {
        let mut grid = Container::prepare();
        grid.container = ContainerType::Row;
        grid
    }

    pub fn column() -> Self {
        let mut grid = Container::prepare();
        grid.container = ContainerType::Column;
        grid
    }

    // Container BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.renderable = renderable;
        self
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn add(mut self, component: impl PageComponent) -> Self {
        self.components.add(component);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_string();
        self
    }

    // Grid GETTERS.

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
