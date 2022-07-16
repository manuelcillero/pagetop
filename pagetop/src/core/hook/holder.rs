use super::HookTrait;

use std::sync::{Arc, RwLock};

pub type HookAction = Box<dyn HookTrait>;

#[macro_export]
macro_rules! hook_action {
    ( $hook:ident => $f:ident $(, $weight:expr)? ) => {{
        Box::new($hook::new().with_hook($f)$(.with_weight($weight))?)
    }};
}

pub struct HooksHolder(Arc<RwLock<Vec<HookAction>>>);

impl HooksHolder {
    pub fn new() -> Self {
        HooksHolder(Arc::new(RwLock::new(Vec::new())))
    }

    pub fn new_with(hook: HookAction) -> Self {
        let mut container = HooksHolder::new();
        container.add(hook);
        container
    }

    pub fn add(&mut self, hook: HookAction) {
        let mut actions = self.0.write().unwrap();
        actions.push(hook);
        actions.sort_by_key(|a| a.weight());
    }

    pub fn iter_map<B, F>(&self, f: F) where Self: Sized, F: FnMut(&HookAction) -> B {
        let _: Vec<_> = self.0.read().unwrap().iter().map(f).collect();
    }
}
