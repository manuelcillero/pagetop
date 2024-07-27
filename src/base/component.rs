use crate::core::component::{AssetsOp, Context};
use crate::html::{JavaScript, StyleSheet};
use crate::{AutoDefault, Weight};

// Context parameters.
pub const PARAM_BASE_WEIGHT: &str = "base.weight";
pub const PARAM_BASE_INCLUDE_ICONS: &str = "base.include.icon";
pub const PARAM_BASE_INCLUDE_FLEX_ASSETS: &str = "base.include.flex";
pub const PARAM_BASE_INCLUDE_MENU_ASSETS: &str = "base.include.menu";

pub(crate) fn add_base_assets(cx: &mut Context) {
    let weight = cx.get_param::<Weight>(PARAM_BASE_WEIGHT).unwrap_or(-90);

    cx.set_assets(AssetsOp::AddStyleSheet(
        StyleSheet::at("/base/css/root.css")
            .with_version("0.0.1")
            .with_weight(weight),
    ))
    .set_assets(AssetsOp::AddStyleSheet(
        StyleSheet::at("/base/css/looks.css")
            .with_version("0.0.1")
            .with_weight(weight),
    ))
    .set_assets(AssetsOp::AddStyleSheet(
        StyleSheet::at("/base/css/buttons.css")
            .with_version("0.0.2")
            .with_weight(weight),
    ));

    if let Ok(true) = cx.get_param::<bool>(PARAM_BASE_INCLUDE_ICONS) {
        cx.set_assets(AssetsOp::AddStyleSheet(
            StyleSheet::at("/base/css/icons.min.css")
                .with_version("1.11.1")
                .with_weight(weight),
        ));
    }

    if let Ok(true) = cx.get_param::<bool>(PARAM_BASE_INCLUDE_FLEX_ASSETS) {
        cx.set_assets(AssetsOp::AddStyleSheet(
            StyleSheet::at("/base/css/flex.css")
                .with_version("0.0.1")
                .with_weight(weight),
        ));
    }

    if let Ok(true) = cx.get_param::<bool>(PARAM_BASE_INCLUDE_MENU_ASSETS) {
        cx.set_assets(AssetsOp::AddStyleSheet(
            StyleSheet::at("/base/css/menu.css")
                .with_version("0.0.1")
                .with_weight(weight),
        ))
        .set_assets(AssetsOp::AddJavaScript(
            JavaScript::at("/base/js/menu.js")
                .with_version("0.0.1")
                .with_weight(weight),
        ));
    }
}

// *************************************************************************************************

#[rustfmt::skip]
#[derive(AutoDefault)]
pub enum BreakPoint {
    #[default]
    None,  // Does not apply. Rest initially assume 1 pixel = 0.0625rem
    SM,    // @media screen and [ (max-width: 35.5rem) <=  568px < (min-width: 35.5625rem) ]
    MD,    // @media screen and [ (max-width: 48rem)   <=  768px < (min-width: 48.0625rem)   ]
    LG,    // @media screen and [ (max-width: 62rem)   <=  992px < (min-width: 62.0625rem)   ]
    XL,    // @media screen and [ (max-width: 80rem)   <= 1280px < (min-width: 80.0625rem)   ]
    X2L,   // @media screen and [ (max-width: 90rem)   <= 1440px < (min-width: 90.0625rem)   ]
    X3L,   // @media screen and [ (max-width: 120rem)  <= 1920px < (min-width: 120.0625rem)  ]
    X2K,   // @media screen and [ (max-width: 160rem)  <= 2560px < (min-width: 160.0625rem)  ]
}

#[rustfmt::skip]
impl ToString for BreakPoint {
    fn to_string(&self) -> String {
        String::from(match self {
            BreakPoint::None => "bp__none",
            BreakPoint::SM   => "bp__sm",
            BreakPoint::MD   => "bp__md",
            BreakPoint::LG   => "bp__lg",
            BreakPoint::XL   => "bp__xl",
            BreakPoint::X2L  => "bp__x2l",
            BreakPoint::X3L  => "bp__x3l",
            BreakPoint::X2K  => "bp__x2k",
        })
    }
}

// *************************************************************************************************

#[derive(AutoDefault)]
pub enum StyleBase {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Danger,
    Light,
    Dark,
    Link,
}

#[rustfmt::skip]
impl ToString for StyleBase {
    fn to_string(&self) -> String {
        String::from(match self {
            StyleBase::Default => "style__default",
            StyleBase::Info    => "style__info",
            StyleBase::Success => "style__success",
            StyleBase::Warning => "style__warning",
            StyleBase::Danger  => "style__danger",
            StyleBase::Light   => "style__light",
            StyleBase::Dark    => "style__dark",
            StyleBase::Link    => "style__link",
        })
    }
}

// *************************************************************************************************

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
impl ToString for FontSize {
    fn to_string(&self) -> String {
        String::from(match self {
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
        })
    }
}

// *************************************************************************************************

pub mod flex;

mod basic;
pub use basic::*;

mod error403;
pub use error403::Error403;

mod error404;
pub use error404::Error404;

mod heading;
pub use heading::{Heading, HeadingSize, HeadingType};

mod paragraph;
pub use paragraph::Paragraph;

mod icon;
pub use icon::Icon;

mod button;
pub use button::{Button, ButtonTarget};

mod image;
pub use image::{Image, ImageSize};

mod block;
pub use block::Block;

mod branding;
pub use branding::Branding;

mod powered_by;
pub use powered_by::{PoweredBy, PoweredByLogo};

pub mod menu;
pub use menu::Menu;

pub mod form;
pub use form::{Form, FormMethod};
