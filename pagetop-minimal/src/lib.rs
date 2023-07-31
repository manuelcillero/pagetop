use pagetop::prelude::*;

pub mod component;

new_handle!(MODULE_MINIMAL);

static_locales!(LOCALES_MINIMAL);

static_files!(minimal);

// Library version.
const VERSION_MINIMAL: &str = env!("CARGO_PKG_VERSION");

// Context parameter.
const PARAM_MINIMAL_ASSETS: &str = "minimal.assets";

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
        vec![action!(ActionAfterPrepareBody => after_prepare_body, 99)]
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        serve_static_files!(scfg, "/minimal", minimal);
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
