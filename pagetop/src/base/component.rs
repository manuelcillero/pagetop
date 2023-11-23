use crate::core::component::{Context, ContextOp};
use crate::html::{JavaScript, StyleSheet};
use crate::{SmartDefault, Weight};

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
            .with_version("0.0.1")
            .with_weight(weight),
    ));
}

// *************************************************************************************************

#[rustfmt::skip]
#[derive(SmartDefault)]
pub enum BreakPoint {
    #[default]
    None,  /* Does not apply. Rest initially assume 1 pixel = 0.0625em */
    SM,    /* PageTop default applies to <= 568px  - @media screen and (max-width: 35.5em) */
    MD,    /* PageTop default applies to <= 768px  - @media screen and (max-width: 48em)   */
    LG,    /* PageTop default applies to <= 992px  - @media screen and (max-width: 62em)   */
    XL,    /* PageTop default applies to <= 1280px - @media screen and (max-width: 80em)   */
    X2L,   /* PageTop default applies to <= 1440px - @media screen and (max-width: 90em)   */
    X3L,   /* PageTop default applies to <= 1920px - @media screen and (max-width: 120em)  */
    X2K,   /* PageTop default applies to <= 2560px - @media screen and (max-width: 160em)  */
}

#[rustfmt::skip]
impl ToString for BreakPoint {
    fn to_string(&self) -> String {
        String::from(match self {
            BreakPoint::None => "pt-bp__none",
            BreakPoint::SM   => "pt-bp__sm",
            BreakPoint::MD   => "pt-bp__md",
            BreakPoint::LG   => "pt-bp__lg",
            BreakPoint::XL   => "pt-bp__xl",
            BreakPoint::X2L  => "pt-bp__x2l",
            BreakPoint::X3L  => "pt-bp__x3l",
            BreakPoint::X2K  => "pt-bp__x2k",
        })
    }
}

// *************************************************************************************************

#[derive(SmartDefault)]
pub enum ButtonStyle {
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
impl ToString for ButtonStyle {
    fn to_string(&self) -> String {
        String::from(match self {
            ButtonStyle::Default   => "pt-button__default",
            ButtonStyle::Info      => "pt-button__info",
            ButtonStyle::Success   => "pt-button__success",
            ButtonStyle::Warning   => "pt-button__warning",
            ButtonStyle::Danger    => "pt-button__danger",
            ButtonStyle::Light     => "pt-button__light",
            ButtonStyle::Dark      => "pt-button__dark",
            ButtonStyle::Link      => "pt-button__link",
        })
    }
}

// *************************************************************************************************

#[derive(SmartDefault)]
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
            FontSize::ExtraLarge => "pt-fs__x3l",
            FontSize::XxLarge    => "pt-fs__x2l",
            FontSize::XLarge     => "pt-fs__xl",
            FontSize::Large      => "pt-fs__l",
            FontSize::Medium     => "pt-fs__m",
            FontSize::Normal     => "",
            FontSize::Small      => "pt-fs__s",
            FontSize::XSmall     => "pt-fs__xs",
            FontSize::XxSmall    => "pt-fs__x2s",
            FontSize::ExtraSmall => "pt-fs__x3s",
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
