use pagetop::prelude::*;

use crate::bs::BreakPoint;

use std::fmt;

mod component;
pub use component::Grid;

mod item;
pub use item::Item;

#[derive(AutoDefault)]
pub enum Layout {
    #[default]
    Default,
    Rows(u8),
    Cols(u8),
    Grid(u8, u8),
}

#[rustfmt::skip]
impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Layout::Rows(r) if *r > 1 => write!(f, "--bs-rows: {r};"),
            Layout::Cols(c) if *c > 0 => write!(f, "--bs-columns: {c};"),
            Layout::Grid(r, c)        => write!(f, "{}", trio_string!(
                Layout::Rows(*r).to_string(), " ", Layout::Cols(*c).to_string()
            )),
            _ => write!(f, ""),
        }
    }
}

#[derive(AutoDefault)]
pub enum Gap {
    #[default]
    Default,
    Row(unit::Value),
    Col(unit::Value),
    Grid(unit::Value, unit::Value),
    Both(unit::Value),
}

#[rustfmt::skip]
impl fmt::Display for Gap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gap::Default    => write!(f, ""),
            Gap::Row(r)     => write!(f, "row-gap: {r};"),
            Gap::Col(c)     => write!(f, "column-gap: {c};"),
            Gap::Grid(r, c) => write!(f, "--bs-gap: {r} {c};"),
            Gap::Both(v)    => write!(f, "--bs-gap: {v};"),
        }
    }
}

#[derive(AutoDefault)]
pub enum ItemColumns {
    #[default]
    Default,
    Cols(u8),
}

#[rustfmt::skip]
impl fmt::Display for ItemColumns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemColumns::Cols(c) if *c > 1 => write!(f, "g-col-{c}"),
            _ => write!(f, ""),
        }
    }
}

#[derive(AutoDefault)]
pub enum ItemResponsive {
    #[default]
    Default,
    Cols(BreakPoint, u8),
}

#[rustfmt::skip]
impl fmt::Display for ItemResponsive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemResponsive::Cols(bp, c) if bp.is_breakpoint() && *c > 0 => {
                write!(f, "g-col-{bp}-{c}")
            }
            _ => write!(f, ""),
        }
    }
}

#[derive(AutoDefault)]
pub enum ItemStart {
    #[default]
    Default,
    Col(u8),
}

#[rustfmt::skip]
impl fmt::Display for ItemStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemStart::Col(c) if *c > 1 => write!(f, "g-start-{c}"),
            _ => write!(f, ""),
        }
    }
}
