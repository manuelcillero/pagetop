use pagetop::prelude::*;

pub mod component;

new_handle!(MODULE_MINIMAL);

static_locales!(LOCALES_MINIMAL);

static_files!(minimal);

// Library version.
const VERSION_MINIMAL: &str = env!("CARGO_PKG_VERSION");

// Context parameter.
const PARAM_MINIMAL_FLEX: &str = "minimal.flex";

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

pub struct Minimal;

impl ModuleTrait for Minimal {
    fn handle(&self) -> Handle {
        MODULE_MINIMAL
    }

    fn name(&self) -> L10n {
        L10n::t("module_name", &LOCALES_MINIMAL)
    }

    fn description(&self) -> L10n {
        L10n::t("module_description", &LOCALES_MINIMAL)
    }

    fn actions(&self) -> Vec<Action> {
        actions![ActionAfterPrepareBody::with(after_prepare_body).with_weight(99)]
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        static_files_service!(scfg, "/minimal", minimal);
    }
}

impl Minimal {
    pub(crate) fn load_assets(&self, cx: &mut Context) -> &Self {
        cx.set_param::<bool>(PARAM_MINIMAL_ASSETS, true);
        self
    }
}

fn after_prepare_body(page: &mut Page) {
    if let Some(true) = page.context().get_param::<bool>(PARAM_MINIMAL_ASSETS) {
        page.context().alter(ContextOp::AddStyleSheet(
            StyleSheet::at("/minimal/css/minimal.css").with_version(VERSION_MINIMAL),
        ));
    }
}
