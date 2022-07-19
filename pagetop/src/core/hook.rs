mod definition;
pub use definition::{action_ref, AnyHook, HookTrait};

mod holder;
pub use holder::HookAction;
use holder::HooksHolder;

mod all;
pub(crate) use all::add_hook;
pub use all::run_actions;
