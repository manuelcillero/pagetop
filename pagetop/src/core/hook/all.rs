use super::{ActionsHolder, HookAction};
use crate::{Handle, LazyStatic};

use std::collections::HashMap;
use std::sync::RwLock;

// Registered actions.
static ACTIONS: LazyStatic<RwLock<HashMap<Handle, ActionsHolder>>> =
    LazyStatic::new(|| RwLock::new(HashMap::new()));

pub fn add_action(action: HookAction) {
    let mut actions = ACTIONS.write().unwrap();
    let action_handle = action.handle();
    if let Some(holder) = actions.get_mut(&action_handle) {
        holder.add(action);
    } else {
        actions.insert(action_handle, ActionsHolder::new_with(action));
    }
}

pub fn run_actions<B, F>(action_handle: Handle, f: F)
where
    F: FnMut(&HookAction) -> B,
{
    if let Some(holder) = ACTIONS.read().unwrap().get(&action_handle) {
        holder.iter_map(f)
    }
}
