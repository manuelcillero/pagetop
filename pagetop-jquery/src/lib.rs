use pagetop::prelude::*;

pub_handle!(MODULE_JQUERY);

include!(concat!(env!("OUT_DIR"), "/jquery.rs"));

const JQUERY_PARAM: &str = "jquery.add";
const JQUERY_SOURCE: &str = "/jquery/3.6.0/jquery.min.js";

pub struct JQuery;

impl ModuleTrait for JQuery {
    fn handle(&self) -> Handle {
        MODULE_JQUERY
    }

    fn configure_service(&self, cfg: &mut server::web::ServiceConfig) {
        serve_static_files!(cfg, "/jquery", bundle_jquery);
    }
}

impl JQuery {
    pub fn add_jquery(rcx: &mut RenderContext) {
        match rcx.get_param::<bool>(JQUERY_PARAM) {
            Some(true) => {}
            _ => {
                rcx.alter(ContextOp::AddJavaScript(
                    JavaScript::located(JQUERY_SOURCE)
                        .with_weight(isize::MIN)
                        .with_mode(ModeJS::Normal),
                ));
                rcx.set_param::<bool>(JQUERY_PARAM, true);
            }
        }
    }

    pub fn remove_jquery(rcx: &mut RenderContext) {
        if let Some(true) = rcx.get_param::<bool>(JQUERY_PARAM) {
            rcx.alter(ContextOp::RemoveJavaScript(JQUERY_SOURCE));
            rcx.set_param::<bool>(JQUERY_PARAM, false);
        }
    }
}
