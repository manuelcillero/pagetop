use crate::core::action::ActionTrait;

use std::sync::{Arc, RwLock};

pub type Action = Box<dyn ActionTrait>;

pub struct ActionsList(Arc<RwLock<Vec<Action>>>);

impl ActionsList {
    pub fn new() -> Self {
        ActionsList(Arc::new(RwLock::new(Vec::new())))
    }

    pub fn with(action: Action) -> Self {
        let mut list = ActionsList::new();
        list.add(action);
        list
    }

    pub fn add(&mut self, action: Action) {
        let mut list = self.0.write().unwrap();
        list.push(action);
        list.sort_by_key(|a| a.weight());
    }

    pub fn iter_map<B, F>(&self, f: F)
    where
        Self: Sized,
        F: FnMut(&Action) -> B,
    {
        let _: Vec<_> = self.0.read().unwrap().iter().map(f).collect();
    }
}
