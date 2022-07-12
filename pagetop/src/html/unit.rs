use crate::concat_string;

// About pixels: Pixels (px) are relative to the viewing device. For low-dpi
// devices, 1px is one device pixel (dot) of the display. For printers and high
// resolution screens 1px implies multiple device pixels.

// About em: 2em means 2 times the size of the current font. The em and rem
// units are practical in creating perfectly scalable layout!

// About viewport: If the browser window size is 50cm wide, 1vw = 0.5cm.

pub enum UnitValue {
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

    UnSet,
}

impl ToString for UnitValue {
    fn to_string(&self) -> String {
        match self {
            UnitValue::Auto       => "auto".to_owned(),
            // Absolute value.
            UnitValue::Cm(aw)     => concat_string!(aw.to_string(), "cm"),
            UnitValue::In(aw)     => concat_string!(aw.to_string(), "in"),
            UnitValue::Mm(aw)     => concat_string!(aw.to_string(), "mm"),
            UnitValue::Pc(aw)     => concat_string!(aw.to_string(), "pc"),
            UnitValue::Pt(aw)     => concat_string!(aw.to_string(), "pt"),
            UnitValue::Px(aw)     => concat_string!(aw.to_string(), "px"),
            // Relative value.
            UnitValue::RelEm(rw)  => concat_string!(rw.to_string(), "em"),
            UnitValue::RelPct(rw) => concat_string!(rw.to_string(), "%"),
            UnitValue::RelRem(rw) => concat_string!(rw.to_string(), "rem"),
            UnitValue::RelVh(rw)  => concat_string!(rw.to_string(), "vh"),
            UnitValue::RelVw(rw)  => concat_string!(rw.to_string(), "vw"),

            _ => "".to_owned(),
        }
    }
}
