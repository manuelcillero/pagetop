use crate::prelude::*;

#[derive(AutoDefault)]
pub enum ContainerType {
    #[default]
    Default,
    Header,
    Main,
    Section,
    Article,
    Footer,
}

#[rustfmt::skip]
#[derive(AutoDefault, ComponentClasses)]
pub struct Container {
    id            : OptionId,
    classes       : OptionClasses,
    container_type: ContainerType,
    direction     : flex::Direction,
    flex_wrap     : flex::Wrap,
    flex_justify  : flex::Justify,
    flex_align    : flex::Align,
    flex_gap      : flex::Gap,
    items         : MixedComponents,
}

impl ComponentTrait for Container {
    fn new() -> Self {
        Container::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn setup_before_prepare(&mut self, cx: &mut Context) {
        self.set_classes(
            ClassesOp::Prepend,
            [
                "flex__container".to_string(),
                self.direction().to_string(),
                self.wrap().to_string(),
                self.justify().to_string(),
                self.align().to_string(),
            ]
            .join(" "),
        );

        cx.set_param::<bool>(PARAM_BASE_INCLUDE_FLEX_ASSETS, &true);
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let output = self.items().render(cx);
        if output.is_empty() {
            return PrepareMarkup::None;
        }

        let gap = match self.gap() {
            flex::Gap::Default => None,
            _ => Some(self.gap().to_string()),
        };
        match self.container_type() {
            ContainerType::Default => PrepareMarkup::With(html! {
                div id=[self.id()] class=[self.classes().get()] style=[gap] {
                    (output)
                }
            }),
            ContainerType::Header => PrepareMarkup::With(html! {
                header id=[self.id()] class=[self.classes().get()] style=[gap] {
                    (output)
                }
            }),
            ContainerType::Main => PrepareMarkup::With(html! {
                main id=[self.id()] class=[self.classes().get()] style=[gap] {
                    (output)
                }
            }),
            ContainerType::Section => PrepareMarkup::With(html! {
                section id=[self.id()] class=[self.classes().get()] style=[gap] {
                    (output)
                }
            }),
            ContainerType::Article => PrepareMarkup::With(html! {
                article id=[self.id()] class=[self.classes().get()] style=[gap] {
                    (output)
                }
            }),
            ContainerType::Footer => PrepareMarkup::With(html! {
                footer id=[self.id()] class=[self.classes().get()] style=[gap] {
                    (output)
                }
            }),
        }
    }
}

impl Container {
    pub fn header() -> Self {
        Container {
            container_type: ContainerType::Header,
            ..Default::default()
        }
    }

    pub fn main() -> Self {
        Container {
            container_type: ContainerType::Main,
            ..Default::default()
        }
    }

    pub fn section() -> Self {
        Container {
            container_type: ContainerType::Section,
            ..Default::default()
        }
    }

    pub fn article() -> Self {
        Container {
            container_type: ContainerType::Article,
            ..Default::default()
        }
    }

    pub fn footer() -> Self {
        Container {
            container_type: ContainerType::Footer,
            ..Default::default()
        }
    }

    // Container BUILDER.

    #[fn_builder]
    pub fn set_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.set_value(id);
        self
    }

    #[fn_builder]
    pub fn set_direction(&mut self, direction: flex::Direction) -> &mut Self {
        self.direction = direction;
        self
    }

    #[fn_builder]
    pub fn set_wrap(&mut self, wrap: flex::Wrap) -> &mut Self {
        self.flex_wrap = wrap;
        self
    }

    #[fn_builder]
    pub fn set_justify(&mut self, justify: flex::Justify) -> &mut Self {
        self.flex_justify = justify;
        self
    }

    #[fn_builder]
    pub fn set_align(&mut self, align: flex::Align) -> &mut Self {
        self.flex_align = align;
        self
    }

    #[fn_builder]
    pub fn set_gap(&mut self, gap: flex::Gap) -> &mut Self {
        self.flex_gap = gap;
        self
    }

    #[fn_builder]
    pub fn set_items(&mut self, op: TypedOp<flex::Item>) -> &mut Self {
        self.items.set_typed(op);
        self
    }

    pub fn add_item(mut self, item: flex::Item) -> Self {
        self.items.set_value(AnyOp::Add(AnyComponent::with(item)));
        self
    }

    // Container GETTERS.

    pub fn container_type(&self) -> &ContainerType {
        &self.container_type
    }

    pub fn direction(&self) -> &flex::Direction {
        &self.direction
    }

    pub fn wrap(&self) -> &flex::Wrap {
        &self.flex_wrap
    }

    pub fn justify(&self) -> &flex::Justify {
        &self.flex_justify
    }

    pub fn align(&self) -> &flex::Align {
        &self.flex_align
    }

    pub fn gap(&self) -> &flex::Gap {
        &self.flex_gap
    }

    pub fn items(&self) -> &MixedComponents {
        &self.items
    }
}
