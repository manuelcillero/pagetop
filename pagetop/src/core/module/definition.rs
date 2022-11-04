use crate::app;
use crate::core::hook::HookAction;
use crate::core::theme::ThemeStaticRef;
use crate::util::{single_type_name, Handler};

#[cfg(feature = "database")]
use crate::db::MigrationItem;

pub type ModuleStaticRef = &'static dyn ModuleTrait;

pub trait BaseModule {
    fn single_name(&self) -> &'static str;
}

/// Los módulos deben implementar este *trait*.
pub trait ModuleTrait: BaseModule + Send + Sync {
    fn handler(&self) -> Handler;

    fn name(&self) -> String {
        self.single_name().to_owned()
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![]
    }

    fn uninstall_modules(&self) -> Vec<ModuleStaticRef> {
        vec![]
    }

    fn themes(&self) -> Vec<ThemeStaticRef> {
        vec![]
    }

    fn actions(&self) -> Vec<HookAction> {
        vec![]
    }

    fn init(&self) {}

    #[cfg(feature = "database")]
    #[allow(unused_variables)]
    fn migrations(&self) -> Vec<MigrationItem> {
        vec![]
    }

    #[allow(unused_variables)]
    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {}
}

impl<M: ?Sized + ModuleTrait> BaseModule for M {
    fn single_name(&self) -> &'static str {
        single_type_name::<Self>()
    }
}
