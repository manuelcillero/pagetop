use crate::Lazy;
use super::{ActionItem, ActionsHolder};

use std::sync::RwLock;
use std::collections::HashMap;

// Registered actions.
static ACTIONS: Lazy<RwLock<HashMap<&str, ActionsHolder>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

pub fn add_action(action: ActionItem) {
    let mut hmap = ACTIONS.write().unwrap();
    let action_handler = action.handler();
    if let Some(actions) = hmap.get_mut(action_handler) {
        actions.add(action);
    } else {
        hmap.insert(action_handler, ActionsHolder::new_with(action));
    }
}

pub fn run_actions<B, F>(action_handler: &str, f: F) where F: FnMut(&ActionItem) -> B {
    if let Some(actions) = ACTIONS.read().unwrap().get(action_handler) {
        actions.iter_map(f)
    }
}
