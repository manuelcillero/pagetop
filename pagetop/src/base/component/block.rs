use crate::prelude::*;

pub struct Block {
    renderable: fn() -> bool,
    weight    : i8,
    title     : OptAttr,
    html      : Vec<Markup>,
    id        : OptIden,
    classes   : Classes,
    template  : String,
}

impl PageComponent for Block {
    fn new() -> Self {
        Block {
            renderable: always,
            weight    : 0,
            title     : OptAttr::none(),
            html      : Vec::new(),
            id        : OptIden::none(),
            classes   : Classes::none(),
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
        let id = assets.serial_id(self.name(), self.id());
        html! {
            div id=(id) class=[self.classes("block")] {
                @match self.title() {
                    Some(title) => h2 class="block-title" { (title) },
                    None => {}
                }
                div class="block-body" {
                    @for html in self.html().iter() {
                        (*html)
                    }
                }
            }
        }
    }
}

impl Block {
    pub fn with(html: Markup) -> Self {
        Block::new().add(html)
    }

    // Block BUILDER.

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.renderable = renderable;
        self
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title.with_value(title);
        self
    }

    pub fn add(mut self, html: Markup) -> Self {
        self.html.push(html);
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

    // Block GETTERS.

    pub fn title(&self) -> &Option<String> {
        self.title.option()
    }

    pub fn html(&self) -> &Vec<Markup> {
        &self.html
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
