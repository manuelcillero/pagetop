//! Key types and functions for creating actions, components, packages, and themes.

use crate::global::TypeInfo;

use std::any::Any;

// Common definitions for core types.
pub trait AnyBase: Any {
    fn type_name(&self) -> &'static str;

    fn short_name(&self) -> &'static str;

    fn as_any_ref(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[allow(clippy::inline_always)]
impl<T: Any> AnyBase for T {
    #[inline(always)]
    fn type_name(&self) -> &'static str {
        TypeInfo::FullName.of::<T>()
    }

    #[inline(always)]
    fn short_name(&self) -> &'static str {
        TypeInfo::ShortName.of::<T>()
    }

    #[inline(always)]
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    #[inline(always)]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait AnyTo: AnyBase {
    #[inline]
    fn is<T>(&self) -> bool
    where
        T: AnyBase,
    {
        self.as_any_ref().is::<T>()
    }

    #[inline]
    fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: AnyBase,
    {
        self.as_any_ref().downcast_ref()
    }

    #[inline]
    fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: AnyBase,
    {
        self.as_any_mut().downcast_mut()
    }
}

impl<T: ?Sized + AnyBase> AnyTo for T {}

// API to define functions that alter the behavior of PageTop core.
pub mod action;

// API to add new features with packages.
pub mod package;
