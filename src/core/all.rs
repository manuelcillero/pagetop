use crate::core::{server, state};

include!(concat!(env!("OUT_DIR"), "/theme.rs"));

pub fn themes(cfg: &mut server::web::ServiceConfig) {
    cfg.service(actix_web_static_files::ResourceFiles::new(
        "/theme",
        assets()
    ));

    for t in state::THEMES.read().unwrap().iter() {
        t.configure_theme(cfg);
    }
}

pub fn modules(cfg: &mut server::web::ServiceConfig) {
    for m in state::MODULES.read().unwrap().iter() {
        m.configure_module(cfg);
    }
}
