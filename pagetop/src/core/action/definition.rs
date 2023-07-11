use crate::Handle;

use std::any::Any;

pub trait BaseAction: Any {
    fn as_ref_any(&self) -> &dyn Any;
}

pub trait ActionTrait: BaseAction + Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn handle(&self) -> Handle;

    fn weight(&self) -> isize {
        0
    }
}

impl<C: ActionTrait> BaseAction for C {
    fn as_ref_any(&self) -> &dyn Any {
        self
    }
}

pub fn action_ref<A: 'static>(action: &dyn ActionTrait) -> &A {
    action.as_ref_any().downcast_ref::<A>().unwrap()
}
