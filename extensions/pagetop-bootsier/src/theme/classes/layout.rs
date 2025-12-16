use pagetop::prelude::*;

use crate::theme::aux::{ScaleSize, Side};
use crate::theme::BreakPoint;

// **< Margin >*************************************************************************************

/// Clases para establecer **margin** por lado, tamaño y punto de ruptura.
///
/// # Ejemplos
///
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let m = classes::Margin::with(Side::Top, ScaleSize::Three);
/// assert_eq!(m.to_class(), "mt-3");
///
/// let m = classes::Margin::with(Side::Start, ScaleSize::Auto).with_breakpoint(BreakPoint::LG);
/// assert_eq!(m.to_class(), "ms-lg-auto");
///
/// let m = classes::Margin::with(Side::All, ScaleSize::None);
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

    /// Devuelve el prefijo `m*` según el lado.
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

    /// Devuelve el sufijo del tamaño (`auto`, `0`..`5`), o `None` si no define clase.
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

    /* Añade la clase de **margin** a la cadena de clases (reservado).
    ///
    /// No añade nada si `size` es `ScaleSize::None`.
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        let Some(size) = self.size_suffix() else {
            return;
        };
        self.breakpoint
            .push_class(classes, self.side_prefix(), size);
    } */

    /// Devuelve la clase de **margin** como cadena (`"mt-3"`, `"ms-lg-auto"`, etc.).
    ///
    /// Si `size` es `ScaleSize::None`, devuelve `""`.
    #[inline]
    pub fn to_class(self) -> String {
        let Some(size) = self.size_suffix() else {
            return String::new();
        };
        self.breakpoint.class_with(self.side_prefix(), size)
    }
}

// **< Padding >************************************************************************************

/// Clases para establecer **padding** por lado, tamaño y punto de ruptura.
///
/// # Ejemplos
///
/// ```rust
/// # use pagetop_bootsier::prelude::*;
/// let p = classes::Padding::with(Side::LeftAndRight, ScaleSize::Two);
/// assert_eq!(p.to_class(), "px-2");
///
/// let p = classes::Padding::with(Side::End, ScaleSize::Four).with_breakpoint(BreakPoint::SM);
/// assert_eq!(p.to_class(), "pe-sm-4");
///
/// let p = classes::Padding::with(Side::All, ScaleSize::Auto);
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

    /// Devuelve el prefijo `p*` según el lado.
    #[rustfmt::skip]
    #[inline]
    const fn prefix(&self) -> &'static str {
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

    /// Devuelve el sufijo del tamaño (`0`..`5`), o `None` si no define clase.
    ///
    /// Nota: `ScaleSize::Auto` **no aplica** a padding ⇒ devuelve `None`.
    #[rustfmt::skip]
    #[inline]
    const fn suffix(&self) -> Option<&'static str> {
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

    /* Añade la clase de **padding** a la cadena de clases (reservado).
    ///
    /// No añade nada si `size` es `ScaleSize::None` o `ScaleSize::Auto`.
    #[inline]
    pub(crate) fn push_class(self, classes: &mut String) {
        let Some(size) = self.suffix() else {
            return;
        };
        self.breakpoint.push_class(classes, self.prefix(), size);
    } */

    /// Devuelve la clase de **padding** como cadena (`"px-2"`, `"pe-sm-4"`, etc.).
    ///
    /// Si `size` es `ScaleSize::None` o `ScaleSize::Auto`, devuelve `""`.
    #[inline]
    pub fn to_class(self) -> String {
        let Some(size) = self.suffix() else {
            return String::new();
        };
        self.breakpoint.class_with(self.prefix(), size)
    }
}
