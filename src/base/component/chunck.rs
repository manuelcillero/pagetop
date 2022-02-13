use crate::prelude::*;

pub struct Chunck {
    renderable: fn() -> bool,
    weight    : i8,
    markup    : Vec<Markup>,
    template  : String,
}

impl PageComponent for Chunck {

    fn prepare() -> Self {
        Chunck {
            renderable: always,
            weight    : 0,
            markup    : Vec::new(),
            template  : "default".to_string(),
        }
    }

    fn is_renderable(&self) -> bool {
        (self.renderable)()
    }

    fn weight(&self) -> i8 {
        self.weight
    }

    fn default_render(&self, _: &mut PageAssets) -> Markup {
        html! {
            @for markup in self.markup.iter() {
                (*markup)
            }
        }
    }
}

impl Chunck {

    // Chunck BUILDER.

    pub fn markup(markup: Markup) -> Self {
        let mut chunck = Chunck::prepare();
        chunck.markup.push(markup);
        chunck
    }

    pub fn with_renderable(mut self, renderable: fn() -> bool) -> Self {
        self.renderable = renderable;
        self
    }

    pub fn with_weight(mut self, weight: i8) -> Self {
        self.weight = weight;
        self
    }

    pub fn add_markup(mut self, markup: Markup) -> Self {
        self.markup.push(markup);
        self
    }

    pub fn using_template(mut self, template: &str) -> Self {
        self.template = template.to_string();
        self
    }

    // Chunck GETTERS.

    pub fn template(&self) -> &str {
        self.template.as_str()
    }
}

fn always() -> bool {
    true
}
