use crate::prelude::*;

pub struct Row {
    renderable: fn() -> bool,
    weight    : i8,
    columns   : PageContainer,
    id        : OptIden,
    classes   : Classes,
    template  : String,
}

impl PageComponent for Row {
    fn new() -> Self {
        Row {
            renderable: always,
            weight    : 0,
            columns   : PageContainer::new(),
            id        : OptIden::none(),
            classes   : Classes::none(),
            template  : "default".to_owned(),
        }
    }

    fn name(&self) -> &'static str {
        "GridRow"
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> i8 {
        self.weight
    }

    fn default_render(&self, assets: &mut PageAssets) -> Markup {
        html! {
            div id=[self.id()] class=[self.classes("row")] {
                (self.columns().render(assets))
            }
        }
    }
}

impl Row {

    // Row BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.renderable = renderable;
        self
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.weight = weight;
        self
    }

    pub fn add_column(mut self, column: grid::Column) -> Self {
        self.columns.add(column);
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

    // Row GETTERS.

    pub fn columns(&self) -> &PageContainer {
        &self.columns
    }

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
