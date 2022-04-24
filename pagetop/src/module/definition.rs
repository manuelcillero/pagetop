use crate::app;

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
use crate::db;

/// Los módulos deben implementar este *trait*.
pub trait ModuleTrait: Send + Sync {
    fn name(&self) -> &'static str;

    fn fullname(&self) -> String;

    fn dependencies(&self) -> Vec<&'static dyn ModuleTrait> {
        vec![]
    }

    fn description(&self) -> Option<String> {
        None
    }

    #[allow(unused_variables)]
    fn configure_module(&self, cfg: &mut app::web::ServiceConfig) {
    }

    #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
    #[allow(unused_variables)]
    fn migrations(&self) -> Vec<Box<dyn db::MigrationTrait>> {
        vec![]
    }
}
