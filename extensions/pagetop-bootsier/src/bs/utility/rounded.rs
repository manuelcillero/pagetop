use pagetop::prelude::*;

use crate::bs::{Color, Opacity};

#[derive(AutoDefault)]
pub enum BorderSize {
    #[default]
    None,
    Width1,
    Width2,
    Width3,
    Width4,
    Width5,
    Free(unit::Value),
}

#[derive(AutoDefault)]
pub enum BorderRadius {
    #[default]
    None,
    Rounded1,
    Rounded2,
    Rounded3,
    Rounded4,
    Rounded5,
    Circle,
    Pill,
    Free(f32),
}

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct BorderProperty {
    color  : Color,
    opacity: Opacity,
    size   : BorderSize,
    radius : BorderRadius,
}

impl BorderProperty {
    pub fn new() -> Self {
        BorderProperty::default()
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_opacity(mut self, opacity: Opacity) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn with_size(mut self, size: BorderSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_radius(mut self, radius: BorderRadius) -> Self {
        self.radius = radius;
        self
    }
}

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Border {
    all   : Option<BorderProperty>,
    top   : Option<BorderProperty>,
    end   : Option<BorderProperty>,
    bottom: Option<BorderProperty>,
    start : Option<BorderProperty>,
}

impl Border {
    pub fn new() -> Self {
        Self::default()
    }

    // Border BUILDER.

    pub fn with_all(mut self, border: BorderProperty) -> Self {
        self.all = Some(border);
        self
    }

    pub fn with_top(mut self, border: BorderProperty) -> Self {
        self.top = Some(border);
        self
    }

    pub fn with_end(mut self, border: BorderProperty) -> Self {
        self.end = Some(border);
        self
    }

    pub fn with_bottom(mut self, border: BorderProperty) -> Self {
        self.bottom = Some(border);
        self
    }

    pub fn with_start(mut self, border: BorderProperty) -> Self {
        self.start = Some(border);
        self
    }

    pub fn with_none(mut self) -> Self {
        self.all = None;
        self.top = None;
        self.end = None;
        self.bottom = None;
        self.start = None;
        self
    }
}
