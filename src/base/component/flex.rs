mod container;
pub use container::Container;
mod item;
pub use item::Item;

use crate::prelude::*;

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
impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Default => concat_string!(
                "flex__row ", BreakPoint::default().to_string()
            ),
            Direction::Row(breakpoint) => concat_string!(
                "flex__row ", breakpoint.to_string()
            ),
            Direction::RowReverse(breakpoint) => concat_string!(
                "flex__row flex__reverse ", breakpoint.to_string()
            ),
            Direction::Column(breakpoint) => concat_string!(
                "flex__col ", breakpoint.to_string()
            ),
            Direction::ColumnReverse(breakpoint) => concat_string!(
                "flex__col flex__reverse ", breakpoint.to_string()
            ),
        }
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
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
            WrapAlign::Default        => "".to_owned(),
            WrapAlign::NoWrap         => "flex-nowrap".to_owned(),
            WrapAlign::Wrap(a)        => concat_string!("flex__wrap ", a.to_string()),
            WrapAlign::WrapReverse(a) => concat_string!("flex__wrap-reverse ", a.to_string()),
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
impl ToString for ContentAlign {
    fn to_string(&self) -> String {
        String::from(match self {
            ContentAlign::Default      => "",
            ContentAlign::Start        => "flex__align-start",
            ContentAlign::End          => "flex__align-end",
            ContentAlign::Center       => "flex__align-center",
            ContentAlign::Stretch      => "flex__align-stretch",
            ContentAlign::SpaceBetween => "flex__align-space-between",
            ContentAlign::SpaceAround  => "flex__align-space-around",
        })
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
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
        String::from(match self {
            ContentJustify::Default      => "",
            ContentJustify::Start        => "flex__justify-start",
            ContentJustify::End          => "flex__justify-end",
            ContentJustify::Center       => "flex__justify-center",
            ContentJustify::SpaceBetween => "flex__justify-space-between",
            ContentJustify::SpaceAround  => "flex__justify-space-around",
            ContentJustify::SpaceEvenly  => "flex__justify-space-evenly",
        })
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum ItemAlign {
    #[default]
    Default,
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

#[rustfmt::skip]
impl ToString for ItemAlign {
    fn to_string(&self) -> String {
        String::from(match self {
            ItemAlign::Default  => "",
            ItemAlign::Start    => "flex__item-start",
            ItemAlign::End      => "flex__item-end",
            ItemAlign::Center   => "flex__item-center",
            ItemAlign::Stretch  => "flex__item-stretch",
            ItemAlign::Baseline => "flex__item-baseline",
        })
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
impl ToString for Gap {
    fn to_string(&self) -> String {
        match self {
            Gap::Default        => "".to_owned(),
            Gap::Row(r)         => concat_string!("row-gap: ", r.to_string(), ";"),
            Gap::Column(c)      => concat_string!("column-gap: ", c.to_string(), ";"),
            Gap::Distinct(r, c) => concat_string!("gap: ", r.to_string(), " ", c.to_string(), ";"),
            Gap::Both(v)        => concat_string!("gap: ", v.to_string(), ";"),
        }
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
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

impl ToString for ItemGrow {
    fn to_string(&self) -> String {
        String::from(match self {
            ItemGrow::Default => "",
            ItemGrow::Is1 => "flex__item-grow-1",
            ItemGrow::Is2 => "flex__item-grow-2",
            ItemGrow::Is3 => "flex__item-grow-3",
            ItemGrow::Is4 => "flex__item-grow-4",
            ItemGrow::Is5 => "flex__item-grow-5",
            ItemGrow::Is6 => "flex__item-grow-6",
            ItemGrow::Is7 => "flex__item-grow-7",
            ItemGrow::Is8 => "flex__item-grow-8",
            ItemGrow::Is9 => "flex__item-grow-9",
        })
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
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

impl ToString for ItemShrink {
    fn to_string(&self) -> String {
        String::from(match self {
            ItemShrink::Default => "",
            ItemShrink::Is1 => "flex__item-shrink-1",
            ItemShrink::Is2 => "flex__item-shrink-2",
            ItemShrink::Is3 => "flex__item-shrink-3",
            ItemShrink::Is4 => "flex__item-shrink-4",
            ItemShrink::Is5 => "flex__item-shrink-5",
            ItemShrink::Is6 => "flex__item-shrink-6",
            ItemShrink::Is7 => "flex__item-shrink-7",
            ItemShrink::Is8 => "flex__item-shrink-8",
            ItemShrink::Is9 => "flex__item-shrink-9",
        })
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
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

impl ToString for ItemSize {
    fn to_string(&self) -> String {
        String::from(match self {
            ItemSize::Default => "",
            ItemSize::Percent10 => "flex__item-size-10",
            ItemSize::Percent20 => "flex__item-size-20",
            ItemSize::Percent25 => "flex__item-size-25",
            ItemSize::Percent33 => "flex__item-size-33",
            ItemSize::Percent40 => "flex__item-size-40",
            ItemSize::Percent50 => "flex__item-size-50",
            ItemSize::Percent60 => "flex__item-size-60",
            ItemSize::Percent66 => "flex__item-size-66",
            ItemSize::Percent75 => "flex__item-size-75",
            ItemSize::Percent80 => "flex__item-size-80",
            ItemSize::Percent90 => "flex__item-size-90",
        })
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
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

impl ToString for ItemOffset {
    fn to_string(&self) -> String {
        String::from(match self {
            ItemOffset::Default => "",
            ItemOffset::Offset10 => "flex__item-offset-10",
            ItemOffset::Offset20 => "flex__item-offset-20",
            ItemOffset::Offset25 => "flex__item-offset-25",
            ItemOffset::Offset33 => "flex__item-offset-33",
            ItemOffset::Offset40 => "flex__item-offset-40",
            ItemOffset::Offset50 => "flex__item-offset-50",
            ItemOffset::Offset60 => "flex__item-offset-60",
            ItemOffset::Offset66 => "flex__item-offset-66",
            ItemOffset::Offset75 => "flex__item-offset-75",
            ItemOffset::Offset80 => "flex__item-offset-80",
            ItemOffset::Offset90 => "flex__item-offset-90",
        })
    }
}
