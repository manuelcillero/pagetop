use pagetop::prelude::*;

use std::fmt;

mod component;
pub use component::Offcanvas;

#[derive(AutoDefault)]
pub enum OffcanvasPlacement {
    #[default]
    Start,
    End,
    Top,
    Bottom,
}

#[rustfmt::skip]
impl fmt::Display for OffcanvasPlacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OffcanvasPlacement::Start  => write!(f, "offcanvas-start"),
            OffcanvasPlacement::End    => write!(f, "offcanvas-end"),
            OffcanvasPlacement::Top    => write!(f, "offcanvas-top"),
            OffcanvasPlacement::Bottom => write!(f, "offcanvas-bottom"),
        }
    }
}

#[derive(AutoDefault)]
pub enum OffcanvasVisibility {
    #[default]
    Default,
    Show,
}

#[rustfmt::skip]
impl fmt::Display for OffcanvasVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OffcanvasVisibility::Default => write!(f, "show"),
            OffcanvasVisibility::Show    => write!(f, ""),
        }
    }
}

#[derive(AutoDefault)]
pub enum OffcanvasBodyScroll {
    #[default]
    Disabled,
    Enabled,
}

#[derive(AutoDefault)]
pub enum OffcanvasBackdrop {
    Disabled,
    #[default]
    Enabled,
    Static,
}
