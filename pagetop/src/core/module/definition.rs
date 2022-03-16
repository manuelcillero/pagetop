use crate::core::server;

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
use crate::db;

/// Los módulos deben implementar este *trait*.
pub trait ModuleTrait: Send + Sync {
    fn name(&self) -> &'static str;

    fn fullname(&self) -> String;

    fn description(&self) -> Option<String> {
        None
    }

    #[allow(unused_variables)]
    fn configure_module(&self, cfg: &mut server::web::ServiceConfig) {
    }

    #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
    #[allow(unused_variables)]
    fn migrations(&self, dbconn: &db::DbConn) -> Result<(), db::DbErr> {
        Ok(())
    }
}
