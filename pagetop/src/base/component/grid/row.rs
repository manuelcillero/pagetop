use crate::prelude::*;

pub struct Row {
    renderable: fn() -> bool,
    weight    : i8,
    id        : OptIden,
    classes   : Classes,
    columns   : PageContainer,
    template  : String,
}

impl PageComponent for Row {

    fn new() -> Self {
        Row {
            renderable: always,
            weight    : 0,
            id        : OptIden::none(),
            classes   : Classes::some(vec!["row"]),
            columns   : PageContainer::new(),
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
                (self.render_columns(assets))
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

    pub fn with_id(mut self, id: &str) -> Self {
        self.id.with_value(id);
        self
    }

    pub fn add_classes(mut self, classes: Vec<&str>) -> Self {
        self.classes.add_classes(classes);
        self
    }

    pub fn add_column(mut self, column: grid::Column) -> Self {
        self.columns.add(column);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_owned();
        self
    }

    // Row GETTERS.

    pub fn id(&self) -> &Option<String> {
        self.id.option()
    }

    pub fn classes(&self) -> &Option<String> {
        self.classes.option()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }

    // Row EXTRAS.

    pub fn render_columns(&self, assets: &mut PageAssets) -> Markup {
        html! { (self.columns.render(assets)) }
    }
}

fn always() -> bool {
    true
}
