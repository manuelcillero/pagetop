mod definition;
pub use definition::{action_ref, ActionTrait};

mod list;
pub use list::Action;
use list::ActionsList;

mod all;
pub(crate) use all::add_action;
pub use all::{dispatch_actions, KeyAction};
