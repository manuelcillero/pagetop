use crate::{concat_string, SmartDefault};

// About pixels: Pixels (px) are relative to the viewing device. For low-dpi devices, 1px is one
// device pixel (dot) of the display. For printers and high resolution screens 1px implies multiple
// device pixels.

// About em: 2em means 2 times the size of the current font. The em and rem units are practical in
// creating perfectly scalable layout!

// About viewport: If the browser window size is 50cm wide, 1vw = 0.5cm.

#[rustfmt::skip]
#[derive(SmartDefault)]
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
impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::None       => "".to_owned(),
            Value::Auto       => "auto".to_owned(),
            // Absolute value.
            Value::Cm(av)     => concat_string!(av.to_string(), "cm"),
            Value::In(av)     => concat_string!(av.to_string(), "in"),
            Value::Mm(av)     => concat_string!(av.to_string(), "mm"),
            Value::Pc(av)     => concat_string!(av.to_string(), "pc"),
            Value::Pt(av)     => concat_string!(av.to_string(), "pt"),
            Value::Px(av)     => concat_string!(av.to_string(), "px"),
            // Relative value.
            Value::RelEm(rv)  => concat_string!(rv.to_string(), "em"),
            Value::RelPct(rv) => concat_string!(rv.to_string(), "%"),
            Value::RelRem(rv) => concat_string!(rv.to_string(), "rem"),
            Value::RelVh(rv)  => concat_string!(rv.to_string(), "vh"),
            Value::RelVw(rv)  => concat_string!(rv.to_string(), "vw"),
        }
    }
}
