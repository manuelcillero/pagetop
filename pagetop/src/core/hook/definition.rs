pub use std::any::Any as AnyHook;

pub trait HookTrait: AnyHook + Send + Sync {
    fn new() -> Self where Self: Sized;

    fn handler(&self) -> &'static str;

    fn weight(&self) -> isize {
        0
    }

    fn as_ref_any(&self) -> &dyn AnyHook;
}

pub fn action_ref<A: 'static>(hook: &dyn HookTrait) -> &A {
    hook.as_ref_any().downcast_ref::<A>().unwrap()
}
