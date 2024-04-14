use crate::prelude::*;

use super::Group;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Megamenu {
    id        : OptionId,
    renderable: Renderable,
    groups    : MixedComponents,
}

impl ComponentTrait for Megamenu {
    fn new() -> Self {
        Megamenu::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div id=[self.id()] class="menu__groups" {
                (self.groups().render(cx))
            }
        })
    }
}

impl Megamenu {
    // Megamenu BUILDER.

    #[fn_builder]
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: FnIsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_groups(&mut self, op: TypedOp<Group>) -> &mut Self {
        self.groups.alter_typed(op);
        self
    }

    #[rustfmt::skip]
    pub fn add_group(mut self, group: Group) -> Self {
        self.groups.alter_value(AnyOp::Add(AnyComponent::with(group)));
        self
    }

    // Megamenu GETTERS.

    pub fn groups(&self) -> &MixedComponents {
        &self.groups
    }
}
