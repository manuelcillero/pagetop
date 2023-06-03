use super::ThemeStaticRef;

use crate::core::component::L10n;
use crate::core::hook::HookAction;
use crate::util::single_type_name;
use crate::{server, Handle};

#[cfg(feature = "database")]
use crate::db::MigrationItem;

pub type ModuleStaticRef = &'static dyn ModuleTrait;

pub trait BaseModule {
    fn single_name(&self) -> &'static str;
}

/// Los módulos deben implementar este *trait*.
pub trait ModuleTrait: BaseModule + Send + Sync {
    fn handle(&self) -> Handle;

    fn name(&self) -> L10n {
        L10n::text(self.single_name())
    }

    fn description(&self) -> L10n {
        L10n::default()
    }

    fn theme(&self) -> Option<ThemeStaticRef> {
        None
    }

    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![]
    }

    fn drop_modules(&self) -> Vec<ModuleStaticRef> {
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
    fn configure_service(&self, cfg: &mut server::web::ServiceConfig) {}
}

impl<M: ?Sized + ModuleTrait> BaseModule for M {
    fn single_name(&self) -> &'static str {
        single_type_name::<Self>()
    }
}
