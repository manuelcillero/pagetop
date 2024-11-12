use crate::core::action::ActionBox;
use crate::core::AnyBase;
use crate::locale::L10n;
use crate::{actions, service};

pub type PackageRef = &'static dyn PackageTrait;

/// Los paquetes deben implementar este *trait*.
pub trait PackageTrait: AnyBase + Send + Sync {
    fn name(&self) -> L10n {
        L10n::n(self.short_name())
    }

    fn description(&self) -> L10n {
        L10n::none()
    }

    fn dependencies(&self) -> Vec<PackageRef> {
        vec![]
    }

    fn drop_packages(&self) -> Vec<PackageRef> {
        vec![]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![]
    }

    fn init(&self) {}

    #[allow(unused_variables)]
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {}
}
