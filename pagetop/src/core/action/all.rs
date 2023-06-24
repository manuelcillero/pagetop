use crate::core::action::{Action, ActionsList};
use crate::{Handle, LazyStatic};

use std::collections::HashMap;
use std::sync::RwLock;

// Registered actions.
static ACTIONS: LazyStatic<RwLock<HashMap<Handle, ActionsList>>> =
    LazyStatic::new(|| RwLock::new(HashMap::new()));

pub fn add_action(action: Action) {
    let mut actions = ACTIONS.write().unwrap();
    let action_handle = action.handle();
    if let Some(bundle) = actions.get_mut(&action_handle) {
        bundle.add(action);
    } else {
        actions.insert(action_handle, ActionsList::new_with(action));
    }
}

pub fn run_actions<B, F>(action_handle: Handle, f: F)
where
    F: FnMut(&Action) -> B,
{
    if let Some(bundle) = ACTIONS.read().unwrap().get(&action_handle) {
        bundle.iter_map(f)
    }
}
