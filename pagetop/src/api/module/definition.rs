use crate::{app, util};
use crate::api::action::ActionItem;

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
use crate::db::MigrationItem;

/// Los módulos deben implementar este *trait*.
pub trait ModuleTrait: Send + Sync {
    fn handler(&self) -> &'static str;

    fn name(&self) -> String {
        util::single_type_name::<Self>().to_owned()
    }

    fn description(&self) -> Option<String> {
        None
    }

    #[allow(unused_variables)]
    fn configure_service(&self, cfg: &mut app::web::ServiceConfig) {
    }

    fn actions(&self) -> Vec<ActionItem> {
        vec![]
    }

    #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
    #[allow(unused_variables)]
    fn migrations(&self) -> Vec<MigrationItem> {
        vec![]
    }

    fn dependencies(&self) -> Vec<&'static dyn ModuleTrait> {
        vec![]
    }
}
