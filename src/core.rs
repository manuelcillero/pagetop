//! Key types and functions for creating actions, components, packages, and themes.

use crate::util;

use std::any::Any;

// Common definitions for core types.
pub trait AnyBase: Any {
    fn single_name(&self) -> &'static str;

    fn as_any_ref(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AnyBase for T {
    fn single_name(&self) -> &'static str {
        util::single_type_name::<Self>()
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// API to define functions that alter the behavior of PageTop core.
pub mod action;

// API to build new components.
pub mod component;

// API to add new features with packages.
pub mod package;

// API to add new layouts with themes.
pub mod theme;
