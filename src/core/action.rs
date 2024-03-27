mod definition;
pub use definition::{ActionBase, ActionBox, ActionKey, ActionTrait};

mod list;
use list::ActionsList;

mod all;
pub(crate) use all::add_action;
pub use all::dispatch_actions;

#[macro_export]
macro_rules! actions {
    () => {
        Vec::<ActionBox>::new()
    };
    ( $($action:expr),+ $(,)? ) => {{
        vec![$(Box::new($action),)+]
    }};
}
