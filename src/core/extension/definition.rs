use crate::core::action::ActionBox;
use crate::core::theme::ThemeRef;
use crate::core::AnyInfo;
use crate::locale::L10n;
use crate::{actions_boxed, service};

/// Representa una referencia a una extensión.
///
/// Las extensiones se definen como instancias estáticas globales para poder acceder a ellas desde
/// cualquier hilo de la ejecución sin necesidad de sincronización adicional.
pub type ExtensionRef = &'static dyn Extension;

/// Interfaz común que debe implementar cualquier extensión de PageTop.
///
/// Este *trait* es fácil de implementar, basta con declarar una estructura de tamaño cero para la
/// extensión y sobreescribir los métodos que sea necesario.
///
/// ```rust
/// # use pagetop::prelude::*;
/// pub struct Blog;
///
/// impl Extension for Blog {
///     fn name(&self) -> L10n { L10n::n("Blog") }
///     fn description(&self) -> L10n { L10n::n("Sistema de blogs") }
/// }
/// ```
pub trait Extension: AnyInfo + Send + Sync {
    /// Nombre localizado de la extensión legible para el usuario.
    ///
    /// Predeterminado por el [`short_name()`](AnyInfo::short_name) del tipo asociado a la
    /// extensión.
    fn name(&self) -> L10n {
        L10n::n(self.short_name())
    }

    /// Descripción corta localizada de la extensión para paneles, listados, etc.
    fn description(&self) -> L10n {
        L10n::default()
    }

    /// Devuelve una referencia a esta misma extensión cuando se trata de un tema.
    ///
    /// Para ello, debe implementar [`Extension`] y también [`Theme`](crate::core::theme::Theme). Si
    /// la extensión no es un tema, este método devuelve `None` por defecto.
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
    /// PageTop las resolverá automáticamente respetando el orden durante el arranque de la
    /// aplicación.
    fn dependencies(&self) -> Vec<ExtensionRef> {
        vec![]
    }

    /// Devuelve la lista de acciones que la extensión va a registrar.
    ///
    /// Estas [acciones](crate::core::action) se despachan por orden de registro o por
    /// [peso](crate::Weight), permitiendo personalizar el comportamiento de la aplicación en puntos
    /// específicos.
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

    /// Permite crear extensiones para deshabilitar y desinstalar recursos de otras de versiones
    /// anteriores de la aplicación.
    ///
    /// Actualmente no se usa, pero se deja como *placeholder* para futuras implementaciones.
    fn drop_extensions(&self) -> Vec<ExtensionRef> {
        vec![]
    }
}
