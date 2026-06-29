//! Tipos enumerados para construir clases del tema.
//!
//! Incluyen puntos de ruptura, colores, escalas de tamaño, lados, etc. Se pueden importar
//! globalmente junto al resto del tema:
//!
//! ```rust,no_run
//! use pagetop_bootsier::theme::*;
//!
//! let bg = class::Background::with(token::Color::Primary);
//! let border = class::Border::new()
//!     .with_side(token::Side::Top, token::ScaleSize::Zero)
//!     .with_color(token::Color::Danger);
//! ```

mod breakpoint;
pub use breakpoint::BreakPoint;

mod color;
pub use color::{Color, ColorBg, ColorText, Opacity};

mod border;
pub use border::ColorBorder;

mod rounded;
pub use rounded::RoundedRadius;

mod layout;
pub use layout::{ScaleSize, Side};
