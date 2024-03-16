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
        flex::Container::new()
            .with_id("body__wrapper")
            .with_direction(flex::Direction::Column(BreakPoint::None))
            .with_items_align(flex::ItemAlign::Center)
            .add_item(flex::Item::full(Region::named("header")).with_id("header"))
            .add_item(flex::Item::full(Region::named("pagetop")).with_id("pagetop"))
            .add_item(flex::Item::full(
                flex::Container::new()
                    .with_id("content__wrapper")
                    .with_direction(flex::Direction::Row(BreakPoint::None))
                    .add_item(
                        flex::Item::with(Region::named("sidebar_left"))
                            .with_id("sidebar_left")
                            .with_grow(flex::ItemGrow::Is1),
                    )
                    .add_item(
                        flex::Item::with(Region::named("content"))
                            .with_id("content")
                            .with_grow(flex::ItemGrow::Is3),
                    )
                    .add_item(
                        flex::Item::with(Region::named("sidebar_right"))
                            .with_id("sidebar_right")
                            .with_grow(flex::ItemGrow::Is1),
                    ),
            ))
            .add_item(flex::Item::full(Region::named("footer")).with_id("footer"))
            .render(cx)
    }

    fn admin_layout(cx: &mut Context) -> Markup {
        Html::with(html! {
            ("admin")
        })
        .render(cx)
    }
}
