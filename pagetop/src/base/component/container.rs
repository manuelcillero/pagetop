use crate::prelude::*;

pub enum ContainerType { Header, Footer, Main, Section, Wrapper }

pub struct Container {
    renderable: fn() -> bool,
    weight    : i8,
    container : ContainerType,
    components: PageContainer,
    id        : OptIden,
    classes   : Classes,
    template  : String,
}

impl PageComponent for Container {
    fn new() -> Self {
        Container {
            renderable: always,
            weight    : 0,
            container : ContainerType::Wrapper,
            components: PageContainer::new(),
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
        match self.container_type() {
            ContainerType::Header => html! {
                header id=[self.id()] class=[self.classes("header")] {
                    div class="container" {
                        (self.components().render(assets))
                    }
                }
            },
            ContainerType::Footer => html! {
                footer id=[self.id()] class=[self.classes("footer")] {
                    div class="container" {
                        (self.components().render(assets))
                    }
                }
            },
            ContainerType::Main => html! {
                main id=[self.id()] class=[self.classes("main")] {
                    div class="container" {
                        (self.components().render(assets))
                    }
                }
            },
            ContainerType::Section => html! {
                section id=[self.id()] class=[self.classes("section")] {
                    div class="container" {
                        (self.components().render(assets))
                    }
                }
            },
            _ => html! {
                div id=[self.id()] class=[self.classes("container")] {
                    (self.components().render(assets))
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

    pub fn add(mut self, component: ArcComponent) -> Self {
        self.components.add(component);
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

    // Container GETTERS.

    pub fn container_type(&self) -> &ContainerType {
        &self.container
    }

    pub fn components(&self) -> &PageContainer {
        &self.components
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
