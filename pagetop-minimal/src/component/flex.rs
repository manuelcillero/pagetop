mod container;
pub use container::{Container, COMPONENT_FLEX_CONTAINER};
mod item;
pub use item::{Item, COMPONENT_FLEX_ITEM};

use pagetop::concat_string;
use pagetop::html::unit;

use crate::BreakPoint;

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
                concat_string!("flex-container flex-row ", BreakPoint::default().to_string())
            }
            Direction::Row(breakpoint) => {
                concat_string!("flex-container flex-row ", breakpoint.to_string())
            }
            Direction::RowReverse(breakpoint) => {
                concat_string!("flex-container flex-row flex-reverse ", breakpoint.to_string())
            }
            Direction::Column(breakpoint) => {
                concat_string!("flex-container flex-col ", breakpoint.to_string())
            }
            Direction::ColumnReverse(breakpoint) => {
                concat_string!("flex-container flex-col flex-reverse ", breakpoint.to_string())
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
            WrapAlign::Wrap(a)        => concat_string!("flex-wrap ", a.to_string()),
            WrapAlign::WrapReverse(a) => concat_string!("flex-wrap-reverse ", a.to_string()),
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
            ContentAlign::Start        => "flex-align-start".to_string(),
            ContentAlign::End          => "flex-align-end".to_string(),
            ContentAlign::Center       => "flex-align-center".to_string(),
            ContentAlign::Stretch      => "flex-align-stretch".to_string(),
            ContentAlign::SpaceBetween => "flex-align-space-between".to_string(),
            ContentAlign::SpaceAround  => "flex-align-space-around".to_string(),
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
            ContentJustify::Start        => "flex-justify-start".to_string(),
            ContentJustify::End          => "flex-justify-end".to_string(),
            ContentJustify::Center       => "flex-justify-center".to_string(),
            ContentJustify::SpaceBetween => "flex-justify-space-between".to_string(),
            ContentJustify::SpaceAround  => "flex-justify-space-around".to_string(),
            ContentJustify::SpaceEvenly  => "flex-justify-space-evenly".to_string(),
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
            ItemAlign::Top      => "flex-item-top".to_string(),
            ItemAlign::Bottom   => "flex-item-bottom".to_string(),
            ItemAlign::Middle   => "flex-item-middle".to_string(),
            ItemAlign::Stretch  => "flex-item-stretch".to_string(),
            ItemAlign::Baseline => "flex-item-baseline".to_string(),
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
            ItemGrow::Is1 => "flex-grow-1".to_string(),
            ItemGrow::Is2 => "flex-grow-2".to_string(),
            ItemGrow::Is3 => "flex-grow-3".to_string(),
            ItemGrow::Is4 => "flex-grow-4".to_string(),
            ItemGrow::Is5 => "flex-grow-5".to_string(),
            ItemGrow::Is6 => "flex-grow-6".to_string(),
            ItemGrow::Is7 => "flex-grow-7".to_string(),
            ItemGrow::Is8 => "flex-grow-8".to_string(),
            ItemGrow::Is9 => "flex-grow-9".to_string(),
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
            ItemShrink::Is1 => "flex-shrink-1".to_string(),
            ItemShrink::Is2 => "flex-shrink-2".to_string(),
            ItemShrink::Is3 => "flex-shrink-3".to_string(),
            ItemShrink::Is4 => "flex-shrink-4".to_string(),
            ItemShrink::Is5 => "flex-shrink-5".to_string(),
            ItemShrink::Is6 => "flex-shrink-6".to_string(),
            ItemShrink::Is7 => "flex-shrink-7".to_string(),
            ItemShrink::Is8 => "flex-shrink-8".to_string(),
            ItemShrink::Is9 => "flex-shrink-9".to_string(),
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
            ItemSize::Percent10 => "flex-width-10".to_string(),
            ItemSize::Percent20 => "flex-width-20".to_string(),
            ItemSize::Percent25 => "flex-width-25".to_string(),
            ItemSize::Percent33 => "flex-width-33".to_string(),
            ItemSize::Percent40 => "flex-width-40".to_string(),
            ItemSize::Percent50 => "flex-width-50".to_string(),
            ItemSize::Percent60 => "flex-width-60".to_string(),
            ItemSize::Percent66 => "flex-width-66".to_string(),
            ItemSize::Percent75 => "flex-width-75".to_string(),
            ItemSize::Percent80 => "flex-width-80".to_string(),
            ItemSize::Percent90 => "flex-width-90".to_string(),
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
            ItemOffset::Offset10 => "flex-offset-10".to_string(),
            ItemOffset::Offset20 => "flex-offset-20".to_string(),
            ItemOffset::Offset25 => "flex-offset-25".to_string(),
            ItemOffset::Offset33 => "flex-offset-33".to_string(),
            ItemOffset::Offset40 => "flex-offset-40".to_string(),
            ItemOffset::Offset50 => "flex-offset-50".to_string(),
            ItemOffset::Offset60 => "flex-offset-60".to_string(),
            ItemOffset::Offset66 => "flex-offset-66".to_string(),
            ItemOffset::Offset75 => "flex-offset-75".to_string(),
            ItemOffset::Offset80 => "flex-offset-80".to_string(),
            ItemOffset::Offset90 => "flex-offset-90".to_string(),
        }
    }
}
