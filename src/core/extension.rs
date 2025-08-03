//! API para añadir nuevas funcionalidades usando extensiones.
//!
//! Cada funcionalidad adicional que quiera incorporarse a una aplicación `PageTop` se debe modelar
//! como una **extensión**. Todas comparten la misma interfaz declarada en [`Extension`].

mod definition;
pub use definition::{Extension, ExtensionRef};

pub(crate) mod all;
