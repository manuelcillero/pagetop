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
                "pt-flex__container pt-flex__row ", BreakPoint::default().to_string()
            ),
            Direction::Row(breakpoint) => concat_string!(
                "pt-flex__container pt-flex__row ", breakpoint.to_string()
            ),
            Direction::RowReverse(breakpoint) => concat_string!(
                "pt-flex__container pt-flex__row pt-flex__reverse ", breakpoint.to_string()
            ),
            Direction::Column(breakpoint) => concat_string!(
                "pt-flex__container pt-flex__col ", breakpoint.to_string()
            ),
            Direction::ColumnReverse(breakpoint) => concat_string!(
                "pt-flex__container pt-flex__col pt-flex__reverse ", breakpoint.to_string()
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
            WrapAlign::Wrap(a)        => concat_string!("pt-flex__wrap ", a.to_string()),
            WrapAlign::WrapReverse(a) => concat_string!("pt-flex__wrap-reverse ", a.to_string()),
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
            ContentAlign::Start        => "pt-flex__align-start",
            ContentAlign::End          => "pt-flex__align-end",
            ContentAlign::Center       => "pt-flex__align-center",
            ContentAlign::Stretch      => "pt-flex__align-stretch",
            ContentAlign::SpaceBetween => "pt-flex__align-space-between",
            ContentAlign::SpaceAround  => "pt-flex__align-space-around",
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
            ContentJustify::Start        => "pt-flex__justify-start",
            ContentJustify::End          => "pt-flex__justify-end",
            ContentJustify::Center       => "pt-flex__justify-center",
            ContentJustify::SpaceBetween => "pt-flex__justify-space-between",
            ContentJustify::SpaceAround  => "pt-flex__justify-space-around",
            ContentJustify::SpaceEvenly  => "pt-flex__justify-space-evenly",
        })
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
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
        String::from(match self {
            ItemAlign::Default  => "",
            ItemAlign::Top      => "pt-flex__item-top",
            ItemAlign::Bottom   => "pt-flex__item-bottom",
            ItemAlign::Middle   => "pt-flex__item-middle",
            ItemAlign::Stretch  => "pt-flex__item-stretch",
            ItemAlign::Baseline => "pt-flex__item-baseline",
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

#[rustfmt::skip]
impl ToString for ItemGrow {
    fn to_string(&self) -> String {
        String::from(match self {
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

#[rustfmt::skip]
impl ToString for ItemShrink {
    fn to_string(&self) -> String {
        String::from(match self {
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

#[rustfmt::skip]
impl ToString for ItemSize {
    fn to_string(&self) -> String {
        String::from(match self {
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

#[rustfmt::skip]
impl ToString for ItemOffset {
    fn to_string(&self) -> String {
        String::from(match self {
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
        })
    }
}
