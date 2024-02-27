use crate::prelude::*;

use super::Group;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Megamenu {
    id        : OptionId,
    weight    : Weight,
    renderable: Renderable,
    groups    : TypedComponents<Group>,
}

impl ComponentTrait for Megamenu {
    fn new() -> Self {
        Megamenu::default()
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

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            div id=[self.id()] class="pt-menu__groups" {
                (self.groups().render(cx))
            }
        })
    }
}

impl Megamenu {
    // Megamenu BUILDER.

    #[fn_with]
    pub fn alter_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.alter_value(id);
        self
    }

    #[fn_with]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_with]
    pub fn alter_renderable(&mut self, check: FnIsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[rustfmt::skip]
    pub fn add_group(mut self, group: Group) -> Self {
        self.groups.alter_value(ArcTypedOp::Add(ArcTypedComponent::new(group)));
        self
    }

    #[fn_with]
    pub fn alter_groups(&mut self, op: ArcTypedOp<Group>) -> &mut Self {
        self.groups.alter_value(op);
        self
    }

    // Megamenu GETTERS.

    pub fn groups(&self) -> &TypedComponents<Group> {
        &self.groups
    }
}
