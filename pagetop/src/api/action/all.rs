use crate::Lazy;
use crate::api::TypeId;
use super::{ActionItem, ActionsHolder};

use std::sync::RwLock;
use std::collections::HashMap;

// Registered actions.
static ACTIONS: Lazy<RwLock<HashMap<TypeId, ActionsHolder>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

pub fn add_action(action: ActionItem) {
    let mut hmap = ACTIONS.write().unwrap();
    let action_id = action.type_id();
    if let Some(actions) = hmap.get_mut(&action_id) {
        actions.add(action);
    } else {
        hmap.insert(action_id, ActionsHolder::new_with(action));
    }
}

pub fn run_actions<B, F>(action_id: TypeId, f: F)
where
    F: FnMut(&ActionItem) -> B
{
    if let Some(actions) = ACTIONS.read().unwrap().get(&action_id) {
        actions.iter_map(f)
    }
}
