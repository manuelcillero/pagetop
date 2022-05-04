use super::ActionTrait;

use std::sync::{Arc, RwLock};

pub type ActionItem = Box<dyn ActionTrait>;

#[derive(Clone)]
pub struct ActionsHolder(Arc<RwLock<Vec<ActionItem>>>);

impl ActionsHolder {
    pub fn new() -> Self {
        ActionsHolder(Arc::new(RwLock::new(Vec::new())))
    }

    pub fn new_with(action: impl ActionTrait) -> Self {
        let mut container = ActionsHolder::new();
        container.add(action);
        container
    }

    pub fn add(&mut self, action: impl ActionTrait) {
        let mut actions = self.0.write().unwrap();
        actions.push(Box::new(action));
        actions.sort_by_key(|a| a.weight());
    }

    pub fn iter_map<B, F>(&self, f: F) where Self: Sized, F: FnMut(&ActionItem) -> B {
        let _ = self.0.read().unwrap().iter().map(f);
    }
}
