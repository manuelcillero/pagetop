use crate::core::server;

/// Modules must implement this trait.
pub trait Module: Send + Sync {
    fn name(&self) -> String;

    fn description(&self) -> String {
        "".to_string()
    }

    #[allow(unused_variables)]
    fn configure_module(&self, cfg: &mut server::web::ServiceConfig) {
    }
}
