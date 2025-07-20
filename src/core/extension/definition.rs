use crate::core::theme::ThemeRef;
use crate::core::AnyInfo;
use crate::locale::L10n;
use crate::service;

/// Representa una referencia a una extensión.
///
/// Las extensiones se definen como instancias estáticas globales para poder acceder a ellas desde
/// cualquier hilo de la ejecución sin necesidad de sincronización adicional.
pub type ExtensionRef = &'static dyn ExtensionTrait;

/// Interfaz común que debe implementar cualquier extensión de `PageTop`.
///
/// Este *trait* es fácil de implementar, basta con declarar una estructura de tamaño cero para la
/// extensión y sobreescribir los métodos que sea necesario.
///
/// ```rust
/// use pagetop::prelude::*;
///
/// pub struct Blog;
///
/// impl ExtensionTrait for Blog {
///     fn name(&self) -> L10n { L10n::n("Blog") }
///     fn description(&self) -> L10n { L10n::n("Sistema de blogs") }
/// }
/// ```
pub trait ExtensionTrait: AnyInfo + Send + Sync {
    /// Nombre legible para el usuario.
    ///
    /// Predeterminado por el [`short_name`](AnyInfo::short_name) del tipo asociado a la extensión.
    fn name(&self) -> L10n {
        L10n::n(self.short_name())
    }

    /// Descripción corta para paneles, listados, etc.
    fn description(&self) -> L10n {
        L10n::default()
    }

    /// Los temas son extensiones que implementan [`ExtensionTrait`] y también
    /// [`ThemeTrait`](crate::core::theme::ThemeTrait).
    ///
    /// Si la extensión no es un tema, este método devuelve `None` por defecto.
    ///
    /// En caso contrario, este método debe implementarse para devolver una referencia de sí mismo
    /// como tema. Por ejemplo:
    ///
    /// ```rust
    /// use pagetop::prelude::*;
    ///
    /// pub struct MyTheme;
    ///
    /// impl ExtensionTrait for MyTheme {
    ///     fn theme(&self) -> Option<ThemeRef> {
    ///         Some(&Self)
    ///     }
    /// }
    ///
    /// impl ThemeTrait for MyTheme {}
    /// ```
    fn theme(&self) -> Option<ThemeRef> {
        None
    }

    /// Otras extensiones que deben habilitarse **antes** de esta.
    ///
    /// `PageTop` las resolverá automáticamente respetando el orden durante el arranque de la
    /// aplicación.
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![]
    }

    /// Inicializa la extensión durante la lógica de arranque de la aplicación.
    ///
    /// Se llama una sola vez, después de que todas las dependencias se han inicializado y antes de
    /// aceptar cualquier petición HTTP.
    fn initialize(&self) {}

    /// Configura los servicios web de la extensión, como rutas, *middleware*, acceso a ficheros
    /// estáticos, etc., usando [`ServiceConfig`](crate::service::web::ServiceConfig).
    ///
    /// ```rust,ignore
    /// use pagetop::prelude::*;
    ///
    /// pub struct ExtensionSample;
    ///
    /// impl ExtensionTrait for ExtensionSample {
    ///     fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
    ///         scfg.route("/sample", web::get().to(route_sample));
    ///     }
    /// }
    /// ```
    #[allow(unused_variables)]
    fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {}

    /// Permite crear extensiones para deshabilitar y desinstalar los recursos de otras extensiones
    /// utilizadas en versiones anteriores de la aplicación.
    ///
    /// Actualmente no se usa, pero se deja como *placeholder* para futuras implementaciones.
    fn drop_extensions(&self) -> Vec<ExtensionRef> {
        vec![]
    }
}
