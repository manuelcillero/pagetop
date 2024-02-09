use crate::prelude::*;

pub type FnActionComponent<C> = fn(component: &mut C, cx: &mut Context);

mod before_prepare_component;
pub use before_prepare_component::*;

mod after_prepare_component;
pub use after_prepare_component::*;
