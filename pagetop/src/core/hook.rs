mod definition;
pub use definition::{action_ref, AnyHookAction, HookActionTrait};

mod holder;
use holder::ActionsHolder;
pub use holder::HookAction;

mod all;
pub(crate) use all::add_action;
pub use all::run_actions;
