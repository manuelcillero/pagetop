use crate::core::action::{ActionBox, ActionKey, ActionTrait, ActionsList};
use crate::LazyStatic;

use std::collections::HashMap;
use std::sync::RwLock;

// Registered actions.
static ACTIONS: LazyStatic<RwLock<HashMap<ActionKey, ActionsList>>> =
    LazyStatic::new(|| RwLock::new(HashMap::new()));

pub fn add_action(action: ActionBox) {
    let key = action.key();
    let mut actions = ACTIONS.write().unwrap();
    if let Some(list) = actions.get_mut(&key) {
        list.add(action);
    } else {
        let mut list = ActionsList::new();
        list.add(action);
        actions.insert(key, list);
    }
}

pub fn dispatch_actions<A, B, F>(key: ActionKey, f: F)
where
    A: ActionTrait,
    F: FnMut(&A) -> B,
{
    if let Some(list) = ACTIONS.read().unwrap().get(&key) {
        list.iter_map(f)
    }
}
