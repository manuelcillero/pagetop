use crate::util;
use crate::core::app;
use crate::core::hook::HookItem;

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
use crate::db::MigrationItem;

pub trait BaseModule {
    fn single_name(&self) -> &'static str;
}

/// Los módulos deben implementar este *trait*.
pub trait ModuleTrait: BaseModule + Send + Sync {
    fn handler(&self) -> &'static str;

    fn name(&self) -> String {
        self.single_name().to_owned()
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn dependencies(&self) -> Vec<&'static dyn ModuleTrait> {
        vec![]
    }

    #[allow(unused_variables)]
    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {
    }

    fn actions(&self) -> Vec<HookItem> {
        vec![]
    }

    #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
    #[allow(unused_variables)]
    fn migrations(&self) -> Vec<MigrationItem> {
        vec![]
    }
}

impl<M: ?Sized + ModuleTrait> BaseModule for M {
    fn single_name(&self) -> &'static str {
        util::single_type_name::<Self>()
    }
}
