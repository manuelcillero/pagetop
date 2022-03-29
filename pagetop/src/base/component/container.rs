use crate::prelude::*;

pub enum ContainerType { Header, Footer, Main, Section, Wrapper }

pub struct Container {
    renderable: fn() -> bool,
    weight    : i8,
    container : ContainerType,
    id        : OptIden,
    components: PageContainer,
    template  : String,
}

impl PageComponent for Container {

    fn new() -> Self {
        Container {
            renderable: always,
            weight    : 0,
            container : ContainerType::Wrapper,
            id        : OptIden::none(),
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
        match self.container_type() {
            ContainerType::Header => html! {
                header id=[self.id()] class="header" {
                    div class="container" {
                        (self.render_components(assets))
                    }
                }
            },
            ContainerType::Footer => html! {
                footer id=[self.id()] class="footer" {
                    div class="container" {
                        (self.render_components(assets))
                    }
                }
            },
            ContainerType::Main => html! {
                main id=[self.id()] class="main" {
                    div class="container" {
                        (self.render_components(assets))
                    }
                }
            },
            ContainerType::Section => html! {
                section id=[self.id()] class="section" {
                    div class="container" {
                        (self.render_components(assets))
                    }
                }
            },
            _ => html! {
                div id=[self.id()] class="container" {
                    (self.render_components(assets))
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

    pub fn container_type(&self) -> &ContainerType {
        &self.container
    }

    pub fn id(&self) -> &Option<String> {
        self.id.option()
    }

    pub fn template(&self) -> &str {
        self.template.as_str()
    }

    // Container EXTRAS.

    pub fn render_components(&self, assets: &mut PageAssets) -> Markup {
        html! { (self.components.render(assets)) }
    }
}

fn always() -> bool {
    true
}
