use pagetop::prelude::*;

use std::fmt;

#[rustfmt::skip]
#[derive(AutoDefault)]
pub enum Opacity {
    #[default]
    Opaque,            // 100%
    SemiOpaque,        // 75%
    Half,              // 50%
    SemiTransparent,   // 25%
    AlmostTransparent, // 10%
}

#[rustfmt::skip]
impl fmt::Display for Opacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opacity::Opaque            => write!(f, "opacity-100"),
            Opacity::SemiOpaque        => write!(f, "opacity-75"),
            Opacity::Half              => write!(f, "opacity-50"),
            Opacity::SemiTransparent   => write!(f, "opacity-25"),
            Opacity::AlmostTransparent => write!(f, "opacity-10"),
        }
    }
}

#[derive(AutoDefault)]
pub enum BgOpacity {
    #[default]
    Default,
    Theme(Opacity),
}

#[rustfmt::skip]
impl fmt::Display for BgOpacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BgOpacity::Default  => write!(f, ""),
            BgOpacity::Theme(o) => write!(f, "bg-{}", o),
        }
    }
}

#[derive(AutoDefault)]
pub enum BorderOpacity {
    #[default]
    Default,
    Theme(Opacity),
}

#[rustfmt::skip]
impl fmt::Display for BorderOpacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BorderOpacity::Default  => write!(f, ""),
            BorderOpacity::Theme(o) => write!(f, "border-{}", o),
        }
    }
}

#[derive(AutoDefault)]
pub enum TextOpacity {
    #[default]
    Default,
    Theme(Opacity),
}

#[rustfmt::skip]
impl fmt::Display for TextOpacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextOpacity::Default  => write!(f, ""),
            TextOpacity::Theme(o) => write!(f, "text-{}", o),
        }
    }
}
