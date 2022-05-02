use crate::app;
use crate::util::partial_type_name;

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
use crate::db;

pub trait BaseModule {
    fn type_name(&self) -> &'static str;

    fn single_name(&self) -> &'static str;

    fn qualified_name(&self, last: usize) -> &'static str;
}

/// Los módulos deben implementar este *trait*.
pub trait ModuleTrait: BaseModule + Send + Sync {
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
    fn configure_module(&self, cfg: &mut app::web::ServiceConfig) {
    }

    #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
    #[allow(unused_variables)]
    fn migrations(&self) -> Vec<Box<dyn db::MigrationTrait>> {
        vec![]
    }
}

impl<M: ?Sized + ModuleTrait> BaseModule for M {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn single_name(&self) -> &'static str {
        partial_type_name(std::any::type_name::<Self>(), 1)
    }

    fn qualified_name(&self, last: usize) -> &'static str {
        partial_type_name(std::any::type_name::<Self>(), last)
    }
}
