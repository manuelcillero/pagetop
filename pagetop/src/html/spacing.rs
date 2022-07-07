use crate::concat_string;

const MARGIN_BOTTOM:  &str = "margin-bottom";
const MARGIN_LEFT:    &str = "margin-left";
const MARGIN_RIGHT:   &str = "margin-right";
const MARGIN_TOP:     &str = "margin-top";

const PADDING_BOTTOM: &str = "padding-bottom";
const PADDING_LEFT:   &str = "padding-left";
const PADDING_RIGHT:  &str = "padding-right";
const PADDING_TOP:    &str = "padding-top";

struct SpaceStyle {
    property: String,
    inline  : String,
}

pub struct Spaces(Vec<SpaceStyle>);

// About pixels: Pixels (px) are relative to the viewing device. For low-dpi
// devices, 1px is one device pixel (dot) of the display. For printers and high
// resolution screens 1px implies multiple device pixels.

// About em: 2em means 2 times the size of the current font. The em and rem
// units are practical in creating perfectly scalable layout!

// About viewport: If the browser window size is 50cm wide, 1vw = 0.5cm.

pub enum SpaceValue {
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

impl SpaceValue {
    fn add(&self, property: &str, into_spaces: &mut Spaces) {
        let style = SpaceStyle {
            property: property.to_owned(),
            inline  : match self {
                SpaceValue::Auto       => concat_string!(property, ":auto;"),
                // Absolute value.
                SpaceValue::Cm(aw)     => concat_string!(property, ":", aw.to_string(), "cm;"),
                SpaceValue::In(aw)     => concat_string!(property, ":", aw.to_string(), "in;"),
                SpaceValue::Mm(aw)     => concat_string!(property, ":", aw.to_string(), "mm;"),
                SpaceValue::Pc(aw)     => concat_string!(property, ":", aw.to_string(), "pc;"),
                SpaceValue::Pt(aw)     => concat_string!(property, ":", aw.to_string(), "pt;"),
                SpaceValue::Px(aw)     => concat_string!(property, ":", aw.to_string(), "px;"),
                // Relative value.
                SpaceValue::RelEm(rw)  => concat_string!(property, ":", rw.to_string(), "em;"),
                SpaceValue::RelPct(rw) => concat_string!(property, ":", rw.to_string(), "%;"),
                SpaceValue::RelRem(rw) => concat_string!(property, ":", rw.to_string(), "rem;"),
                SpaceValue::RelVh(rw)  => concat_string!(property, ":", rw.to_string(), "vh;"),
                SpaceValue::RelVw(rw)  => concat_string!(property, ":", rw.to_string(), "vw;"),

                _ => "".to_owned(),
            }
        };
        match into_spaces.0.iter().position(|s| s.property.eq(&style.property)) {
            Some(pos) => {
                into_spaces.0.remove(pos);
                if !style.inline.is_empty() {
                    into_spaces.0.insert(pos, style);
                }
            },
            _ => if !style.inline.is_empty() {
                into_spaces.0.push(style)
            }
        }
    }
}

pub enum SpaceSet {
    Margin(SpaceValue, SpaceValue, SpaceValue, SpaceValue),
    MarginAll(SpaceValue),
    MarginBoth(SpaceValue, SpaceValue),
    MarginBottom(SpaceValue),
    MarginLeft(SpaceValue),
    MarginRight(SpaceValue),
    MarginTop(SpaceValue),
    Padding(SpaceValue, SpaceValue, SpaceValue, SpaceValue),
    PaddingAll(SpaceValue),
    PaddingBoth(SpaceValue, SpaceValue),
    PaddingBottom(SpaceValue),
    PaddingLeft(SpaceValue),
    PaddingRight(SpaceValue),
    PaddingTop(SpaceValue),
}

impl SpaceSet {
    fn add(&self, into_spaces: &mut Spaces) {
        match self {
            SpaceSet::Margin(top, right, bottom, left) => {
                   top.add(MARGIN_TOP,    into_spaces);
                 right.add(MARGIN_RIGHT,  into_spaces);
                bottom.add(MARGIN_BOTTOM, into_spaces);
                  left.add(MARGIN_LEFT,   into_spaces);
            },
            SpaceSet::MarginAll(value) => {
                value.add(MARGIN_TOP,    into_spaces);
                value.add(MARGIN_RIGHT,  into_spaces);
                value.add(MARGIN_BOTTOM, into_spaces);
                value.add(MARGIN_LEFT,   into_spaces);
            },
            SpaceSet::MarginBoth(top_bottom, right_left) => {
                top_bottom.add(MARGIN_TOP,    into_spaces);
                right_left.add(MARGIN_RIGHT,  into_spaces);
                top_bottom.add(MARGIN_BOTTOM, into_spaces);
                right_left.add(MARGIN_LEFT,   into_spaces);
            },

            SpaceSet::MarginBottom(value)  => value.add(MARGIN_BOTTOM,  into_spaces),
            SpaceSet::MarginLeft(value)    => value.add(MARGIN_LEFT,    into_spaces),
            SpaceSet::MarginRight(value)   => value.add(MARGIN_RIGHT,   into_spaces),
            SpaceSet::MarginTop(value)     => value.add(MARGIN_TOP,     into_spaces),

            SpaceSet::Padding(top, right, bottom, left) => {
                   top.add(PADDING_TOP,    into_spaces);
                 right.add(PADDING_RIGHT,  into_spaces);
                bottom.add(PADDING_BOTTOM, into_spaces);
                  left.add(PADDING_LEFT,   into_spaces);
            },
            SpaceSet::PaddingAll(value) => {
                value.add(PADDING_TOP,    into_spaces);
                value.add(PADDING_RIGHT,  into_spaces);
                value.add(PADDING_BOTTOM, into_spaces);
                value.add(PADDING_LEFT,   into_spaces);
            },
            SpaceSet::PaddingBoth(top_bottom, right_left) => {
                top_bottom.add(PADDING_TOP,    into_spaces);
                right_left.add(PADDING_RIGHT,  into_spaces);
                top_bottom.add(PADDING_BOTTOM, into_spaces);
                right_left.add(PADDING_LEFT,   into_spaces);
            },

            SpaceSet::PaddingBottom(value) => value.add(PADDING_BOTTOM, into_spaces),
            SpaceSet::PaddingLeft(value)   => value.add(PADDING_LEFT,   into_spaces),
            SpaceSet::PaddingRight(value)  => value.add(PADDING_RIGHT,  into_spaces),
            SpaceSet::PaddingTop(value)    => value.add(PADDING_TOP,    into_spaces),
        }
    }
}

impl Spaces {
    pub fn new() -> Self {
        Spaces(Vec::new())
    }

    pub fn add(&mut self, spaces: &[SpaceSet]) -> &Self {
        for i in 0..spaces.len() {
            spaces[i].add(self);
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
