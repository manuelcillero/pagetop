use crate::core::action::{Action, ActionsList};
use crate::{Handle, LazyStatic};

use std::collections::HashMap;
use std::sync::RwLock;

type KeyHandles = (Handle, Option<Handle>);

// Registered actions.
static ACTIONS: LazyStatic<RwLock<HashMap<KeyHandles, ActionsList>>> =
    LazyStatic::new(|| RwLock::new(HashMap::new()));

pub fn add_action(action: Action) {
    let mut actions = ACTIONS.write().unwrap();
    let action_handle = (action.handle(), action.referer_handle());
    if let Some(list) = actions.get_mut(&action_handle) {
        list.add(action);
    } else {
        actions.insert(action_handle, ActionsList::with(action));
    }
}

pub fn run_actions<B, F>(action_handle: (Handle, Option<Handle>), f: F)
where
    F: FnMut(&Action) -> B,
{
    if let Some(list) = ACTIONS.read().unwrap().get(&action_handle) {
        list.iter_map(f)
    }
}
