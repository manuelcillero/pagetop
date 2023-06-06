use crate::{define_handle, Handle};

pub use std::any::Any as AnyAction;

define_handle!(ACTION_UNNAMED);

pub trait ActionTrait: AnyAction + Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn handle(&self) -> Handle {
        ACTION_UNNAMED
    }

    fn weight(&self) -> isize {
        0
    }

    fn as_ref_any(&self) -> &dyn AnyAction;
}

pub fn action_ref<A: 'static>(action: &dyn ActionTrait) -> &A {
    action.as_ref_any().downcast_ref::<A>().unwrap()
}
