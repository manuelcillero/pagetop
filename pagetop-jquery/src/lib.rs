use pagetop::prelude::*;

new_handle!(MODULE_JQUERY);

static_locales!(LOCALES_JQUERY);

static_files!(jquery);

// Library version.
const VERSION_JQUERY: &str = "3.6.0";

// Context parameter.
const PARAM_JQUERY: &str = "jquery.lib";

/// Implements [`ModuleTrait`](pagetop::core::module::ModuleTrait) and specific module API.
pub struct JQuery;

impl ModuleTrait for JQuery {
    fn handle(&self) -> Handle {
        MODULE_JQUERY
    }

    fn name(&self) -> L10n {
        L10n::t("module_name", &LOCALES_JQUERY)
    }

    fn description(&self) -> L10n {
        L10n::t("module_description", &LOCALES_JQUERY)
    }

    fn actions(&self) -> Vec<Action> {
        vec![action!(ActionAfterPrepareBody => after_prepare_body)]
    }

    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
        static_files_service!(scfg, "/jquery", jquery);
    }
}

impl JQuery {
    pub fn enable_jquery(&self, cx: &mut Context) -> &Self {
        cx.set_param::<bool>(PARAM_JQUERY, true);
        self
    }

    pub fn disable_jquery(&self, cx: &mut Context) -> &Self {
        cx.set_param::<bool>(PARAM_JQUERY, false);
        self
    }
}

fn after_prepare_body(page: &mut Page) {
    if let Some(true) = page.context().get_param::<bool>(PARAM_JQUERY) {
        page.context().alter(ContextOp::AddJavaScript(
            JavaScript::at("/jquery/jquery.min.js")
                .with_version(VERSION_JQUERY)
                .with_weight(-99)
                .with_mode(ModeJS::Normal),
        ));
    }
}
