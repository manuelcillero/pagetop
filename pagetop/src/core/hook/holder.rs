use super::HookTrait;

use std::sync::{Arc, RwLock};

pub type HookItem = Box<dyn HookTrait>;

#[macro_export]
macro_rules! hook_item {
    ( $action:ident => $f:ident $(, $weight:expr)? ) => {{
        Box::new($action::new().with_hook($f)$(.with_weight($weight))?)
    }};
}

pub struct HooksHolder(Arc<RwLock<Vec<HookItem>>>);

impl HooksHolder {
    pub fn new() -> Self {
        HooksHolder(Arc::new(RwLock::new(Vec::new())))
    }

    pub fn new_with(action: HookItem) -> Self {
        let mut container = HooksHolder::new();
        container.add(action);
        container
    }

    pub fn add(&mut self, action: HookItem) {
        let mut actions = self.0.write().unwrap();
        actions.push(action);
        actions.sort_by_key(|a| a.weight());
    }

    pub fn iter_map<B, F>(&self, f: F) where Self: Sized, F: FnMut(&HookItem) -> B {
        let _: Vec<_> = self.0.read().unwrap().iter().map(f).collect();
    }
}
