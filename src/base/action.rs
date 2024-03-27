use crate::prelude::*;

pub type FnActionWithComponent<C> = fn(component: &mut C, cx: &mut Context);

pub mod page;

pub mod theme;

pub mod component;
