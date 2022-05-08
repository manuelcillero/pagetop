use crate::Lazy;
use super::{HookItem, HooksHolder};

use std::sync::RwLock;
use std::collections::HashMap;

// Registered actions.
static ACTIONS: Lazy<RwLock<HashMap<&str, HooksHolder>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

pub fn add_hook(action: HookItem) {
    let mut hmap = ACTIONS.write().unwrap();
    let action_handler = action.handler();
    if let Some(actions) = hmap.get_mut(action_handler) {
        actions.add(action);
    } else {
        hmap.insert(action_handler, HooksHolder::new_with(action));
    }
}

pub fn run_hooks<B, F>(action_handler: &str, f: F) where F: FnMut(&HookItem) -> B {
    if let Some(actions) = ACTIONS.read().unwrap().get(action_handler) {
        actions.iter_map(f)
    }
}
