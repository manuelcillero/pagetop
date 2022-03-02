use crate::prelude::*;

pub struct Block {
    renderable: fn() -> bool,
    weight    : i8,
    id        : Option<String>,
    title     : Option<String>,
    markup    : Vec<Markup>,
    template  : String,
}

impl PageComponent for Block {

    fn prepare() -> Self {
        Block {
            renderable: always,
            weight    : 0,
            id        : None,
            title     : None,
            markup    : Vec::new(),
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
            div id=(id) class="block" {
                @if self.title != None {
                    h2 class="block-title" { (self.title()) }
                }
                div class="block-body" {
                    @for markup in self.markup.iter() {
                        (*markup)
                    }
                }
            }
        }
    }
}

impl Block {

    pub fn markup(markup: Markup) -> Self {
        Block::prepare().add_markup(markup)
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

    pub fn with_id(mut self, id: &str) -> Self {
        self.id = util::valid_id(id);
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = util::optional_str(title);
        self
    }

    pub fn add_markup(mut self, markup: Markup) -> Self {
        self.markup.push(markup);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_owned();
        self
    }

    // Block GETTERS.

    pub fn id(&self) -> &str {
        util::assigned_str(&self.id)
    }

    pub fn title(&self) -> &str {
        util::assigned_str(&self.title)
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
