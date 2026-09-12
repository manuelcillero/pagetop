//! Márgenes y relleno nativos aplicados a componentes.
//!
//! [`Margin`] y [`Padding`] configuran márgenes externos y relleno interno por lado lógico y punto
//! de corte. Aplican sobre cualquier componente. Se usan igual que [`FlexItem`]: pasándolos
//! directamente al `with_prop()` que normalmente ya expone cualquier componente, gracias a su
//! `From` hacia [`PropsOp`].
//!
//! # Ejemplo
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//!
//! // Margen exterior arriba/abajo y relleno interno por los cuatro lados.
//! let card = Container::new()
//!     .with_prop(Margin::new().with_y(UnitValue::RelRem(1.0)))
//!     .with_prop(Padding::new().with_all(UnitValue::RelRem(1.5)))
//!     .with_child(Button::plain(Lc::n("Aceptar")));
//! ```
//!
//! [`FlexItem`]: crate::html::flex::FlexItem
//! [`PropsOp`]: crate::html::props::PropsOp

mod margin;
pub use margin::Margin;

mod padding;
pub use padding::Padding;
