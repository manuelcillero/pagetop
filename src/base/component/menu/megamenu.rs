use crate::prelude::*;

use super::Group;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Megamenu {
    id    : OptionId,
    groups: MixedComponents,
}

impl ComponentTrait for Megamenu {
    fn new() -> Self {
        Megamenu::default()
    }

    fn id(&self) -> Option<String> {
        self.id.get()
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
    pub fn set_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.id.set_value(id);
        self
    }

    #[fn_builder]
    pub fn set_groups(&mut self, op: TypedOp<Group>) -> &mut Self {
        self.groups.set_typed(op);
        self
    }

    #[rustfmt::skip]
    pub fn add_group(mut self, group: Group) -> Self {
        self.groups.set_value(AnyOp::Add(AnyComponent::with(group)));
        self
    }

    // Megamenu GETTERS.

    pub fn groups(&self) -> &MixedComponents {
        &self.groups
    }
}
