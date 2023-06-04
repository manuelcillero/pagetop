use crate::{define_handle, Handle};

pub use std::any::Any as AnyHookAction;

define_handle!(HOOK_UNNAMED);

pub trait HookActionTrait: AnyHookAction + Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn handle(&self) -> Handle {
        HOOK_UNNAMED
    }

    fn weight(&self) -> isize {
        0
    }

    fn as_ref_any(&self) -> &dyn AnyHookAction;
}

pub fn action_ref<A: 'static>(action: &dyn HookActionTrait) -> &A {
    action.as_ref_any().downcast_ref::<A>().unwrap()
}
