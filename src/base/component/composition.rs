mod container;
pub use container::Container;

mod flex;
pub use flex::Flex;

mod layout;
pub use layout::Layout;

mod region;
pub use region::Region;

use crate::prelude::*;

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum FlexDirection {
    #[default]
    Default,
    Row(BreakPoint),
    RowReverse(BreakPoint),
    Column(BreakPoint),
    ColumnReverse(BreakPoint),
}

#[rustfmt::skip]
impl ToString for FlexDirection {
    fn to_string(&self) -> String {
        match self {
            FlexDirection::Default => concat_string!(
                "flex__row ", BreakPoint::default().to_string()
            ),
            FlexDirection::Row(breakpoint) => concat_string!(
                "flex__row ", breakpoint.to_string()
            ),
            FlexDirection::RowReverse(breakpoint) => concat_string!(
                "flex__row flex__reverse ", breakpoint.to_string()
            ),
            FlexDirection::Column(breakpoint) => concat_string!(
                "flex__col ", breakpoint.to_string()
            ),
            FlexDirection::ColumnReverse(breakpoint) => concat_string!(
                "flex__col flex__reverse ", breakpoint.to_string()
            ),
        }
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum FlexWrap {
    #[default]
    Default,
    NoWrap,
    Wrap(ContentAlign),
    WrapReverse(ContentAlign),
}

#[rustfmt::skip]
impl ToString for FlexWrap {
    fn to_string(&self) -> String {
        match self {
            FlexWrap::Default        => "".to_owned(),
            FlexWrap::NoWrap         => "flex__nowrap".to_owned(),
            FlexWrap::Wrap(a)        => concat_string!("flex__wrap ", a.to_string()),
            FlexWrap::WrapReverse(a) => concat_string!("flex__wrap-reverse ", a.to_string()),
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
pub enum FlexJustify {
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
impl ToString for FlexJustify {
    fn to_string(&self) -> String {
        String::from(match self {
            FlexJustify::Default      => "",
            FlexJustify::Start        => "flex__justify-start",
            FlexJustify::End          => "flex__justify-end",
            FlexJustify::Center       => "flex__justify-center",
            FlexJustify::SpaceBetween => "flex__justify-space-between",
            FlexJustify::SpaceAround  => "flex__justify-space-around",
            FlexJustify::SpaceEvenly  => "flex__justify-space-evenly",
        })
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum FlexAlign {
    #[default]
    Default,
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

#[rustfmt::skip]
impl ToString for FlexAlign {
    fn to_string(&self) -> String {
        String::from(match self {
            FlexAlign::Default  => "",
            FlexAlign::Start    => "flex__start",
            FlexAlign::End      => "flex__end",
            FlexAlign::Center   => "flex__center",
            FlexAlign::Stretch  => "flex__stretch",
            FlexAlign::Baseline => "flex__baseline",
        })
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum FlexGap {
    #[default]
    Default,
    Row(unit::Value),
    Column(unit::Value),
    Distinct(unit::Value, unit::Value),
    Both(unit::Value),
}

#[rustfmt::skip]
impl ToString for FlexGap {
    fn to_string(&self) -> String {
        match self {
            FlexGap::Default        => "".to_owned(),
            FlexGap::Row(r)         => concat_string!("row-gap: ", r.to_string(), ";"),
            FlexGap::Column(c)      => concat_string!("column-gap: ", c.to_string(), ";"),
            FlexGap::Distinct(r, c) => concat_string!("gap: ", r.to_string(), " ", c.to_string(), ";"),
            FlexGap::Both(v)        => concat_string!("gap: ", v.to_string(), ";"),
        }
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum FlexGrow {
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

impl ToString for FlexGrow {
    fn to_string(&self) -> String {
        String::from(match self {
            FlexGrow::Default => "",
            FlexGrow::Is1 => "flex__grow-1",
            FlexGrow::Is2 => "flex__grow-2",
            FlexGrow::Is3 => "flex__grow-3",
            FlexGrow::Is4 => "flex__grow-4",
            FlexGrow::Is5 => "flex__grow-5",
            FlexGrow::Is6 => "flex__grow-6",
            FlexGrow::Is7 => "flex__grow-7",
            FlexGrow::Is8 => "flex__grow-8",
            FlexGrow::Is9 => "flex__grow-9",
        })
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum FlexShrink {
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

impl ToString for FlexShrink {
    fn to_string(&self) -> String {
        String::from(match self {
            FlexShrink::Default => "",
            FlexShrink::Is1 => "flex__shrink-1",
            FlexShrink::Is2 => "flex__shrink-2",
            FlexShrink::Is3 => "flex__shrink-3",
            FlexShrink::Is4 => "flex__shrink-4",
            FlexShrink::Is5 => "flex__shrink-5",
            FlexShrink::Is6 => "flex__shrink-6",
            FlexShrink::Is7 => "flex__shrink-7",
            FlexShrink::Is8 => "flex__shrink-8",
            FlexShrink::Is9 => "flex__shrink-9",
        })
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum FlexSize {
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

impl ToString for FlexSize {
    fn to_string(&self) -> String {
        String::from(match self {
            FlexSize::Default => "",
            FlexSize::Percent10 => "flex__size-10",
            FlexSize::Percent20 => "flex__size-20",
            FlexSize::Percent25 => "flex__size-25",
            FlexSize::Percent33 => "flex__size-33",
            FlexSize::Percent40 => "flex__size-40",
            FlexSize::Percent50 => "flex__size-50",
            FlexSize::Percent60 => "flex__size-60",
            FlexSize::Percent66 => "flex__size-66",
            FlexSize::Percent75 => "flex__size-75",
            FlexSize::Percent80 => "flex__size-80",
            FlexSize::Percent90 => "flex__size-90",
        })
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum FlexOffset {
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

impl ToString for FlexOffset {
    fn to_string(&self) -> String {
        String::from(match self {
            FlexOffset::Default => "",
            FlexOffset::Offset10 => "flex__offset-10",
            FlexOffset::Offset20 => "flex__offset-20",
            FlexOffset::Offset25 => "flex__offset-25",
            FlexOffset::Offset33 => "flex__offset-33",
            FlexOffset::Offset40 => "flex__offset-40",
            FlexOffset::Offset50 => "flex__offset-50",
            FlexOffset::Offset60 => "flex__offset-60",
            FlexOffset::Offset66 => "flex__offset-66",
            FlexOffset::Offset75 => "flex__offset-75",
            FlexOffset::Offset80 => "flex__offset-80",
            FlexOffset::Offset90 => "flex__offset-90",
        })
    }
}
