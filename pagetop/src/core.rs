//! Tipos y funciones esenciales para crear acciones, componentes, diseños y paquetes.

use crate::util::TypeInfo;

use std::any::Any;

/// A base trait that extends `Any` to provide metadata and dynamic type casting capabilities.
pub trait AnyBase: Any {
    /// Returns the full name of the type.
    fn type_name(&self) -> &'static str;

    /// Returns a short name for the type.
    fn short_name(&self) -> &'static str;

    /// Returns a reference to `dyn Any` for dynamic type casting.
    fn as_any_ref(&self) -> &dyn Any;

    /// Returns a mutable reference to `dyn Any` for dynamic type casting.
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

/// A trait for advanced dynamic type manipulation and downcasting.
pub trait AnyTo: AnyBase {
    /// Checks if the type is of the specified type `T`.
    #[inline]
    fn is<T>(&self) -> bool
    where
        T: AnyBase,
    {
        self.as_any_ref().is::<T>()
    }

    /// Attempts to downcast a reference to the specified type `T`.
    #[inline]
    fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: AnyBase,
    {
        self.as_any_ref().downcast_ref()
    }

    /// Attempts to downcast a mutable reference to the specified type `T`.
    #[inline]
    fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: AnyBase,
    {
        self.as_any_mut().downcast_mut()
    }
}

impl<T: ?Sized + AnyBase> AnyTo for T {}

// API to define functions that alter the predefined behavior of the code.
pub mod action;

// API to build new components.
pub mod component;

// API to add new features with packages.
pub mod package;

// API to add new themes.
pub mod theme;
