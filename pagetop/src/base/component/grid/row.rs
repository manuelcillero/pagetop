use crate::prelude::*;

pub struct Row {
    renderable: fn() -> bool,
    weight    : i8,
    id        : OptIden,
    columns   : PageContainer,
    template  : String,
}

impl PageComponent for Row {

    fn new() -> Self {
        Row {
            renderable: always,
            weight    : 0,
            id        : OptIden::none(),
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
            div id=[&self.id.option()] class="row" {
                (self.columns.render(assets))
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

    pub fn add_column(mut self, column: grid::Column) -> Self {
        self.columns.add(column);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_owned();
        self
    }

    // Row GETTERS.

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
