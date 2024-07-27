mod container;
pub use container::Container;

mod item;
pub use item::Item;

use crate::prelude::*;

use std::fmt;

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum Direction {
    #[default]
    Default,
    Row(BreakPoint),
    RowReverse(BreakPoint),
    Column(BreakPoint),
    ColumnReverse(BreakPoint),
}

#[rustfmt::skip]
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Default           => write!(f, "flex__row {}", BreakPoint::default()),
            Direction::Row(bp)           => write!(f, "flex__row {bp}"),
            Direction::RowReverse(bp)    => write!(f, "flex__row flex__reverse {bp}"),
            Direction::Column(bp)        => write!(f, "flex__col {bp}"),
            Direction::ColumnReverse(bp) => write!(f, "flex__col flex__reverse {bp}"),
        }
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum Wrap {
    #[default]
    Default,
    NoWrap,
    Wrap(ContentAlign),
    WrapReverse(ContentAlign),
}

#[rustfmt::skip]
impl fmt::Display for Wrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Wrap::Default        => write!(f, ""),
            Wrap::NoWrap         => write!(f, "flex__nowrap"),
            Wrap::Wrap(a)        => write!(f, "flex__wrap {a}"),
            Wrap::WrapReverse(a) => write!(f, "flex__wrap-reverse {a}"),
        }
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum ContentAlign {
    #[default]
    Default,
    Start,
    End,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

#[rustfmt::skip]
impl fmt::Display for ContentAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentAlign::Default      => write!(f, ""),
            ContentAlign::Start        => write!(f, "flex__align-start"),
            ContentAlign::End          => write!(f, "flex__align-end"),
            ContentAlign::Center       => write!(f, "flex__align-center"),
            ContentAlign::Stretch      => write!(f, "flex__align-stretch"),
            ContentAlign::SpaceBetween => write!(f, "flex__align-space-between"),
            ContentAlign::SpaceAround  => write!(f, "flex__align-space-around"),
        }
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum Justify {
    #[default]
    Default,
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[rustfmt::skip]
impl fmt::Display for Justify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Justify::Default      => write!(f, ""),
            Justify::Start        => write!(f, "flex__justify-start"),
            Justify::End          => write!(f, "flex__justify-end"),
            Justify::Center       => write!(f, "flex__justify-center"),
            Justify::SpaceBetween => write!(f, "flex__justify-space-between"),
            Justify::SpaceAround  => write!(f, "flex__justify-space-around"),
            Justify::SpaceEvenly  => write!(f, "flex__justify-space-evenly"),
        }
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum Align {
    #[default]
    Default,
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

#[rustfmt::skip]
impl fmt::Display for Align {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Align::Default  => write!(f, ""),
            Align::Start    => write!(f, "flex__start"),
            Align::End      => write!(f, "flex__end"),
            Align::Center   => write!(f, "flex__center"),
            Align::Stretch  => write!(f, "flex__stretch"),
            Align::Baseline => write!(f, "flex__baseline"),
        }
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum Gap {
    #[default]
    Default,
    Row(unit::Value),
    Column(unit::Value),
    Distinct(unit::Value, unit::Value),
    Both(unit::Value),
}

#[rustfmt::skip]
impl fmt::Display for Gap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gap::Default        => write!(f, ""),
            Gap::Row(r)         => write!(f, "row-gap: {r};"),
            Gap::Column(c)      => write!(f, "column-gap: {c};"),
            Gap::Distinct(r, c) => write!(f, "gap: {r} {c};"),
            Gap::Both(v)        => write!(f, "gap: {v};"),
        }
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum Grow {
    #[default]
    Default,
    Is1,
    Is2,
    Is3,
    Is4,
    Is5,
    Is6,
    Is7,
    Is8,
    Is9,
}

impl fmt::Display for Grow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Grow::Default => write!(f, ""),
            Grow::Is1 => write!(f, "flex__grow-1"),
            Grow::Is2 => write!(f, "flex__grow-2"),
            Grow::Is3 => write!(f, "flex__grow-3"),
            Grow::Is4 => write!(f, "flex__grow-4"),
            Grow::Is5 => write!(f, "flex__grow-5"),
            Grow::Is6 => write!(f, "flex__grow-6"),
            Grow::Is7 => write!(f, "flex__grow-7"),
            Grow::Is8 => write!(f, "flex__grow-8"),
            Grow::Is9 => write!(f, "flex__grow-9"),
        }
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum Shrink {
    #[default]
    Default,
    Is1,
    Is2,
    Is3,
    Is4,
    Is5,
    Is6,
    Is7,
    Is8,
    Is9,
}

impl fmt::Display for Shrink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shrink::Default => write!(f, ""),
            Shrink::Is1 => write!(f, "flex__shrink-1"),
            Shrink::Is2 => write!(f, "flex__shrink-2"),
            Shrink::Is3 => write!(f, "flex__shrink-3"),
            Shrink::Is4 => write!(f, "flex__shrink-4"),
            Shrink::Is5 => write!(f, "flex__shrink-5"),
            Shrink::Is6 => write!(f, "flex__shrink-6"),
            Shrink::Is7 => write!(f, "flex__shrink-7"),
            Shrink::Is8 => write!(f, "flex__shrink-8"),
            Shrink::Is9 => write!(f, "flex__shrink-9"),
        }
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum Size {
    #[default]
    Default,
    Percent10,
    Percent20,
    Percent25,
    Percent33,
    Percent40,
    Percent50,
    Percent60,
    Percent66,
    Percent75,
    Percent80,
    Percent90,
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Size::Default => write!(f, ""),
            Size::Percent10 => write!(f, "flex__size-10"),
            Size::Percent20 => write!(f, "flex__size-20"),
            Size::Percent25 => write!(f, "flex__size-25"),
            Size::Percent33 => write!(f, "flex__size-33"),
            Size::Percent40 => write!(f, "flex__size-40"),
            Size::Percent50 => write!(f, "flex__size-50"),
            Size::Percent60 => write!(f, "flex__size-60"),
            Size::Percent66 => write!(f, "flex__size-66"),
            Size::Percent75 => write!(f, "flex__size-75"),
            Size::Percent80 => write!(f, "flex__size-80"),
            Size::Percent90 => write!(f, "flex__size-90"),
        }
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum Offset {
    #[default]
    Default,
    Offset10,
    Offset20,
    Offset25,
    Offset33,
    Offset40,
    Offset50,
    Offset60,
    Offset66,
    Offset75,
    Offset80,
    Offset90,
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Offset::Default => write!(f, ""),
            Offset::Offset10 => write!(f, "flex__offset-10"),
            Offset::Offset20 => write!(f, "flex__offset-20"),
            Offset::Offset25 => write!(f, "flex__offset-25"),
            Offset::Offset33 => write!(f, "flex__offset-33"),
            Offset::Offset40 => write!(f, "flex__offset-40"),
            Offset::Offset50 => write!(f, "flex__offset-50"),
            Offset::Offset60 => write!(f, "flex__offset-60"),
            Offset::Offset66 => write!(f, "flex__offset-66"),
            Offset::Offset75 => write!(f, "flex__offset-75"),
            Offset::Offset80 => write!(f, "flex__offset-80"),
            Offset::Offset90 => write!(f, "flex__offset-90"),
        }
    }
}
