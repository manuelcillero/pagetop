//! API para construir nuevos componentes.

mod definition;
pub use definition::{Component, ComponentRender};

mod children;
pub use children::Children;
pub use children::{Child, ChildOp};
pub use children::{Typed, TypedOp};

mod context;
pub use context::{Context, ContextError, ContextOp, Contextual};

/// Alias de función (*callback*) para **resolver una URL** según el contexto de renderizado.
///
/// Se usa para generar enlaces dinámicos en función del contexto (petición, idioma, etc.). Debe
/// devolver una referencia válida durante el renderizado.
pub type FnPathByContext = fn(cx: &Context) -> &str;
