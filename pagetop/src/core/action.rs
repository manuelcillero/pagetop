mod definition;
pub use definition::{action_ref, ActionTrait, AnyAction, ACTION_UNNAMED};

mod bundle;
pub use bundle::Action;
use bundle::ActionsBundle;

mod all;
pub(crate) use all::add_action;
pub use all::run_actions;
