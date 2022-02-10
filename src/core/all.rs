use crate::core::{server, state};

pub fn modules(cfg: &mut server::web::ServiceConfig) {
    for m in state::MODULES.read().unwrap().iter() {
        m.configure_module(cfg);
    }
}
