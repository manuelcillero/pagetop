use crate::Lazy;
use super::{ActionItem, ActionsHolder, ActionTrait};

use std::sync::RwLock;
use std::collections::HashMap;

// Registered actions.
static ACTIONS: Lazy<RwLock<HashMap<&str, ActionsHolder>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

pub fn register_action(action: impl ActionTrait) {
    let mut hmap = ACTIONS.write().unwrap();
    let action_name = action.type_name();
    if let Some(actions) = hmap.get_mut(action_name) {
        actions.add(action);
    } else {
        hmap.insert(action_name, ActionsHolder::new_with(action));
    }
}

pub fn run_actions<B, F>(type_name: &'static str, f: F) where F: FnMut(&ActionItem) -> B {
    let hmap = ACTIONS.read().unwrap();
    if let Some(actions) = hmap.get(type_name) {
        actions.iter_map(f)
    }
}
