use crate::core::action::ActionTrait;

use std::sync::{Arc, RwLock};

pub type Action = Box<dyn ActionTrait>;

#[macro_export]
macro_rules! action {
    ( $action:ident => $f:ident $(, $weight:expr)? ) => {{
        Box::new($action::new().with_action($f)$(.with_weight($weight))?)
    }};
}

pub struct ActionsBundle(Arc<RwLock<Vec<Action>>>);

impl ActionsBundle {
    pub fn new() -> Self {
        ActionsBundle(Arc::new(RwLock::new(Vec::new())))
    }

    pub fn new_with(action: Action) -> Self {
        let mut bundle = ActionsBundle::new();
        bundle.add(action);
        bundle
    }

    pub fn add(&mut self, action: Action) {
        let mut bundle = self.0.write().unwrap();
        bundle.push(action);
        bundle.sort_by_key(|a| a.weight());
    }

    pub fn iter_map<B, F>(&self, f: F)
    where
        Self: Sized,
        F: FnMut(&Action) -> B,
    {
        let _: Vec<_> = self.0.read().unwrap().iter().map(f).collect();
    }
}
