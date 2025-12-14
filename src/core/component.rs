//! API para construir nuevos componentes.

use crate::html::RoutePath;

mod definition;
pub use definition::{Component, ComponentRender};

mod children;
pub use children::Children;
pub use children::{Child, ChildOp};
pub use children::{Typed, TypedOp};

mod context;
pub use context::{Context, ContextError, ContextOp, Contextual};

/// Alias de función (*callback*) para **determinar si un componente se renderiza o no**.
///
/// Puede usarse para permitir que una instancia concreta de un tipo de componente dado decida
/// dinámicamente durante el proceso de renderizado ([`Component::is_renderable()`]), si se
/// renderiza o no.
///
/// # Ejemplo
///
/// ```rust
/// # use pagetop::prelude::*;
/// #[derive(AutoDefault)]
/// struct SampleComponent {
///     renderable: Option<FnIsRenderable>,
/// }
///
/// impl Component for SampleComponent {
///     fn new() -> Self {
///         Self::default()
///     }
///
///     fn is_renderable(&self, cx: &mut Context) -> bool {
///         // Si hay callback, se usa; en caso contrario, se renderiza por defecto.
///         self.renderable.map_or(true, |f| f(cx))
///     }
///
///     fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
///         PrepareMarkup::Escaped("Visible component".into())
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
///         cx.param::<bool>("user_logged_in").copied().unwrap_or(false)
///     }));
///
///     // Aquí simplemente se comprueba que compila y se puede invocar.
///     let _markup = component.render(&mut cx);
/// }
/// ```
pub type FnIsRenderable = fn(cx: &Context) -> bool;

/// Alias de función (*callback*) para **resolver una ruta URL** según el contexto de renderizado.
///
/// Se usa para generar enlaces dinámicos en función del contexto (petición, idioma, parámetros,
/// etc.). Devuelve una [`RoutePath`], que representa un *path* base junto con una lista opcional de
/// parámetros de consulta.
///
/// El caso más común es construir rutas relativas dependientes del contexto, normalmente usando
/// [`Context::route`](crate::core::component::Context::route):
///
/// ```rust
/// # use pagetop::prelude::*;
/// # let relative_route: FnPathByContext =
/// |cx| cx.route("/path/to/page")
/// # ;
/// ```
///
/// También es posible usar rutas estáticas sin asignaciones adicionales:
///
/// ```rust
/// # use pagetop::prelude::*;
/// # let external_route: FnPathByContext =
/// |_| "https://www.example.com".into()
/// # ;
/// ```
///
/// O componer rutas dinámicas en tiempo de ejecución:
///
/// ```rust
/// # use pagetop::prelude::*;
/// # let dynamic_route: FnPathByContext =
/// |cx| RoutePath::new("/user").with_param("id", cx.param::<u64>("user_id").unwrap().to_string())
/// # ;
/// ```
///
/// Los componentes que acepten un [`FnPathByContext`] invocarán esta función durante el renderizado
/// para obtener la URL final que se asignará al atributo HTML correspondiente.
pub type FnPathByContext = fn(cx: &Context) -> RoutePath;
