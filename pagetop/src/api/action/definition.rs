use crate::util;

pub use std::any::Any as AnyAction;

pub trait BaseAction {
    fn type_name(&self) -> &'static str;

    fn single_name(&self) -> &'static str;

    fn qualified_name(&self, last: u8) -> &'static str;
}

pub trait ActionTrait: AnyAction + BaseAction + Send + Sync {
    fn new() -> Self where Self: Sized;

    fn weight(&self) -> i8 {
        0
    }

    fn as_ref_any(&self) -> &dyn AnyAction;
}

impl<C: ?Sized + ActionTrait> BaseAction for C {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn single_name(&self) -> &'static str {
        util::partial_type_name(std::any::type_name::<Self>(), 1)
    }

    fn qualified_name(&self, last: u8) -> &'static str {
        util::partial_type_name(std::any::type_name::<Self>(), last)
    }
}

pub fn action_ref<A: 'static>(action: &dyn ActionTrait) -> &A {
    action.as_ref_any().downcast_ref::<A>().unwrap()
}
