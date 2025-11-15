//! Componentes nativos proporcionados por PageTop.

use crate::prelude::*;

// **< FontSize >***********************************************************************************

#[derive(AutoDefault, Clone, Copy, Debug, PartialEq)]
pub enum FontSize {
    ExtraLarge,
    XxLarge,
    XLarge,
    Large,
    Medium,
    #[default]
    Normal,
    Small,
    XSmall,
    XxSmall,
    ExtraSmall,
}

#[rustfmt::skip]
impl FontSize {
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            FontSize::ExtraLarge => "fs__x3l",
            FontSize::XxLarge    => "fs__x2l",
            FontSize::XLarge     => "fs__xl",
            FontSize::Large      => "fs__l",
            FontSize::Medium     => "fs__m",
            FontSize::Normal     => "",
            FontSize::Small      => "fs__s",
            FontSize::XSmall     => "fs__xs",
            FontSize::XxSmall    => "fs__x2s",
            FontSize::ExtraSmall => "fs__x3s",
        }
    }
}

// *************************************************************************************************

mod html;
pub use html::Html;

mod block;
pub use block::Block;

mod intro;
pub use intro::{Intro, IntroOpening};

mod poweredby;
pub use poweredby::PoweredBy;

mod icon;
pub use icon::{Icon, IconKind};
