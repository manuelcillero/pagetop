use crate::prelude::*;

/// Componente básico para renderizar dinámicamente código HTML recibiendo el contexto.
///
/// Este componente permite generar contenido HTML arbitrario, usando la macro `html!` y accediendo
/// opcionalmente al contexto de renderizado.
///
/// # Ejemplo
///
/// ```rust
/// use pagetop::prelude::*;
///
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
/// use pagetop::prelude::*;
///
/// let component = Html::with(|cx| {
///     let user = cx.get_param::<String>("username").unwrap_or(String::from("visitor"));
///     html! {
///         h1 { "Hello, " (user) }
///     }
/// });
/// ```
pub struct Html(Box<dyn Fn(&mut Context) -> Markup + Send + Sync>);

impl Default for Html {
    fn default() -> Self {
        Html::with(|_| html! {})
    }
}
impl Component for Html {
    fn new() -> Self {
        Html::default()
    }

    fn prepare_component(&self, cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(self.html(cx))
    }
}

impl Html {
    // Html BUILDER ********************************************************************************

    /// Crea una instancia que generará el `Markup`, con acceso opcional al contexto.
    ///
    /// El método [`prepare_component()`](crate::core::component::Component::prepare_component)
    /// delega el renderizado en la función proporcionada, que recibe una referencia mutable al
    /// contexto de renderizado ([`Context`]).
    pub fn with<F>(f: F) -> Self
    where
        F: Fn(&mut Context) -> Markup + Send + Sync + 'static,
    {
        Html(Box::new(f))
    }

    /// Sustituye la función que genera el `Markup`.
    ///
    /// Permite a otras extensiones modificar la función de renderizado que se ejecutará cuando
    /// [`prepare_component()`](crate::core::component::Component::prepare_component) invoque esta
    /// instancia. La nueva función también recibe una referencia al contexto ([`Context`]).
    #[builder_fn]
    pub fn with_fn<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Context) -> Markup + Send + Sync + 'static,
    {
        self.0 = Box::new(f);
        self
    }

    // Html GETTERS ********************************************************************************

    /// Aplica la función interna de renderizado con el [`Context`] proporcionado.
    ///
    /// Normalmente no se invoca manualmente, ya que el proceso de renderizado de los componentes lo
    /// invoca automáticamente durante la construcción de la página. Puede usarse, no obstante, para
    /// sobrescribir [`prepare_component()`](crate::core::component::Component::prepare_component)
    /// y alterar el comportamiento del componente.
    pub fn html(&self, cx: &mut Context) -> Markup {
        (self.0)(cx)
    }
}
