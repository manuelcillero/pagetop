//! API para construir nuevos componentes.

mod definition;
pub use definition::{Component, ComponentRender};

mod children;
pub use children::Children;
pub use children::{Child, ChildOp};
pub use children::{Typed, TypedOp};

// **< HTML DOCUMENT CONTEXT >**********************************************************************

mod context;
pub use context::{Context, ContextOp, Contextual, ErrorParam};
pub type FnPathByContext = fn(cx: &Context) -> &str;
