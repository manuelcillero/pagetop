use crate::prelude::*;

enum ContainerType { Header, Footer, Main, Section, Wrapper }

pub struct Container {
    renderable: fn() -> bool,
    weight    : i8,
    id        : OptionId,
    container : ContainerType,
    components: PageContainer,
    template  : String,
}

impl PageComponent for Container {

    fn new() -> Self {
        Container {
            renderable: always,
            weight    : 0,
            id        : OptionId::none(),
            container : ContainerType::Wrapper,
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
        match self.container {
            ContainerType::Header => html! {
                header id=[&self.id.option()] class="header" {
                    div class="container" {
                        (self.components.render(assets))
                    }
                }
            },
            ContainerType::Footer => html! {
                footer id=[&self.id.option()] class="footer" {
                    div class="container" {
                        (self.components.render(assets))
                    }
                }
            },
            ContainerType::Main => html! {
                main id=[&self.id.option()] class="main" {
                    div class="container" {
                        (self.components.render(assets))
                    }
                }
            },
            ContainerType::Section => html! {
                section id=[&self.id.option()] class="section" {
                    div class="container" {
                        (self.components.render(assets))
                    }
                }
            },
            _ => html! {
                div id=[&self.id.option()] class="container" {
                    (self.components.render(assets))
                }
            }
        }
    }
}

impl Container {

    pub fn header() -> Self {
        let mut c = Container::new();
        c.container = ContainerType::Header;
        c
    }

    pub fn footer() -> Self {
        let mut c = Container::new();
        c.container = ContainerType::Footer;
        c
    }

    pub fn main() -> Self {
        let mut c = Container::new();
        c.container = ContainerType::Main;
        c
    }

    pub fn section() -> Self {
        let mut c = Container::new();
        c.container = ContainerType::Section;
        c
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

    // Container GETTERS.

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
