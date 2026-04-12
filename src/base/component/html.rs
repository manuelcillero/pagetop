use crate::prelude::*;

use std::fmt;
use std::sync::Arc;

/// Componente básico que renderiza dinámicamente código HTML según el contexto.
///
/// Este componente permite generar contenido HTML arbitrario, usando la macro `html!` y accediendo
/// opcionalmente al contexto de renderizado.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// let component = Html::with(|_| {
///     html! {
///         div class="example" {
///             p { "Hello from PageTop." }
///         }
///     }
/// });
/// ```
///
/// Para renderizar contenido que dependa del contexto, se puede acceder a él dentro del *closure*:
///
/// ```rust
/// # use pagetop::prelude::*;
/// let component = Html::with(|cx| {
///     let user = cx.param_or("username", "visitor".to_string());
///     html! {
///         h1 { "Hello, " (user) }
///     }
/// });
/// ```
#[derive(Clone)]
pub struct Html(Arc<dyn Fn(&mut Context) -> Markup + Send + Sync>);

impl fmt::Debug for Html {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Html")
            .field(&"Fn(&mut Context) -> Markup")
            .finish()
    }
}

impl Default for Html {
    fn default() -> Self {
        Self::with(|_| html! {})
    }
}

impl Component for Html {
    fn new() -> Self {
        Self::default()
    }

    fn prepare(&self, cx: &mut Context) -> Result<Markup, ComponentError> {
        Ok(self.html(cx))
    }
}

impl Html {
    // **< Html BUILDER >***************************************************************************

    /// Crea una instancia que generará el `Markup`, con acceso opcional al contexto.
    ///
    /// El método [`Self::prepare()`] delega el renderizado a la función que aquí se proporciona,
    /// con una llamada que requiere una referencia mutable al [`Context`].
    pub fn with<F>(f: F) -> Self
    where
        F: Fn(&mut Context) -> Markup + Send + Sync + 'static,
    {
        Html(Arc::new(f))
    }

    /// Sustituye la función que genera el `Markup`.
    ///
    /// Permite a otras extensiones modificar la función de renderizado que se ejecutará cuando
    /// [`Self::prepare()`] invoque esta instancia. La nueva función también recibe una referencia
    /// mutable al [`Context`].
    #[builder_fn]
    pub fn with_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Context) -> Markup + Send + Sync + 'static,
    {
        self.0 = Arc::new(f);
        self
    }

    // **< Html GETTERS >***************************************************************************

    /// Aplica la función interna de renderizado con el [`Context`] proporcionado.
    ///
    /// Normalmente no se invoca manualmente, ya que el proceso de renderizado de los componentes lo
    /// invoca automáticamente durante la construcción de la página. Puede usarse, no obstante, para
    /// sobrescribir [`prepare()`](crate::core::component::Component::prepare) y alterar el
    /// comportamiento del componente.
    pub fn html(&self, cx: &mut Context) -> Markup {
        (self.0)(cx)
    }
}
