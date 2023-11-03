use crate::{Handle, HasHandle, Weight};

use std::any::Any;

pub trait ActionBase: Any {
    fn as_ref_any(&self) -> &dyn Any;
}

pub trait ActionTrait: ActionBase + HasHandle + Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn referer_handle(&self) -> Option<Handle> {
        None
    }

    fn weight(&self) -> Weight {
        0
    }
}

impl<C: ActionTrait> ActionBase for C {
    fn as_ref_any(&self) -> &dyn Any {
        self
    }
}

pub fn action_ref<A: 'static>(action: &dyn ActionTrait) -> &A {
    action.as_ref_any().downcast_ref::<A>().unwrap()
}
