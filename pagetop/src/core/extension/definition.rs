use crate::core::action::ActionBox;
use crate::core::theme::ThemeRef;
use crate::core::AnyBase;
use crate::locale::L10n;
use crate::{actions, service};

pub type ExtensionRef = &'static dyn ExtensionTrait;

/// Las extensiones deben implementar este *trait*.
pub trait ExtensionTrait: AnyBase + Send + Sync {
    fn name(&self) -> L10n {
        L10n::n(self.short_name())
    }

    fn description(&self) -> L10n {
        L10n::default()
    }

    fn theme(&self) -> Option<ThemeRef> {
        None
    }

    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![]
    }

    fn drop_extensions(&self) -> Vec<ExtensionRef> {
        vec![]
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![]
    }

    fn init(&self) {}

    #[allow(unused_variables)]
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {}
}
