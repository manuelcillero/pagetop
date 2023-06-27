use pagetop::prelude::*;

use_handle!(MODULE_JQUERY);

use_locale!(LOCALE_JQUERY);

use_static!(jquery);

const JQUERY_PARAM: &str = "jquery.add";

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
    pub fn add_in(rcx: &mut RenderContext) {
        rcx.set_param::<bool>(JQUERY_PARAM, true);
    }

    pub fn remove_from(rcx: &mut RenderContext) {
        rcx.set_param::<bool>(JQUERY_PARAM, false);
    }
}

fn before_render_page(page: &mut Page) {
    if let Some(true) = page.context().get_param::<bool>(JQUERY_PARAM) {
        page.context().alter(ContextOp::AddJavaScript(
            JavaScript::located("/jquery/jquery.min.js?v=3.6.0")
                .with_weight(isize::MIN)
                .with_mode(ModeJS::Normal),
        ));
    }
}
