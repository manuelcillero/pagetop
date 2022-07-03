use crate::concat_string;

#[derive(Clone, Copy, PartialEq)]
pub enum LayoutProperty {
    MarginBottom,
    MarginLeft,
    MarginRight,
    MarginTop,
    PaddingBottom,
    PaddingLeft,
    PaddingRight,
    PaddingTop,
}

impl std::convert::AsRef<str> for LayoutProperty {
    fn as_ref(&self) -> &str {
        match *self {
            LayoutProperty::MarginBottom  => "margin-bottom",
            LayoutProperty::MarginLeft    => "margin-left",
            LayoutProperty::MarginRight   => "margin-right",
            LayoutProperty::MarginTop     => "margin-top",
            LayoutProperty::PaddingBottom => "padding-bottom",
            LayoutProperty::PaddingLeft   => "padding-left",
            LayoutProperty::PaddingRight  => "padding-right",
            LayoutProperty::PaddingTop    => "padding-top",
        }
    }
}

// About pixels: Pixels (px) are relative to the viewing device. For low-dpi
// devices, 1px is one device pixel (dot) of the display. For printers and high
// resolution screens 1px implies multiple device pixels.

// About em: 2em means 2 times the size of the current font. The em and rem
// units are practical in creating perfectly scalable layout!

// About viewport: If the browser window size is 50cm wide, 1vw = 0.5cm.

#[derive(PartialEq)]
pub enum LayoutUnit {
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
    RelVw(f32),     // Relative to 1% of the width of the viewport.

    UnSet,
}

impl LayoutUnit {
    fn to_inline(&self, property: LayoutProperty) -> String {
        match self {
            LayoutUnit::Auto          => concat_string!(property, ":auto;"),

            LayoutUnit::Cm(value)     => concat_string!(property, ":", value.to_string(), "cm;"),
            LayoutUnit::In(value)     => concat_string!(property, ":", value.to_string(), "in;"),
            LayoutUnit::Mm(value)     => concat_string!(property, ":", value.to_string(), "mm;"),
            LayoutUnit::Pc(value)     => concat_string!(property, ":", value.to_string(), "pc;"),
            LayoutUnit::Pt(value)     => concat_string!(property, ":", value.to_string(), "pt;"),
            LayoutUnit::Px(value)     => concat_string!(property, ":", value.to_string(), "px;"),

            LayoutUnit::RelEm(value)  => concat_string!(property, ":", value.to_string(), "em;"),
            LayoutUnit::RelPct(value) => concat_string!(property, ":", value.to_string(), "%;"),
            LayoutUnit::RelRem(value) => concat_string!(property, ":", value.to_string(), "rem;"),
            LayoutUnit::RelVh(value)  => concat_string!(property, ":", value.to_string(), "vh;"),
            LayoutUnit::RelVw(value)  => concat_string!(property, ":", value.to_string(), "vw;"),

            _ => "".to_owned(),
        }
    }
}

struct Style {
    property: LayoutProperty,
    inline  : String,
}

pub struct Layout(Vec<Style>);

impl Layout {
    pub fn new() -> Self {
        Layout(Vec::new())
    }

    pub fn add(&mut self, property: LayoutProperty, value: LayoutUnit) -> &Self {
        match self.0.iter().position(|s| s.property.eq(&property)) {
            Some(pos) => {
                self.0.remove(pos);
                if value != LayoutUnit::UnSet {
                    self.0.insert(pos, Style {
                        property,
                        inline: value.to_inline(property),
                    });
                }
            },
            _ => if value != LayoutUnit::UnSet {
                self.0.push(Style {
                    property,
                    inline: value.to_inline(property),
                });
            }
        }
        self
    }

    pub fn get(&self) -> Option<String> {
        if self.0.len() == 0 {
            None
        } else {
            let mut inline = "".to_owned();
            self.0.iter().for_each(|s| inline.push_str(s.inline.as_str()));
            Some(inline)
        }
    }
}
