use crate::core::AnyBase;
use crate::{TypeId, Weight};

pub trait ActionTrait: AnyBase + Send + Sync {
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
