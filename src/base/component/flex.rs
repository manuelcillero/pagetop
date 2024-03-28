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
pub enum Wrap {
    #[default]
    Default,
    NoWrap,
    Wrap(ContentAlign),
    WrapReverse(ContentAlign),
}

#[rustfmt::skip]
impl ToString for Wrap {
    fn to_string(&self) -> String {
        match self {
            Wrap::Default        => "".to_owned(),
            Wrap::NoWrap         => "flex__nowrap".to_owned(),
            Wrap::Wrap(a)        => concat_string!("flex__wrap ", a.to_string()),
            Wrap::WrapReverse(a) => concat_string!("flex__wrap-reverse ", a.to_string()),
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
impl ToString for Justify {
    fn to_string(&self) -> String {
        String::from(match self {
            Justify::Default      => "",
            Justify::Start        => "flex__justify-start",
            Justify::End          => "flex__justify-end",
            Justify::Center       => "flex__justify-center",
            Justify::SpaceBetween => "flex__justify-space-between",
            Justify::SpaceAround  => "flex__justify-space-around",
            Justify::SpaceEvenly  => "flex__justify-space-evenly",
        })
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
impl ToString for Align {
    fn to_string(&self) -> String {
        String::from(match self {
            Align::Default  => "",
            Align::Start    => "flex__start",
            Align::End      => "flex__end",
            Align::Center   => "flex__center",
            Align::Stretch  => "flex__stretch",
            Align::Baseline => "flex__baseline",
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

impl ToString for Grow {
    fn to_string(&self) -> String {
        String::from(match self {
            Grow::Default => "",
            Grow::Is1 => "flex__grow-1",
            Grow::Is2 => "flex__grow-2",
            Grow::Is3 => "flex__grow-3",
            Grow::Is4 => "flex__grow-4",
            Grow::Is5 => "flex__grow-5",
            Grow::Is6 => "flex__grow-6",
            Grow::Is7 => "flex__grow-7",
            Grow::Is8 => "flex__grow-8",
            Grow::Is9 => "flex__grow-9",
        })
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

impl ToString for Shrink {
    fn to_string(&self) -> String {
        String::from(match self {
            Shrink::Default => "",
            Shrink::Is1 => "flex__shrink-1",
            Shrink::Is2 => "flex__shrink-2",
            Shrink::Is3 => "flex__shrink-3",
            Shrink::Is4 => "flex__shrink-4",
            Shrink::Is5 => "flex__shrink-5",
            Shrink::Is6 => "flex__shrink-6",
            Shrink::Is7 => "flex__shrink-7",
            Shrink::Is8 => "flex__shrink-8",
            Shrink::Is9 => "flex__shrink-9",
        })
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

impl ToString for Size {
    fn to_string(&self) -> String {
        String::from(match self {
            Size::Default => "",
            Size::Percent10 => "flex__size-10",
            Size::Percent20 => "flex__size-20",
            Size::Percent25 => "flex__size-25",
            Size::Percent33 => "flex__size-33",
            Size::Percent40 => "flex__size-40",
            Size::Percent50 => "flex__size-50",
            Size::Percent60 => "flex__size-60",
            Size::Percent66 => "flex__size-66",
            Size::Percent75 => "flex__size-75",
            Size::Percent80 => "flex__size-80",
            Size::Percent90 => "flex__size-90",
        })
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

impl ToString for Offset {
    fn to_string(&self) -> String {
        String::from(match self {
            Offset::Default => "",
            Offset::Offset10 => "flex__offset-10",
            Offset::Offset20 => "flex__offset-20",
            Offset::Offset25 => "flex__offset-25",
            Offset::Offset33 => "flex__offset-33",
            Offset::Offset40 => "flex__offset-40",
            Offset::Offset50 => "flex__offset-50",
            Offset::Offset60 => "flex__offset-60",
            Offset::Offset66 => "flex__offset-66",
            Offset::Offset75 => "flex__offset-75",
            Offset::Offset80 => "flex__offset-80",
            Offset::Offset90 => "flex__offset-90",
        })
    }
}
