use crate::core::server;

/// Los módulos deben implementar este *trait*.
pub trait Module: Send + Sync {
    fn name(&self) -> String;

    fn description(&self) -> String {
        "".to_string()
    }

    #[allow(unused_variables)]
    fn configure_module(&self, cfg: &mut server::web::ServiceConfig) {
    }
}
