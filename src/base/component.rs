use crate::core::component::{Context, ContextOp};
use crate::html::{JavaScript, StyleSheet};
use crate::{AutoDefault, Weight};

// Context parameters.
pub const PARAM_BASE_WEIGHT: &str = "base.weight";
pub const PARAM_BASE_INCLUDE_ICONS: &str = "base.include.icon";
pub const PARAM_BASE_INCLUDE_FLEX_ASSETS: &str = "base.include.flex";
pub const PARAM_BASE_INCLUDE_MENU_ASSETS: &str = "base.include.menu";

pub(crate) fn add_base_assets(cx: &mut Context) {
    let weight = cx.get_param::<Weight>(PARAM_BASE_WEIGHT).unwrap_or(-90);

    cx.alter(ContextOp::AddStyleSheet(
        StyleSheet::at("/base/css/root.css")
            .with_version("0.0.1")
            .with_weight(weight),
    ));
    if let Some(true) = cx.get_param::<bool>(PARAM_BASE_INCLUDE_ICONS) {
        cx.alter(ContextOp::AddStyleSheet(
            StyleSheet::at("/base/css/icons.min.css")
                .with_version("1.11.1")
                .with_weight(weight),
        ));
    }
    if let Some(true) = cx.get_param::<bool>(PARAM_BASE_INCLUDE_FLEX_ASSETS) {
        cx.alter(ContextOp::AddStyleSheet(
            StyleSheet::at("/base/css/flex.css")
                .with_version("0.0.1")
                .with_weight(weight),
        ));
    }
    if let Some(true) = cx.get_param::<bool>(PARAM_BASE_INCLUDE_MENU_ASSETS) {
        cx.alter(ContextOp::AddStyleSheet(
            StyleSheet::at("/base/css/menu.css")
                .with_version("0.0.1")
                .with_weight(weight),
        ))
        .alter(ContextOp::AddJavaScript(
            JavaScript::at("/base/js/menu.js")
                .with_version("0.0.1")
                .with_weight(weight),
        ));
    }
    cx.alter(ContextOp::AddStyleSheet(
        StyleSheet::at("/base/css/looks.css")
            .with_version("0.0.1")
            .with_weight(weight),
    ))
    .alter(ContextOp::AddStyleSheet(
        StyleSheet::at("/base/css/buttons.css")
            .with_version("0.0.2")
            .with_weight(weight),
    ));
}

// *************************************************************************************************

#[rustfmt::skip]
#[derive(AutoDefault)]
pub enum BreakPoint {
    #[default]
    None,  /* Does not apply. Rest initially assume 1 pixel = 0.0625rem */
    SM,    /* PageTop default applies to <= 568px  - @media screen and (max-width: 35.5rem) */
    MD,    /* PageTop default applies to <= 768px  - @media screen and (max-width: 48rem)   */
    LG,    /* PageTop default applies to <= 992px  - @media screen and (max-width: 62rem)   */
    XL,    /* PageTop default applies to <= 1280px - @media screen and (max-width: 80rem)   */
    X2L,   /* PageTop default applies to <= 1440px - @media screen and (max-width: 90rem)   */
    X3L,   /* PageTop default applies to <= 1920px - @media screen and (max-width: 120rem)  */
    X2K,   /* PageTop default applies to <= 2560px - @media screen and (max-width: 160rem)  */
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
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    Link,
}

#[rustfmt::skip]
impl ToString for StyleBase {
    fn to_string(&self) -> String {
        String::from(match self {
            StyleBase::Default => "style__default",
            StyleBase::Success => "style__success",
            StyleBase::Danger  => "style__danger",
            StyleBase::Warning => "style__warning",
            StyleBase::Info    => "style__info",
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

mod html;
pub use html::Html;

mod translate;
pub use translate::Translate;

mod wrapper;
pub use wrapper::{Wrapper, WrapperType};

pub mod flex;

mod icon;
pub use icon::Icon;

mod heading;
pub use heading::{Heading, HeadingSize, HeadingType};

mod paragraph;
pub use paragraph::Paragraph;

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

mod error403;
pub use error403::Error403;

mod error404;
pub use error404::Error404;

pub mod menu;
pub use menu::Menu;

pub mod form;
pub use form::{Form, FormMethod};
