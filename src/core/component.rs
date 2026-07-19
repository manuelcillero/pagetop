//! API para construir nuevos componentes.
//!
//! Para cualquier `href`, `action` o redirección que deba preservar el idioma negociado (anexando a
//! la URL el parámetro de consulta `?lang=...`), PageTop no usa `String`/`&str` sueltos, sino que
//! usa [`Context::route()`] cuando ya se tiene el contexto de renderizado a mano, o [`Route`] para
//! campos de componente que se construyen una vez y se resuelven más tarde, en cada petición. La
//! documentación de [`Route`] explica el criterio completo para elegir entre ambos.

mod error;
pub use error::ComponentError;

mod definition;
pub use definition::{Component, ComponentClone, ComponentRender};

mod children;
pub use children::Children;
pub use children::{Child, ChildOp, Embed};

mod message;
pub use message::{MessageLevel, StatusMessage};

mod context;
pub use context::{AssetsOp, Context, ContextError, Contextual};

mod route;
pub use route::Route;

/// Alias de función (*callback*) para **determinar si un componente se renderiza o no**.
///
/// Puede usarse para permitir que una instancia concreta de un tipo de componente dado decida
/// dinámicamente durante el proceso de renderizado ([`Component::is_renderable()`]), si se
/// renderiza o no.
///
/// # Ejemplo
///
/// ```rust,no_run
/// # use pagetop::prelude::*;
/// #[derive(AutoDefault, Clone)]
/// struct SampleComponent {
///     renderable: Option<FnIsRenderable>,
/// }
///
/// #[async_trait]
/// impl Component for SampleComponent {
///     fn new() -> Self {
///         Self::default()
///     }
///
///     fn is_renderable(&self, cx: &Context) -> bool {
///         // Si hay callback, se usa; en caso contrario, se renderiza por defecto.
///         self.renderable.map_or(true, |f| f(cx))
///     }
///
///     async fn prepare(&self, _cx: &mut Context) -> Result<Markup, ComponentError> {
///         Ok(html! { "Visible component" })
///     }
/// }
///
/// impl SampleComponent {
///     /// Asigna una función que decidirá si el componente se renderiza o no.
///     #[builder_fn]
///     pub fn with_renderable(mut self, f: Option<FnIsRenderable>) -> Self {
///         self.renderable = f;
///         self
///     }
/// }
///
/// fn sample() {
///     let mut cx = Context::default().with_param("user_logged_in", true);
///
///     // Se instancia un componente que sólo se renderiza si `user_logged_in` es `true`.
///     let mut component = SampleComponent::new().with_renderable(Some(|cx: &Context| {
///         cx.param_or_default::<bool>("user_logged_in")
///     }));
///
///     // Aquí simplemente se comprueba que compila y se puede invocar.
///     let _markup = component.render(&mut cx);
/// }
/// ```
pub type FnIsRenderable = fn(cx: &Context) -> bool;
