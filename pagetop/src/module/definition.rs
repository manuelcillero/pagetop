#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
use crate::db;
use crate::app;

use std::any::type_name;

/// Los módulos deben implementar este *trait*.
pub trait ModuleTrait: Send + Sync {
    fn name(&self) -> &'static str {
        let name = type_name::<Self>();
        match name.rfind("::") {
            Some(position) => &name[(position + 2)..],
            None => name
        }
    }

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
    fn migrations(&self) -> Vec<Box<dyn db::migration::MigrationTrait>> {
        vec![]
    }
}
