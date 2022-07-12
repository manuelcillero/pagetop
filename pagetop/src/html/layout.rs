use crate::concat_string;

use super::unit::UnitValue;

const RADIUS_BOTTOM_LEFT:  &str = "border-bottom-left-radius";
const RADIUS_BOTTOM_RIGHT: &str = "border-bottom-right-radius";
const RADIUS_TOP_LEFT:     &str = "border-top-left-radius";
const RADIUS_TOP_RIGHT:    &str = "border-top-right-radius";

const MARGIN_BOTTOM:       &str = "margin-bottom";
const MARGIN_LEFT:         &str = "margin-left";
const MARGIN_RIGHT:        &str = "margin-right";
const MARGIN_TOP:          &str = "margin-top";

const PADDING_BOTTOM:      &str = "padding-bottom";
const PADDING_LEFT:        &str = "padding-left";
const PADDING_RIGHT:       &str = "padding-right";
const PADDING_TOP:         &str = "padding-top";

pub enum LayoutSet {
    Margin(UnitValue, UnitValue, UnitValue, UnitValue),
    MarginAll(UnitValue),
    MarginSide(UnitValue, UnitValue),
    MarginBottom(UnitValue),
    MarginLeft(UnitValue),
    MarginRight(UnitValue),
    MarginTop(UnitValue),

    Padding(UnitValue, UnitValue, UnitValue, UnitValue),
    PaddingAll(UnitValue),
    PaddingSide(UnitValue, UnitValue),
    PaddingBottom(UnitValue),
    PaddingLeft(UnitValue),
    PaddingRight(UnitValue),
    PaddingTop(UnitValue),

    Radius(UnitValue, UnitValue, UnitValue, UnitValue),
    RadiusAll(UnitValue),
    RadiusBottomLeft(UnitValue),
    RadiusBottomRight(UnitValue),
    RadiusTopLeft(UnitValue),
    RadiusTopRight(UnitValue),
}

impl LayoutSet {
    fn set(&self, into_spaces: &mut InlineLayout) {
        match self {

            // MARGIN LAYOUT.
            LayoutSet::Margin(top, right, bottom, left) => {
                self.add(MARGIN_TOP,    top,    into_spaces);
                self.add(MARGIN_RIGHT,  right,  into_spaces);
                self.add(MARGIN_BOTTOM, bottom, into_spaces);
                self.add(MARGIN_LEFT,   left,   into_spaces);
            },
            LayoutSet::MarginAll(val) => {
                self.add(MARGIN_TOP,    val, into_spaces);
                self.add(MARGIN_RIGHT,  val, into_spaces);
                self.add(MARGIN_BOTTOM, val, into_spaces);
                self.add(MARGIN_LEFT,   val, into_spaces);
            },
            LayoutSet::MarginSide(top_bottom, right_left) => {
                self.add(MARGIN_TOP,    top_bottom, into_spaces);
                self.add(MARGIN_RIGHT,  right_left, into_spaces);
                self.add(MARGIN_BOTTOM, top_bottom, into_spaces);
                self.add(MARGIN_LEFT,   right_left, into_spaces);
            },
            LayoutSet::MarginTop(val)    => self.add(MARGIN_TOP,    val, into_spaces),
            LayoutSet::MarginRight(val)  => self.add(MARGIN_RIGHT,  val, into_spaces),
            LayoutSet::MarginBottom(val) => self.add(MARGIN_BOTTOM, val, into_spaces),
            LayoutSet::MarginLeft(val)   => self.add(MARGIN_LEFT,   val, into_spaces),

            // PADDING LAYOUT.
            LayoutSet::Padding(top, right, bottom, left) => {
                self.add(PADDING_TOP,    top, into_spaces);
                self.add(PADDING_RIGHT,  right, into_spaces);
                self.add(PADDING_BOTTOM, bottom, into_spaces);
                self.add(PADDING_LEFT,   left, into_spaces);
            },
            LayoutSet::PaddingAll(val) => {
                self.add(PADDING_TOP,    val, into_spaces);
                self.add(PADDING_RIGHT,  val, into_spaces);
                self.add(PADDING_BOTTOM, val, into_spaces);
                self.add(PADDING_LEFT,   val, into_spaces);
            },
            LayoutSet::PaddingSide(top_bottom, right_left) => {
                self.add(PADDING_TOP,    top_bottom, into_spaces);
                self.add(PADDING_RIGHT,  right_left, into_spaces);
                self.add(PADDING_BOTTOM, top_bottom, into_spaces);
                self.add(PADDING_LEFT,   right_left, into_spaces);
            },
            LayoutSet::PaddingTop(val)    => self.add(PADDING_TOP,    val, into_spaces),
            LayoutSet::PaddingRight(val)  => self.add(PADDING_RIGHT,  val, into_spaces),
            LayoutSet::PaddingBottom(val) => self.add(PADDING_BOTTOM, val, into_spaces),
            LayoutSet::PaddingLeft(val)   => self.add(PADDING_LEFT,   val, into_spaces),

            // BORDER RADIUS LAYOUT.
            LayoutSet::Radius(top_left, top_right, bottom_right, bottom_left) => {
                self.add(RADIUS_TOP_LEFT,     top_left,     into_spaces);
                self.add(RADIUS_TOP_RIGHT,    top_right,    into_spaces);
                self.add(RADIUS_BOTTOM_RIGHT, bottom_right, into_spaces);
                self.add(RADIUS_BOTTOM_LEFT,  bottom_left,  into_spaces);
            },
            LayoutSet::RadiusAll(val) => {
                self.add(RADIUS_TOP_LEFT,     val, into_spaces);
                self.add(RADIUS_TOP_RIGHT,    val, into_spaces);
                self.add(RADIUS_BOTTOM_RIGHT, val, into_spaces);
                self.add(RADIUS_BOTTOM_LEFT,  val, into_spaces);
            },
            LayoutSet::RadiusTopLeft(val)     => self.add(RADIUS_TOP_LEFT,     val, into_spaces),
            LayoutSet::RadiusTopRight(val)    => self.add(RADIUS_TOP_RIGHT,    val, into_spaces),
            LayoutSet::RadiusBottomRight(val) => self.add(RADIUS_BOTTOM_RIGHT, val, into_spaces),
            LayoutSet::RadiusBottomLeft(val)  => self.add(RADIUS_BOTTOM_LEFT,  val, into_spaces),
        }
    }

    fn add(&self, property: &str, value: &UnitValue, into_spaces: &mut InlineLayout) {
        let val = value.to_string();
        let style = InlineProperty {
            property: property.to_owned(),
            inline  : concat_string!(property, ":", val, ";"),
        };
        match into_spaces.0.iter().position(|s| s.property.eq(&style.property)) {
            Some(pos) => {
                into_spaces.0.remove(pos);
                if !val.is_empty() {
                    into_spaces.0.insert(pos, style);
                }
            },
            _ => if !val.is_empty() {
                into_spaces.0.push(style)
            }
        }
    }
}

struct InlineProperty {
    property: String,
    inline  : String,
}

pub struct InlineLayout(Vec<InlineProperty>);

impl InlineLayout {
    pub fn new() -> Self {
        InlineLayout(Vec::new())
    }

    pub fn set(&mut self, layout: &[LayoutSet]) -> &Self {
        for i in 0..layout.len() {
            layout[i].set(self);
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
