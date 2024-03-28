use crate::prelude::*;

#[derive(AutoDefault)]
pub enum ContainerType {
    #[default]
    Default,
    Body,
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
    weight        : Weight,
    renderable    : Renderable,
    classes       : OptionClasses,
    container_type: ContainerType,
    direction     : FlexDirection,
    flex_wrap     : FlexWrap,
    flex_justify  : FlexJustify,
    flex_align    : FlexAlign,
    flex_gap      : FlexGap,
    items         : MixedComponents,
}

impl ComponentTrait for Container {
    fn new() -> Self {
        Container::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn setup_before_prepare(&mut self, cx: &mut Context) {
        self.alter_classes(
            ClassesOp::Prepend,
            [
                String::from("flex__container"),
                self.direction().to_string(),
                self.wrap().to_string(),
                self.justify().to_string(),
                self.align().to_string(),
            ]
            .join(" "),
        );

        cx.set_param::<bool>(PARAM_BASE_INCLUDE_FLEX_ASSETS, true);
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        let output = self.items().render(cx);
        if output.is_empty() {
            return PrepareMarkup::None;
        }

        let gap = match self.gap() {
            FlexGap::Default => None,
            _ => Some(self.gap().to_string()),
        };
        match self.container_type() {
            ContainerType::Default => PrepareMarkup::With(html! {
                div id=[self.id()] class=[self.classes().get()] style=[gap] {
                    (output)
                }
            }),
            ContainerType::Body => PrepareMarkup::With(html! {
                body id=[self.id()] class=[self.classes().get()] style=[gap] {
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
    pub fn body() -> Self {
        Container {
            container_type: ContainerType::Body,
            ..Default::default()
        }
    }

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
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: FnIsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_direction(&mut self, direction: FlexDirection) -> &mut Self {
        self.direction = direction;
        self
    }

    #[fn_builder]
    pub fn alter_wrap(&mut self, wrap: FlexWrap) -> &mut Self {
        self.flex_wrap = wrap;
        self
    }

    #[fn_builder]
    pub fn alter_justify(&mut self, justify: FlexJustify) -> &mut Self {
        self.flex_justify = justify;
        self
    }

    #[fn_builder]
    pub fn alter_align(&mut self, align: FlexAlign) -> &mut Self {
        self.flex_align = align;
        self
    }

    #[fn_builder]
    pub fn alter_gap(&mut self, gap: FlexGap) -> &mut Self {
        self.flex_gap = gap;
        self
    }

    #[fn_builder]
    pub fn alter_items(&mut self, op: TypedOp<Flex>) -> &mut Self {
        self.items.alter_typed(op);
        self
    }

    #[rustfmt::skip]
    pub fn add_item(mut self, item: Flex) -> Self {
        self.items.alter_value(AnyOp::Add(AnyComponent::with(item)));
        self
    }

    // Container GETTERS.

    pub fn container_type(&self) -> &ContainerType {
        &self.container_type
    }

    pub fn direction(&self) -> &FlexDirection {
        &self.direction
    }

    pub fn wrap(&self) -> &FlexWrap {
        &self.flex_wrap
    }

    pub fn justify(&self) -> &FlexJustify {
        &self.flex_justify
    }

    pub fn align(&self) -> &FlexAlign {
        &self.flex_align
    }

    pub fn gap(&self) -> &FlexGap {
        &self.flex_gap
    }

    pub fn items(&self) -> &MixedComponents {
        &self.items
    }
}
