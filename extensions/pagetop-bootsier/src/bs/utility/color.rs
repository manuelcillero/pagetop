use pagetop::prelude::*;

use std::fmt;

#[derive(AutoDefault)]
pub enum Color {
    #[default]
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Danger,
    Light,
    Dark,
}

#[rustfmt::skip]
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Primary   => write!(f, "primary"),
            Color::Secondary => write!(f, "secondary"),
            Color::Success   => write!(f, "success"),
            Color::Info      => write!(f, "info"),
            Color::Warning   => write!(f, "warning"),
            Color::Danger    => write!(f, "danger"),
            Color::Light     => write!(f, "light"),
            Color::Dark      => write!(f, "dark"),
        }
    }
}

#[derive(AutoDefault)]
pub enum BgColor {
    #[default]
    Default,
    Body,
    BodySecondary,
    BodyTertiary,
    Theme(Color),
    Subtle(Color),
    Black,
    White,
    Transparent,
}

#[rustfmt::skip]
impl fmt::Display for BgColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BgColor::Default       => write!(f, ""),
            BgColor::Body          => write!(f, "bg-body"),
            BgColor::BodySecondary => write!(f, "bg-body-secondary"),
            BgColor::BodyTertiary  => write!(f, "bg-body-tertiary"),
            BgColor::Theme(c)      => write!(f, "bg-{}", c),
            BgColor::Subtle(c)     => write!(f, "bg-{}-subtle", c),
            BgColor::Black         => write!(f, "bg-black"),
            BgColor::White         => write!(f, "bg-white"),
            BgColor::Transparent   => write!(f, "bg-transparent"),
        }
    }
}

#[derive(AutoDefault)]
pub enum BorderColor {
    #[default]
    Default,
    Theme(Color),
    Subtle(Color),
    Black,
    White,
}

#[rustfmt::skip]
impl fmt::Display for BorderColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BorderColor::Default   => write!(f, ""),
            BorderColor::Theme(c)  => write!(f, "border-{}", c),
            BorderColor::Subtle(c) => write!(f, "border-{}-subtle", c),
            BorderColor::Black     => write!(f, "border-black"),
            BorderColor::White     => write!(f, "border-white"),
        }
    }
}

#[derive(AutoDefault)]
pub enum TextColor {
    #[default]
    Default,
    Body,
    BodyEmphasis,
    BodySecondary,
    BodyTertiary,
    Theme(Color),
    Emphasis(Color),
    Background(Color),
    Black,
    White,
}

#[rustfmt::skip]
impl fmt::Display for TextColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextColor::Default       => write!(f, ""),
            TextColor::Body          => write!(f, "text-body"),
            TextColor::BodyEmphasis  => write!(f, "text-body-emphasis"),
            TextColor::BodySecondary => write!(f, "text-body-secondary"),
            TextColor::BodyTertiary  => write!(f, "text-body-tertiary"),
            TextColor::Theme(c)      => write!(f, "text-{}", c),
            TextColor::Emphasis(c)   => write!(f, "text-{}-emphasis", c),
            TextColor::Background(c) => write!(f, "text-bg-{}", c),
            TextColor::Black         => write!(f, "text-black"),
            TextColor::White         => write!(f, "text-white"),
        }
    }
}
