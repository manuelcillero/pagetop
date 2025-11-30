use crate::core::action::ActionBox;
use crate::core::theme::ThemeRef;
use crate::core::AnyInfo;
use crate::locale::L10n;
use crate::{actions_boxed, service};

/// Interfaz común que debe implementar cualquier extensión de PageTop.
///
/// Este *trait* es fácil de implementar, basta con declarar una estructura sin campos para la
/// extensión y sobrescribir los métodos que sean necesarios. Por ejemplo:
///
/// ```rust
/// # use pagetop::prelude::*;
/// pub struct Blog;
///
/// impl Extension for Blog {
///     fn name(&self) -> L10n {
///         L10n::n("Blog")
///     }
///
///     fn description(&self) -> L10n {
///         L10n::n("Blog system")
///     }
/// }
/// ```
pub trait Extension: AnyInfo + Send + Sync {
    /// Nombre de la extensión como *texto localizado* legible para el usuario.
    ///
    /// Predeterminado por el [`short_name()`](AnyInfo::short_name) del tipo asociado a la
    /// extensión.
    fn name(&self) -> L10n {
        L10n::n(self.short_name())
    }

    /// Descripción corta de la extensión como *texto localizado* para paneles, listados, etc.
    ///
    /// Por defecto devuelve un valor vacío (`L10n::default()`).
    fn description(&self) -> L10n {
        L10n::default()
    }

    /// Devuelve una referencia a esta misma extensión cuando actúa como un tema.
    ///
    /// Para ello, la implementación concreta debe ser una extensión que también implemente
    /// [`Theme`](crate::core::theme::Theme). Por defecto, asume que la extensión no es un tema y
    /// devuelve `None`.
    ///
    /// # Ejemplo
    ///
    /// ```rust
    /// # use pagetop::prelude::*;
    /// pub struct MyTheme;
    ///
    /// impl Extension for MyTheme {
    ///     fn theme(&self) -> Option<ThemeRef> {
    ///         Some(&Self)
    ///     }
    /// }
    ///
    /// impl Theme for MyTheme {}
    /// ```
    fn theme(&self) -> Option<ThemeRef> {
        None
    }

    /// Otras extensiones que deben habilitarse **antes** de esta.
    ///
    /// PageTop resolverá automáticamente estas dependencias respetando el orden durante el arranque
    /// de la aplicación.
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![]
    }

    /// Devuelve la lista de acciones que la extensión registra.
    ///
    /// Estas [acciones](crate::core::action) se despachan por orden de registro o por
    /// [peso](crate::Weight) (ver [`actions_boxed!`](crate::actions_boxed)), permitiendo
    /// personalizar el comportamiento de la aplicación en puntos específicos.
    fn actions(&self) -> Vec<ActionBox> {
        actions_boxed![]
    }

    /// Inicializa la extensión durante la fase de arranque de la aplicación.
    ///
    /// Se llama una sola vez, después de que todas las dependencias se han inicializado y antes de
    /// aceptar cualquier petición HTTP.
    fn initialize(&self) {}

    /// Configura los servicios web de la extensión, como rutas, *middleware*, acceso a ficheros
    /// estáticos, etc., usando [`ServiceConfig`](crate::service::web::ServiceConfig).
    ///
    /// # Ejemplo
    ///
    /// ```rust,ignore
    /// # use pagetop::prelude::*;
    /// pub struct ExtensionSample;
    ///
    /// impl Extension for ExtensionSample {
    ///     fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
    ///         scfg.route("/sample", web::get().to(route_sample));
    ///     }
    /// }
    /// ```
    #[allow(unused_variables)]
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {}

    /// Permite declarar extensiones destinadas a deshabilitar o desinstalar recursos de otras
    /// extensiones asociadas a versiones anteriores de la aplicación.
    ///
    /// Actualmente PageTop no utiliza este método, pero se reserva como *placeholder* para futuras
    /// implementaciones.
    fn drop_extensions(&self) -> Vec<ExtensionRef> {
        vec![]
    }
}

/// Representa una referencia a una extensión.
pub type ExtensionRef = &'static dyn Extension;
