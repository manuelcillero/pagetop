use crate::core::component::{Context, ContextOp};
use crate::html::{JavaScript, StyleSheet};
use crate::Weight;

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
    ));
}

#[rustfmt::skip]
#[derive(Default)]
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
        match self {
            BreakPoint::None => "pt-bp__none".to_string(),
            BreakPoint::SM   => "pt-bp__sm".to_string(),
            BreakPoint::MD   => "pt-bp__md".to_string(),
            BreakPoint::LG   => "pt-bp__lg".to_string(),
            BreakPoint::XL   => "pt-bp__xl".to_string(),
            BreakPoint::X2L  => "pt-bp__x2l".to_string(),
            BreakPoint::X3L  => "pt-bp__x3l".to_string(),
            BreakPoint::X2K  => "pt-bp__x2k".to_string(),
        }
    }
}

mod html;
pub use html::{Html, COMPONENT_BASE_HTML};

mod l10n;
pub use l10n::{L10n, COMPONENT_BASE_L10N};

mod wrapper;
pub use wrapper::{Wrapper, WrapperType, COMPONENT_BASE_WRAPPER};

pub mod flex;

mod icon;
pub use icon::{Icon, COMPONENT_BASE_ICON};
mod heading;
pub use heading::{Heading, HeadingDisplay, HeadingType, COMPONENT_BASE_HEADING};
mod paragraph;
pub use paragraph::{Paragraph, ParagraphDisplay, COMPONENT_BASE_PARAGRAPH};
mod anchor;
pub use anchor::{Anchor, AnchorTarget, AnchorType, COMPONENT_BASE_ANCHOR};
mod image;
pub use image::{Image, ImageSize, COMPONENT_BASE_IMAGE};
mod block;
pub use block::{Block, COMPONENT_BASE_BLOCK};
mod branding;
pub use branding::{Branding, COMPONENT_BASE_BRANDING};
mod powered_by;
pub use powered_by::{PoweredBy, PoweredByLogo, COMPONENT_BASE_POWEREDBY};

pub mod menu;
pub use menu::{Menu, COMPONENT_BASE_MENU};

pub mod form;
pub use form::{Form, FormMethod, COMPONENT_BASE_FORM};
