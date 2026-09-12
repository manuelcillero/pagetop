//! Márgenes y relleno nativos aplicados a componentes.
//!
//! [`Margin`] y [`Padding`] configuran márgenes externos y relleno interno por lado lógico y punto
//! de corte. Aplican sobre cualquier componente. Se usan igual que [`FlexItem`], con
//! [`PropsOp::margin()`] y [`PropsOp::padding()`] sobre el `with_prop()` que normalmente ya expone
//! cualquier componente.
//!
//! # Ejemplo
//!
//! ```rust,no_run
//! use pagetop::prelude::*;
//!
//! // Margen exterior arriba/abajo y relleno interno por los cuatro lados.
//! // `Margin` y `Padding` implementan `From` para `PropsOp`, así que `with_prop()`
//! // acepta `.into()` en vez de `PropsOp::margin()`/`PropsOp::padding()`.
//! let card = Container::new()
//!     .with_prop(Margin::new().with_y(UnitValue::RelRem(1.0)).into())
//!     .with_prop(Padding::new().with_all(UnitValue::RelRem(1.5)).into())
//!     .with_child(Button::plain(Lc::n("Aceptar")));
//! ```
//!
//! [`FlexItem`]: crate::html::flex::FlexItem
//! [`PropsOp::margin()`]: crate::html::props::PropsOp::margin
//! [`PropsOp::padding()`]: crate::html::props::PropsOp::padding

mod margin;
pub use margin::Margin;

mod padding;
pub use padding::Padding;
