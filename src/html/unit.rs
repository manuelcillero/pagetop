use crate::AutoDefault;

use std::fmt;

// About pixels: Pixels (px) are relative to the viewing device. For low-dpi devices, 1px is one
// device pixel (dot) of the display. For printers and high resolution screens 1px implies multiple
// device pixels.

// About em: 2em means 2 times the size of the current font. The em and rem units are practical in
// creating perfectly scalable layout!

// About viewport: If the browser window size is 50cm wide, 1vw = 0.5cm.

#[rustfmt::skip]
#[derive(AutoDefault)]
pub enum Value {
    #[default]
    None,
    Auto,

    Cm(isize),      // Centimeters.
    In(isize),      // Inches (1in = 96px = 2.54cm).
    Mm(isize),      // Millimeters.
    Pc(isize),      // Picas (1pc = 12pt).
    Pt(isize),      // Points (1pt = 1/72 of 1in).
    Px(isize),      // Pixels (1px = 1/96th of 1in).

    RelEm(f32),     // Relative to the font-size of the element.
    RelPct(f32),    // Percentage relative to the parent element.
    RelRem(f32),    // Relative to font-size of the root element.
    RelVh(f32),     // Relative to 1% of the height of the viewport.
    RelVw(f32),     // Relative to 1% of the value of the viewport.
}

#[rustfmt::skip]
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::None       => write!(f, ""),
            Value::Auto       => write!(f, "auto"),
            // Absolute value.
            Value::Cm(av)     => write!(f, "{av}cm"),
            Value::In(av)     => write!(f, "{av}in"),
            Value::Mm(av)     => write!(f, "{av}mm"),
            Value::Pc(av)     => write!(f, "{av}pc"),
            Value::Pt(av)     => write!(f, "{av}pt"),
            Value::Px(av)     => write!(f, "{av}px"),
            // Relative value.
            Value::RelEm(rv)  => write!(f, "{rv}em"),
            Value::RelPct(rv) => write!(f, "{rv}%"),
            Value::RelRem(rv) => write!(f, "{rv}rem"),
            Value::RelVh(rv)  => write!(f, "{rv}vh"),
            Value::RelVw(rv)  => write!(f, "{rv}vw"),
        }
    }
}
