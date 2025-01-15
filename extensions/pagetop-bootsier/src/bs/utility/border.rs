use pagetop::{prelude::*, strict_string};

use crate::bs::{BorderColor, BorderOpacity};

use std::fmt;

#[derive(AutoDefault)]
pub enum BorderSize {
    #[default]
    Default,
    Zero,
    Width1,
    Width2,
    Width3,
    Width4,
    Width5,
}

#[rustfmt::skip]
impl fmt::Display for BorderSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BorderSize::Default => write!(f, ""),
            BorderSize::Zero    => write!(f, "0"),
            BorderSize::Width1  => write!(f, "1"),
            BorderSize::Width2  => write!(f, "2"),
            BorderSize::Width3  => write!(f, "3"),
            BorderSize::Width4  => write!(f, "4"),
            BorderSize::Width5  => write!(f, "5"),
        }
    }
}

#[rustfmt::skip]
#[derive(AutoDefault)]
pub struct Border {
    color  : BorderColor,
    opacity: BorderOpacity,
    size   : BorderSize,
    top    : BorderSize,
    end    : BorderSize,
    bottom : BorderSize,
    start  : BorderSize,
}

impl Border {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(size: BorderSize) -> Self {
        Self::default().with_size(size)
    }

    // Border BUILDER.

    pub fn with_color(mut self, color: BorderColor) -> Self {
        self.color = color;
        self
    }

    pub fn with_opacity(mut self, opacity: BorderOpacity) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn with_size(mut self, size: BorderSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_top(mut self, size: BorderSize) -> Self {
        self.top = size;
        self
    }

    pub fn with_end(mut self, size: BorderSize) -> Self {
        self.end = size;
        self
    }

    pub fn with_bottom(mut self, size: BorderSize) -> Self {
        self.bottom = size;
        self
    }

    pub fn with_start(mut self, size: BorderSize) -> Self {
        self.start = size;
        self
    }
}

#[rustfmt::skip]
impl fmt::Display for Border {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", strict_string!([
            "border",
            &self.color.to_string(),
            &self.opacity.to_string(),
        ]; " ").unwrap_or_default())
    }
}
