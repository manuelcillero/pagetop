mod definition;
pub use definition::{action_ref, ActionBase, ActionTrait};

mod list;
pub use list::Action;
use list::ActionsList;

mod all;
pub(crate) use all::add_action;
pub use all::run_actions;

#[macro_export]
macro_rules! action {
    ( $action:ty => $f:ident $(, $weight:expr)? ) => {{
        Box::new(<$action>::new().with_action($f)$(.with_weight($weight))?)
    }};
}
