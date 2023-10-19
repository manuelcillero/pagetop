mod container;
pub use container::{Container, COMPONENT_BASE_FLEX_CONTAINER};
mod item;
pub use item::{Item, COMPONENT_BASE_FLEX_ITEM};

use crate::prelude::*;

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
impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Default => {
                concat_string!(
                    "pt-flex__container pt-flex__row ",
                    BreakPoint::default().to_string()
                )
            }
            Direction::Row(breakpoint) => {
                concat_string!(
                    "pt-flex__container pt-flex__row ",
                    breakpoint.to_string()
                )
            }
            Direction::RowReverse(breakpoint) => {
                concat_string!(
                    "pt-flex__container pt-flex__row pt-flex__reverse ",
                    breakpoint.to_string()
                )
            }
            Direction::Column(breakpoint) => {
                concat_string!(
                    "pt-flex__container pt-flex__col ",
                    breakpoint.to_string()
                )
            }
            Direction::ColumnReverse(breakpoint) => {
                concat_string!(
                    "pt-flex__container pt-flex__col pt-flex__reverse ",
                    breakpoint.to_string()
                )
            }
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
impl ToString for WrapAlign {
    fn to_string(&self) -> String {
        match self {
            WrapAlign::Default        => "".to_string(),
            WrapAlign::NoWrap         => "flex-nowrap".to_string(),
            WrapAlign::Wrap(a)        => concat_string!("pt-flex__wrap ", a.to_string()),
            WrapAlign::WrapReverse(a) => concat_string!("pt-flex__wrap-reverse ", a.to_string()),
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
impl ToString for ContentAlign {
    fn to_string(&self) -> String {
        match self {
            ContentAlign::Default      => "".to_string(),
            ContentAlign::Start        => "pt-flex__align-start".to_string(),
            ContentAlign::End          => "pt-flex__align-end".to_string(),
            ContentAlign::Center       => "pt-flex__align-center".to_string(),
            ContentAlign::Stretch      => "pt-flex__align-stretch".to_string(),
            ContentAlign::SpaceBetween => "pt-flex__align-space-between".to_string(),
            ContentAlign::SpaceAround  => "pt-flex__align-space-around".to_string(),
        }
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
impl ToString for ContentJustify {
    fn to_string(&self) -> String {
        match self {
            ContentJustify::Default      => "".to_string(),
            ContentJustify::Start        => "pt-flex__justify-start".to_string(),
            ContentJustify::End          => "pt-flex__justify-end".to_string(),
            ContentJustify::Center       => "pt-flex__justify-center".to_string(),
            ContentJustify::SpaceBetween => "pt-flex__justify-space-between".to_string(),
            ContentJustify::SpaceAround  => "pt-flex__justify-space-around".to_string(),
            ContentJustify::SpaceEvenly  => "pt-flex__justify-space-evenly".to_string(),
        }
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
impl ToString for ItemAlign {
    fn to_string(&self) -> String {
        match self {
            ItemAlign::Default  => "".to_string(),
            ItemAlign::Top      => "pt-flex__item-top".to_string(),
            ItemAlign::Bottom   => "pt-flex__item-bottom".to_string(),
            ItemAlign::Middle   => "pt-flex__item-middle".to_string(),
            ItemAlign::Stretch  => "pt-flex__item-stretch".to_string(),
            ItemAlign::Baseline => "pt-flex__item-baseline".to_string(),
        }
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
impl ToString for Gap {
    fn to_string(&self) -> String {
        match self {
            Gap::Default        => "".to_string(),
            Gap::Row(r)         => concat_string!("row-gap: ", r.to_string(), ";"),
            Gap::Column(c)      => concat_string!("column-gap: ", c.to_string(), ";"),
            Gap::Distinct(r, c) => concat_string!("gap: ", r.to_string(), " ", c.to_string(), ";"),
            Gap::Both(v)        => concat_string!("gap: ", v.to_string(), ";"),
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
impl ToString for ItemGrow {
    fn to_string(&self) -> String {
        match self {
            ItemGrow::Default => "".to_string(),
            ItemGrow::Is1 => "pt-flex__grow-1".to_string(),
            ItemGrow::Is2 => "pt-flex__grow-2".to_string(),
            ItemGrow::Is3 => "pt-flex__grow-3".to_string(),
            ItemGrow::Is4 => "pt-flex__grow-4".to_string(),
            ItemGrow::Is5 => "pt-flex__grow-5".to_string(),
            ItemGrow::Is6 => "pt-flex__grow-6".to_string(),
            ItemGrow::Is7 => "pt-flex__grow-7".to_string(),
            ItemGrow::Is8 => "pt-flex__grow-8".to_string(),
            ItemGrow::Is9 => "pt-flex__grow-9".to_string(),
        }
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
impl ToString for ItemShrink {
    fn to_string(&self) -> String {
        match self {
            ItemShrink::Default => "".to_string(),
            ItemShrink::Is1 => "pt-flex__shrink-1".to_string(),
            ItemShrink::Is2 => "pt-flex__shrink-2".to_string(),
            ItemShrink::Is3 => "pt-flex__shrink-3".to_string(),
            ItemShrink::Is4 => "pt-flex__shrink-4".to_string(),
            ItemShrink::Is5 => "pt-flex__shrink-5".to_string(),
            ItemShrink::Is6 => "pt-flex__shrink-6".to_string(),
            ItemShrink::Is7 => "pt-flex__shrink-7".to_string(),
            ItemShrink::Is8 => "pt-flex__shrink-8".to_string(),
            ItemShrink::Is9 => "pt-flex__shrink-9".to_string(),
        }
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
impl ToString for ItemSize {
    fn to_string(&self) -> String {
        match self {
            ItemSize::Default => "".to_string(),
            ItemSize::Percent10 => "pt-flex__width-10".to_string(),
            ItemSize::Percent20 => "pt-flex__width-20".to_string(),
            ItemSize::Percent25 => "pt-flex__width-25".to_string(),
            ItemSize::Percent33 => "pt-flex__width-33".to_string(),
            ItemSize::Percent40 => "pt-flex__width-40".to_string(),
            ItemSize::Percent50 => "pt-flex__width-50".to_string(),
            ItemSize::Percent60 => "pt-flex__width-60".to_string(),
            ItemSize::Percent66 => "pt-flex__width-66".to_string(),
            ItemSize::Percent75 => "pt-flex__width-75".to_string(),
            ItemSize::Percent80 => "pt-flex__width-80".to_string(),
            ItemSize::Percent90 => "pt-flex__width-90".to_string(),
        }
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
impl ToString for ItemOffset {
    fn to_string(&self) -> String {
        match self {
            ItemOffset::Default => "".to_string(),
            ItemOffset::Offset10 => "pt-flex__offset-10".to_string(),
            ItemOffset::Offset20 => "pt-flex__offset-20".to_string(),
            ItemOffset::Offset25 => "pt-flex__offset-25".to_string(),
            ItemOffset::Offset33 => "pt-flex__offset-33".to_string(),
            ItemOffset::Offset40 => "pt-flex__offset-40".to_string(),
            ItemOffset::Offset50 => "pt-flex__offset-50".to_string(),
            ItemOffset::Offset60 => "pt-flex__offset-60".to_string(),
            ItemOffset::Offset66 => "pt-flex__offset-66".to_string(),
            ItemOffset::Offset75 => "pt-flex__offset-75".to_string(),
            ItemOffset::Offset80 => "pt-flex__offset-80".to_string(),
            ItemOffset::Offset90 => "pt-flex__offset-90".to_string(),
        }
    }
}
