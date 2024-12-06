use crate::core::AnyBase;
use crate::{UniqueId, Weight};

pub type ActionBox = Box<dyn ActionTrait>;

#[derive(Eq, PartialEq, Hash)]
pub struct ActionKey {
    action_type_id: UniqueId,
    layout_type_id: Option<UniqueId>,
    referer_type_id: Option<UniqueId>,
    referer_id: Option<String>,
}

impl ActionKey {
    pub fn new(
        action_type_id: UniqueId,
        layout_type_id: Option<UniqueId>,
        referer_type_id: Option<UniqueId>,
        referer_id: Option<String>,
    ) -> Self {
        ActionKey {
            action_type_id,
            layout_type_id,
            referer_type_id,
            referer_id,
        }
    }
}

pub trait ActionBase {
    fn key(&self) -> ActionKey;
}

pub trait ActionTrait: ActionBase + AnyBase + Send + Sync {
    fn layout_type_id(&self) -> Option<UniqueId> {
        None
    }

    fn referer_type_id(&self) -> Option<UniqueId> {
        None
    }

    fn referer_id(&self) -> Option<String> {
        None
    }

    fn weight(&self) -> Weight {
        0
    }
}

impl<A: ActionTrait> ActionBase for A {
    fn key(&self) -> ActionKey {
        ActionKey {
            action_type_id: self.type_id(),
            layout_type_id: self.layout_type_id(),
            referer_type_id: self.referer_type_id(),
            referer_id: self.referer_id(),
        }
    }
}
