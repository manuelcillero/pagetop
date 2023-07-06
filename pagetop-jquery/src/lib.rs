use pagetop::prelude::*;

use_handle!(MODULE_JQUERY);

use_locale!(LOCALE_JQUERY);

use_static!(jquery);

const PARAM_JQUERY: &str = "jquery.js";

pub struct JQuery;

impl ModuleTrait for JQuery {
    fn handle(&self) -> Handle {
        MODULE_JQUERY
    }

    fn name(&self) -> L10n {
        L10n::t("module_name", &LOCALE_JQUERY)
    }

    fn description(&self) -> L10n {
        L10n::t("module_description", &LOCALE_JQUERY)
    }

    fn actions(&self) -> Vec<Action> {
        vec![action!(actions::page::ActionBeforeRenderPage => before_render_page)]
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        serve_static_files!(cfg, "/jquery", jquery);
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

fn before_render_page(page: &mut Page) {
    if let Some(true) = page.context().get_param::<bool>(PARAM_JQUERY) {
        page.context().alter(ContextOp::AddJavaScript(
            JavaScript::located("/jquery/jquery.min.js")
                .with_version("3.6.0")
                .with_weight(isize::MIN)
                .with_mode(ModeJS::Normal),
        ));
    }
}
