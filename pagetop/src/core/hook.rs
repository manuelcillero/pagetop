mod definition;
pub use definition::{
    AnyHook,
    HookTrait,
    action_ref,
};

mod holder;
pub use holder::HookAction;
use holder::HooksHolder;

mod all;
pub use all::run_actions;
pub(crate) use all::add_hook;
