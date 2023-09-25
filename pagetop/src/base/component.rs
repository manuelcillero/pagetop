// Context parameters.
pub const PARAM_INCLUDE_FLEX: &str = "theme.include.flex";
pub const PARAM_INCLUDE_ICONS: &str = "theme.include.icons";

#[rustfmt::skip]
#[derive(Default)]
pub enum BreakPoint {
    #[default]
    None,  /* Does not apply */
    SM,    /* @media screen and (max-width: 35.5em) - Applies <= 568px  */
    MD,    /* @media screen and (max-width: 48em)   - Applies <= 768px  */
    LG,    /* @media screen and (max-width: 64em)   - Applies <= 1024px */
    XL,    /* @media screen and (max-width: 80em)   - Applies <= 1280px */
    X2L,   /* @media screen and (max-width: 120em)  - Applies <= 1920px */
    X3L,   /* @media screen and (max-width: 160em)  - Applies <= 2560px */
}

#[rustfmt::skip]
impl ToString for BreakPoint {
    fn to_string(&self) -> String {
        match self {
            BreakPoint::None => "bp-no".to_string(),
            BreakPoint::SM   => "bp-sm".to_string(),
            BreakPoint::MD   => "bp-md".to_string(),
            BreakPoint::LG   => "bp-lg".to_string(),
            BreakPoint::XL   => "bp-xl".to_string(),
            BreakPoint::X2L  => "bp-x2l".to_string(),
            BreakPoint::X3L  => "bp-x3l".to_string(),
        }
    }
}

mod html;
pub use html::{Html, COMPONENT_HTML};

mod l10n;
pub use l10n::{L10n, COMPONENT_L10N};

mod wrapper;
pub use wrapper::{Wrapper, WrapperType, COMPONENT_WRAPPER};

pub mod flex;

mod icon;
pub use icon::{Icon, COMPONENT_ICON};
mod heading;
pub use heading::{Heading, HeadingDisplay, HeadingType, COMPONENT_HEADING};
mod paragraph;
pub use paragraph::{Paragraph, ParagraphDisplay, COMPONENT_PARAGRAPH};
mod anchor;
pub use anchor::{Anchor, AnchorTarget, AnchorType, COMPONENT_ANCHOR};
mod image;
pub use image::{Image, ImageSize, COMPONENT_IMAGE};
mod block;
pub use block::{Block, COMPONENT_BLOCK};
mod site_branding;
pub use site_branding::{SiteBranding, COMPONENT_BRANDING};
mod powered_by;
pub use powered_by::{PoweredBy, PoweredByLogo, COMPONENT_POWEREDBY};

pub mod form_element;
pub use form_element::{Form, FormMethod, COMPONENT_FORM};
