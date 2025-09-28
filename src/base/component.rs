//! Componentes nativos proporcionados por PageTop.

use crate::AutoDefault;

use std::fmt;

// **< FontSize >***********************************************************************************

#[derive(AutoDefault)]
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
    pub const fn as_str(&self) -> &'static str {
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

impl fmt::Display for FontSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// *************************************************************************************************

mod html;
pub use html::Html;

mod block;
pub use block::Block;

mod poweredby;
pub use poweredby::PoweredBy;

mod icon;
pub use icon::{Icon, IconKind};

pub mod menu;
