use crate::prelude::*;

pub type FnActionWithComponent<C> = fn(component: &mut C, cx: &mut Context);

pub mod component;

pub mod layout;

pub mod page;
