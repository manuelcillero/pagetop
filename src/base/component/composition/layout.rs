use crate::prelude::*;

#[derive(AutoDefault)]
pub struct Layout;

impl ComponentTrait for Layout {
    fn new() -> Self {
        Layout
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(match cx.layout() {
            "default" => Self::default_layout(cx),
            "admin" => Self::admin_layout(cx),
            _ => Self::default_layout(cx),
        })
    }
}

impl Layout {
    fn default_layout(cx: &mut Context) -> Markup {
        Container::new()
            .with_id("body__wrapper")
            .with_direction(FlexDirection::Column(BreakPoint::None))
            .with_align(FlexAlign::Center)
            .add_item(Flex::with(Region::named("header")).with_id("header"))
            .add_item(Flex::with(Region::named("pagetop")).with_id("pagetop"))
            .add_item(
                Flex::with(
                    Container::new()
                        .with_direction(FlexDirection::Row(BreakPoint::None))
                        .add_item(
                            Flex::with(Region::named("sidebar_left"))
                                .with_id("sidebar_left")
                                .with_grow(FlexGrow::Is1),
                        )
                        .add_item(
                            Flex::with(Region::named("content"))
                                .with_id("content")
                                .with_grow(FlexGrow::Is3),
                        )
                        .add_item(
                            Flex::with(Region::named("sidebar_right"))
                                .with_id("sidebar_right")
                                .with_grow(FlexGrow::Is1),
                        ),
                )
                .with_id("flex__wrapper"),
            )
            .add_item(Flex::with(Region::named("footer")).with_id("footer"))
            .render(cx)
    }

    fn admin_layout(cx: &mut Context) -> Markup {
        Html::with(html! {
            ("admin")
        })
        .render(cx)
    }
}
