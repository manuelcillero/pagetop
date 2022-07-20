mod definition;
pub use definition::{action_ref, AnyHookAction, HookActionTrait};

mod holder;
pub use holder::HookAction;
use holder::ActionsHolder;

mod all;
pub(crate) use all::add_action;
pub use all::run_actions;
