mod container;
pub use container::{Container, COMPONENT_BASE_FLEX_CONTAINER};
mod item;
pub use item::{Item, COMPONENT_BASE_FLEX_ITEM};

use crate::prelude::*;

use std::fmt;

// *************************************************************************************************

#[derive(Default)]
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
            Direction::Default => write!(
                f, "pt-flex__container pt-flex__row {}", BreakPoint::default()
            ),
            Direction::Row(breakpoint) => write!(
                f, "pt-flex__container pt-flex__row {breakpoint}"
            ),
            Direction::RowReverse(breakpoint) => write!(
                f, "pt-flex__container pt-flex__row pt-flex__reverse {breakpoint}"
            ),
            Direction::Column(breakpoint) => write!(
                f, "pt-flex__container pt-flex__col {breakpoint}"
            ),
            Direction::ColumnReverse(breakpoint) => write!(
                f, "pt-flex__container pt-flex__col pt-flex__reverse {breakpoint}"
            ),
        }
    }
}

// *************************************************************************************************

#[derive(Default)]
pub enum WrapAlign {
    #[default]
    Default,
    NoWrap,
    Wrap(ContentAlign),
    WrapReverse(ContentAlign),
}

#[rustfmt::skip]
impl fmt::Display for WrapAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WrapAlign::Default        => write!(f, ""),
            WrapAlign::NoWrap         => write!(f, "flex-nowrap"),
            WrapAlign::Wrap(a)        => write!(f, "pt-flex__wrap {a}"),
            WrapAlign::WrapReverse(a) => write!(f, "pt-flex__wrap-reverse {a}"),
        }
    }
}

// *************************************************************************************************

#[derive(Default)]
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
        let content_align = match self {
            ContentAlign::Default      => "",
            ContentAlign::Start        => "pt-flex__align-start",
            ContentAlign::End          => "pt-flex__align-end",
            ContentAlign::Center       => "pt-flex__align-center",
            ContentAlign::Stretch      => "pt-flex__align-stretch",
            ContentAlign::SpaceBetween => "pt-flex__align-space-between",
            ContentAlign::SpaceAround  => "pt-flex__align-space-around",
        };
        write!(f, "{content_align}")
    }
}

// *************************************************************************************************

#[derive(Default)]
pub enum ContentJustify {
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
impl fmt::Display for ContentJustify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content_justify = match self {
            ContentJustify::Default      => "",
            ContentJustify::Start        => "pt-flex__justify-start",
            ContentJustify::End          => "pt-flex__justify-end",
            ContentJustify::Center       => "pt-flex__justify-center",
            ContentJustify::SpaceBetween => "pt-flex__justify-space-between",
            ContentJustify::SpaceAround  => "pt-flex__justify-space-around",
            ContentJustify::SpaceEvenly  => "pt-flex__justify-space-evenly",
        };
        write!(f, "{content_justify}")
    }
}

// *************************************************************************************************

#[derive(Default)]
pub enum ItemAlign {
    #[default]
    Default,
    Top,
    Bottom,
    Middle,
    Stretch,
    Baseline,
}

#[rustfmt::skip]
impl fmt::Display for ItemAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let item_align = match self {
            ItemAlign::Default  => "",
            ItemAlign::Top      => "pt-flex__item-top",
            ItemAlign::Bottom   => "pt-flex__item-bottom",
            ItemAlign::Middle   => "pt-flex__item-middle",
            ItemAlign::Stretch  => "pt-flex__item-stretch",
            ItemAlign::Baseline => "pt-flex__item-baseline",
        };
        write!(f, "{item_align}")
    }
}

// *************************************************************************************************

#[derive(Default)]
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

#[derive(Default)]
pub enum ItemGrow {
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

#[rustfmt::skip]
impl fmt::Display for ItemGrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let item_grow = match self {
            ItemGrow::Default => "",
            ItemGrow::Is1 => "pt-flex__grow-1",
            ItemGrow::Is2 => "pt-flex__grow-2",
            ItemGrow::Is3 => "pt-flex__grow-3",
            ItemGrow::Is4 => "pt-flex__grow-4",
            ItemGrow::Is5 => "pt-flex__grow-5",
            ItemGrow::Is6 => "pt-flex__grow-6",
            ItemGrow::Is7 => "pt-flex__grow-7",
            ItemGrow::Is8 => "pt-flex__grow-8",
            ItemGrow::Is9 => "pt-flex__grow-9",
        };
        write!(f, "{item_grow}")
    }
}

// *************************************************************************************************

#[derive(Default)]
pub enum ItemShrink {
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

#[rustfmt::skip]
impl fmt::Display for ItemShrink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let item_shrink = match self {
            ItemShrink::Default => "",
            ItemShrink::Is1 => "pt-flex__shrink-1",
            ItemShrink::Is2 => "pt-flex__shrink-2",
            ItemShrink::Is3 => "pt-flex__shrink-3",
            ItemShrink::Is4 => "pt-flex__shrink-4",
            ItemShrink::Is5 => "pt-flex__shrink-5",
            ItemShrink::Is6 => "pt-flex__shrink-6",
            ItemShrink::Is7 => "pt-flex__shrink-7",
            ItemShrink::Is8 => "pt-flex__shrink-8",
            ItemShrink::Is9 => "pt-flex__shrink-9",
        };
        write!(f, "{item_shrink}")
    }
}

// *************************************************************************************************

#[derive(Default)]
pub enum ItemSize {
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

#[rustfmt::skip]
impl fmt::Display for ItemSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let item_size = match self {
            ItemSize::Default   => "",
            ItemSize::Percent10 => "pt-flex__width-10",
            ItemSize::Percent20 => "pt-flex__width-20",
            ItemSize::Percent25 => "pt-flex__width-25",
            ItemSize::Percent33 => "pt-flex__width-33",
            ItemSize::Percent40 => "pt-flex__width-40",
            ItemSize::Percent50 => "pt-flex__width-50",
            ItemSize::Percent60 => "pt-flex__width-60",
            ItemSize::Percent66 => "pt-flex__width-66",
            ItemSize::Percent75 => "pt-flex__width-75",
            ItemSize::Percent80 => "pt-flex__width-80",
            ItemSize::Percent90 => "pt-flex__width-90",
        };
        write!(f, "{item_size}")
    }
}

// *************************************************************************************************

#[derive(Default)]
pub enum ItemOffset {
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

#[rustfmt::skip]
impl fmt::Display for ItemOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let item_offset = match self {
            ItemOffset::Default  => "",
            ItemOffset::Offset10 => "pt-flex__offset-10",
            ItemOffset::Offset20 => "pt-flex__offset-20",
            ItemOffset::Offset25 => "pt-flex__offset-25",
            ItemOffset::Offset33 => "pt-flex__offset-33",
            ItemOffset::Offset40 => "pt-flex__offset-40",
            ItemOffset::Offset50 => "pt-flex__offset-50",
            ItemOffset::Offset60 => "pt-flex__offset-60",
            ItemOffset::Offset66 => "pt-flex__offset-66",
            ItemOffset::Offset75 => "pt-flex__offset-75",
            ItemOffset::Offset80 => "pt-flex__offset-80",
            ItemOffset::Offset90 => "pt-flex__offset-90",
        };
        write!(f, "{item_offset}")
    }
}
