//! Define clases para aplicar en componentes del tema.
//!
//! Incluyen puntos de ruptura, colores y niveles de opacidad, escalas de tamaño y lados, necesarios
//! para crear determinadas clases del tema:
//!
//! ```rust,no_run
//! use pagetop_bootsier::theme::*;
//!
//! let bg = class::Bg::with(ThemeColor::Primary);
//! let border = class::Border::new()
//!     .with_side(BoxSide::Top, ScaleSize::Zero)
//!     .with_color(ThemeColor::Danger);
//! ```

mod color;
pub use color::{Bg, BgColor};
pub use color::{Text, TextColor};

mod button;
pub use button::{ButtonColor, ButtonColorStyle, ButtonSize, ButtonSizeKind};

mod border;
pub use border::{Border, BorderColor};

mod rounded;
pub use rounded::{Rounded, RoundedRadius};

mod layout;
pub use layout::{Margin, Padding};
