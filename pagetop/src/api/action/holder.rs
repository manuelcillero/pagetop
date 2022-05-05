use super::ActionTrait;

use std::sync::{Arc, RwLock};

pub type ActionItem = Box<dyn ActionTrait>;

#[macro_export]
macro_rules! action_item {
    ( $action:ident => $f:ident $(, $weight:expr)? ) => {{
        Box::new($action::new().with_action($f)$(.with_weight($weight))?)
    }};
}

pub struct ActionsHolder(Arc<RwLock<Vec<ActionItem>>>);

impl ActionsHolder {
    pub fn new() -> Self {
        ActionsHolder(Arc::new(RwLock::new(Vec::new())))
    }

    pub fn new_with(action: ActionItem) -> Self {
        let mut container = ActionsHolder::new();
        container.add(action);
        container
    }

    pub fn add(&mut self, action: ActionItem) {
        let mut actions = self.0.write().unwrap();
        actions.push(action);
        actions.sort_by_key(|a| a.weight());
    }

    pub fn iter_map<B, F>(&self, f: F)
    where
        Self: Sized,
        F: FnMut(&ActionItem) -> B,
    {
        let _: Vec<_> = self.0.read().unwrap().iter().map(f).collect();
    }
}
