use crate::core::action::Action;
use crate::core::theme::ThemeRef;
use crate::core::AnyBase;
use crate::locale::L10n;
use crate::{actions, service};

#[cfg(feature = "database")]
use crate::{db::MigrationItem, migrations};

pub type PackageRef = &'static dyn PackageTrait;

/// Los paquetes deben implementar este *trait*.
pub trait PackageTrait: AnyBase + Send + Sync {
    fn name(&self) -> L10n {
        L10n::n(self.single_name())
    }

    fn description(&self) -> L10n {
        L10n::none()
    }

    fn theme(&self) -> Option<ThemeRef> {
        None
    }

    fn dependencies(&self) -> Vec<PackageRef> {
        vec![]
    }

    fn drop_packages(&self) -> Vec<PackageRef> {
        vec![]
    }

    fn actions(&self) -> Vec<Action> {
        actions![]
    }

    fn init(&self) {}

    #[cfg(feature = "database")]
    fn migrations(&self) -> Vec<MigrationItem> {
        migrations![]
    }

    #[allow(unused_variables)]
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {}
}
