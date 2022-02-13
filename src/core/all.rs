use crate::core::{server, state};

pub fn themes(cfg: &mut server::web::ServiceConfig) {
    for t in state::THEMES.read().unwrap().iter() {
        t.configure_theme(cfg);
    }
}

pub fn modules(cfg: &mut server::web::ServiceConfig) {
    for m in state::MODULES.read().unwrap().iter() {
        m.configure_module(cfg);
    }
}
