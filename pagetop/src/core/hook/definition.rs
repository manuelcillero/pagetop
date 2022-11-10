use crate::util::Handle;

pub use std::any::Any as AnyHookAction;

pub trait HookActionTrait: AnyHookAction + Send + Sync {
    fn new() -> Self
    where
        Self: Sized;

    fn handle(&self) -> Handle;

    fn weight(&self) -> isize {
        0
    }

    fn as_ref_any(&self) -> &dyn AnyHookAction;
}

pub fn action_ref<A: 'static>(action: &dyn HookActionTrait) -> &A {
    action.as_ref_any().downcast_ref::<A>().unwrap()
}
