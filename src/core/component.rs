//! API para construir nuevos componentes.

mod definition;
pub use definition::{Component, ComponentRender};

mod children;
pub use children::Children;
pub use children::{Child, ChildOp};
pub use children::{Typed, TypedOp};

mod context;
pub use context::{Context, ContextError, ContextOp, Contextual};
pub type FnPathByContext = fn(cx: &Context) -> &str;
