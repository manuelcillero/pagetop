use pagetop::prelude::*;

use crate::theme::token::{BreakPoint, ScaleSize, Side};

// **< Margin >*************************************************************************************

/// Clases para establecer **margin** por lado, tamaño y punto de ruptura.
///
/// # Ejemplos
///
/// ```rust
/// use pagetop_bootsier::theme::*;
///
/// let m = class::Margin::with(token::Side::Top, token::ScaleSize::Three);
/// assert_eq!(m.to_class(), "mt-3");
///
/// let m = class::Margin::with(token::Side::Start, token::ScaleSize::Auto)
///     .with_breakpoint(token::BreakPoint::LG);
/// assert_eq!(m.to_class(), "ms-lg-auto");
///
/// let m = class::Margin::with(token::Side::All, token::ScaleSize::None);
/// assert_eq!(m.to_class(), "");
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub struct Margin {
    side: Side,
    size: ScaleSize,
    breakpoint: BreakPoint,
}

impl Margin {
    /// Crea un **margin** indicando lado(s) y tamaño. Por defecto no se aplica a ningún punto de
    /// ruptura.
    pub fn with(side: Side, size: ScaleSize) -> Self {
        Margin {
            side,
            size,
            breakpoint: BreakPoint::None,
        }
    }

    // **< Margin BUILDER >*************************************************************************

    /// Establece el punto de ruptura a partir del cual se empieza a aplicar el **margin**.
    pub fn with_breakpoint(mut self, breakpoint: BreakPoint) -> Self {
        self.breakpoint = breakpoint;
        self
    }

    // **< Margin HELPERS >*************************************************************************

    // Devuelve el prefijo `m*` según el lado.
    #[rustfmt::skip]
    #[inline]
    const fn side_prefix(&self) -> &'static str {
        match self.side {
            Side::All          => "m",
            Side::Top          => "mt",
            Side::Bottom       => "mb",
            Side::Start        => "ms",
            Side::End          => "me",
            Side::LeftAndRight => "mx",
            Side::TopAndBottom => "my",
        }
    }

    // Devuelve el sufijo del tamaño (`auto`, `0`..`5`), o `None` si no define clase.
    #[rustfmt::skip]
    #[inline]
    const fn size_suffix(&self) -> Option<&'static str> {
        match self.size {
            ScaleSize::None  => None,
            ScaleSize::Auto  => Some("auto"),
            ScaleSize::Zero  => Some("0"),
            ScaleSize::One   => Some("1"),
            ScaleSize::Two   => Some("2"),
            ScaleSize::Three => Some("3"),
            ScaleSize::Four  => Some("4"),
            ScaleSize::Five  => Some("5"),
        }
    }

    /// Añade la clase de **margin** a la cadena de clases.
    pub fn push_to(self, classes: &mut String) {
        if let Some(size) = self.size_suffix() {
            let side = self.side_prefix();
            self.breakpoint.push_to(classes, side, size);
        }
    }

    /// Devuelve la clase de *margin* como cadena (`"mt-3"`, `"ms-lg-auto"`, etc.).
    ///
    /// Si `size` es `ScaleSize::None`, devuelve `""`.
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class);
        class
    }
}

impl Into<CowStr> for Margin {
    /// Permite pasar [`Margin`] directamente a [`PropsOp`](pagetop::prelude::PropsOp).
    fn into(self) -> CowStr {
        self.to_class().into()
    }
}

// **< Padding >************************************************************************************

/// Clases para establecer **padding** por lado, tamaño y punto de ruptura.
///
/// # Ejemplos
///
/// ```rust
/// use pagetop_bootsier::theme::*;
///
/// let p = class::Padding::with(token::Side::LeftAndRight, token::ScaleSize::Two);
/// assert_eq!(p.to_class(), "px-2");
///
/// let p = class::Padding::with(token::Side::End, token::ScaleSize::Four)
///     .with_breakpoint(token::BreakPoint::SM);
/// assert_eq!(p.to_class(), "pe-sm-4");
///
/// let p = class::Padding::with(token::Side::All, token::ScaleSize::Auto);
/// assert_eq!(p.to_class(), ""); // `Auto` no aplica a padding.
/// ```
#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub struct Padding {
    side: Side,
    size: ScaleSize,
    breakpoint: BreakPoint,
}

impl Padding {
    /// Crea un **padding** indicando lado(s) y tamaño. Por defecto no se aplica a ningún punto de
    /// ruptura.
    pub fn with(side: Side, size: ScaleSize) -> Self {
        Padding {
            side,
            size,
            breakpoint: BreakPoint::None,
        }
    }

    // **< Padding BUILDER >************************************************************************

    /// Establece el punto de ruptura a partir del cual se empieza a aplicar el **padding**.
    pub fn with_breakpoint(mut self, breakpoint: BreakPoint) -> Self {
        self.breakpoint = breakpoint;
        self
    }

    // **< Padding HELPERS >************************************************************************

    // Devuelve el prefijo `p*` según el lado.
    #[rustfmt::skip]
    #[inline]
    const fn side_prefix(&self) -> &'static str {
        match self.side {
            Side::All          => "p",
            Side::Top          => "pt",
            Side::Bottom       => "pb",
            Side::Start        => "ps",
            Side::End          => "pe",
            Side::LeftAndRight => "px",
            Side::TopAndBottom => "py",
        }
    }

    // Devuelve el sufijo del tamaño (`0`..`5`), o None si no define clase.
    //
    // Nota: `ScaleSize::Auto` **no aplica** a *padding* => devuelve `None`.
    #[rustfmt::skip]
    #[inline]
    const fn size_suffix(&self) -> Option<&'static str> {
        match self.size {
            ScaleSize::None  => None,
            ScaleSize::Auto  => None,
            ScaleSize::Zero  => Some("0"),
            ScaleSize::One   => Some("1"),
            ScaleSize::Two   => Some("2"),
            ScaleSize::Three => Some("3"),
            ScaleSize::Four  => Some("4"),
            ScaleSize::Five  => Some("5"),
        }
    }

    /// Añade la clase de **padding** a la cadena de clases.
    pub fn push_to(self, classes: &mut String) {
        if let Some(size) = self.size_suffix() {
            let side = self.side_prefix();
            self.breakpoint.push_to(classes, side, size);
        }
    }

    /// Devuelve la clase de *padding* como cadena (`"px-2"`, `"pe-sm-4"`, etc.).
    ///
    /// Si `size` es `ScaleSize::None` o `ScaleSize::Auto`, devuelve `""`.
    pub fn to_class(self) -> String {
        let mut class = String::new();
        self.push_to(&mut class);
        class
    }
}

impl Into<CowStr> for Padding {
    /// Permite pasar [`Padding`] directamente a [`PropsOp`](pagetop::prelude::PropsOp).
    fn into(self) -> CowStr {
        self.to_class().into()
    }
}
