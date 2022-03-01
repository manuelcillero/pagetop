use crate::core::{all, server};

/// Los módulos deben implementar este *trait*.
pub trait Module: Send + Sync {
    fn name(&self) -> &'static str;

    fn fullname(&self) -> String;

    fn description(&self) -> String {
        "".to_string()
    }

    #[allow(unused_variables)]
    fn configure_module(&self, cfg: &mut server::web::ServiceConfig) {
    }
}

pub fn register_module(m: &'static (dyn Module + 'static)) {
    all::MODULES.write().unwrap().push(m);
}

pub fn find_module(name: &str) -> Option<&'static (dyn Module + 'static)> {
    let modules = all::MODULES.write().unwrap();
    match modules.iter().find(|t| t.name() == name) {
        Some(module) => Some(*module),
        _ => None,
    }
}
