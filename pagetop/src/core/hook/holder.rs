use super::HookTrait;

use std::sync::{Arc, RwLock};

pub type HookItem = Box<dyn HookTrait>;

#[macro_export]
macro_rules! hook_item {
    ( $hook:ident => $f:ident $(, $weight:expr)? ) => {{
        Box::new($hook::new().with_hook($f)$(.with_weight($weight))?)
    }};
}

pub struct HooksHolder(Arc<RwLock<Vec<HookItem>>>);

impl HooksHolder {
    pub fn new() -> Self {
        HooksHolder(Arc::new(RwLock::new(Vec::new())))
    }

    pub fn new_with(hook: HookItem) -> Self {
        let mut container = HooksHolder::new();
        container.add(hook);
        container
    }

    pub fn add(&mut self, hook: HookItem) {
        let mut actions = self.0.write().unwrap();
        actions.push(hook);
        actions.sort_by_key(|a| a.weight());
    }

    pub fn iter_map<B, F>(&self, f: F) where Self: Sized, F: FnMut(&HookItem) -> B {
        let _: Vec<_> = self.0.read().unwrap().iter().map(f).collect();
    }
}
