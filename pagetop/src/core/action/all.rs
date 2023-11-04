use crate::core::action::{Action, ActionsList};
use crate::{Handle, LazyStatic};

use std::collections::HashMap;
use std::sync::RwLock;

pub type KeyAction = (Handle, Option<Handle>, Option<String>);

// Registered actions.
static ACTIONS: LazyStatic<RwLock<HashMap<KeyAction, ActionsList>>> =
    LazyStatic::new(|| RwLock::new(HashMap::new()));

pub fn add_action(action: Action) {
    let mut actions = ACTIONS.write().unwrap();
    let key_action = (
        action.handle(),
        action.referer_handle(),
        action.referer_id(),
    );
    if let Some(list) = actions.get_mut(&key_action) {
        list.add(action);
    } else {
        actions.insert(key_action, ActionsList::with(action));
    }
}

pub fn dispatch_actions<B, F>(key_action: KeyAction, f: F)
where
    F: FnMut(&Action) -> B,
{
    if let Some(list) = ACTIONS.read().unwrap().get(&key_action) {
        list.iter_map(f)
    }
}
