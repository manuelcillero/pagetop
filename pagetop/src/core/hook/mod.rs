mod definition;
pub use definition::{
    HookTrait,
    AnyHook,
    hook_ref,
};

mod holder;
pub use holder::HookItem;
use holder::HooksHolder;

mod all;
pub use all::run_hooks;
pub(crate) use all::add_hook;
