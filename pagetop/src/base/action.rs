use crate::prelude::*;

pub type FnActionWithComponent<C> = fn(component: &mut C, cx: &mut Context);

pub type FnActionWithPage = fn(page: &mut Page);

pub mod component;

pub mod theme;

pub mod page;
