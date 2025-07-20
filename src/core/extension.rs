//! API para añadir nuevas funcionalidades usando extensiones.
//!
//! Cada funcionalidad adicional que quiera incorporarse a una aplicación `PageTop` se debe modelar
//! como una **extensión**. Todas comparten la misma interfaz declarada en [`ExtensionTrait`].

mod definition;
pub use definition::{ExtensionRef, ExtensionTrait};

pub(crate) mod all;
