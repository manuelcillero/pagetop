use crate::core::AnyBase;
use crate::{TypeId, Weight};

pub type ActionBox = Box<dyn ActionTrait>;

#[derive(Eq, PartialEq, Hash)]
pub struct ActionKey {
    action_type_id: TypeId,
    theme_type_id: Option<TypeId>,
    referer_type_id: Option<TypeId>,
    referer_id: Option<String>,
}

impl ActionKey {
    pub fn new(
        action_type_id: TypeId,
        theme_type_id: Option<TypeId>,
        referer_type_id: Option<TypeId>,
        referer_id: Option<String>,
    ) -> Self {
        ActionKey {
            action_type_id,
            theme_type_id,
            referer_type_id,
            referer_id,
        }
    }
}

pub trait ActionBase {
    fn key(&self) -> ActionKey;
}

pub trait ActionTrait: ActionBase + AnyBase + Send + Sync {
    fn theme_type_id(&self) -> Option<TypeId> {
        None
    }

    fn referer_type_id(&self) -> Option<TypeId> {
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
            theme_type_id: self.theme_type_id(),
            referer_type_id: self.referer_type_id(),
            referer_id: self.referer_id(),
        }
    }
}
