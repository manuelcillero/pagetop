use crate::db;
use crate::core::server;

/// Los módulos deben implementar este *trait*.
pub trait Module: Send + Sync {
    fn name(&self) -> &'static str;

    fn fullname(&self) -> String;

    fn description(&self) -> Option<String> {
        None
    }

    #[allow(unused_variables)]
    fn configure_module(&self, cfg: &mut server::web::ServiceConfig) {
    }

    #[allow(unused_variables)]
    fn migrations(&self, dbconn: &db::DbConn) -> Result<(), db::DbErr> {
        Ok(())
    }
}
