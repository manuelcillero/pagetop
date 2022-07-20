use super::HookActionTrait;

use std::sync::{Arc, RwLock};

pub type HookAction = Box<dyn HookActionTrait>;

#[macro_export]
macro_rules! hook_action {
    ( $hook:ident => $f:ident $(, $weight:expr)? ) => {{
        Box::new($hook::new().with_hook($f)$(.with_weight($weight))?)
    }};
}

pub struct ActionsHolder(Arc<RwLock<Vec<HookAction>>>);

impl ActionsHolder {
    pub fn new() -> Self {
        ActionsHolder(Arc::new(RwLock::new(Vec::new())))
    }

    pub fn new_with(action: HookAction) -> Self {
        let mut holder = ActionsHolder::new();
        holder.add(action);
        holder
    }

    pub fn add(&mut self, action: HookAction) {
        let mut holder = self.0.write().unwrap();
        holder.push(action);
        holder.sort_by_key(|a| a.weight());
    }

    pub fn iter_map<B, F>(&self, f: F)
    where
        Self: Sized,
        F: FnMut(&HookAction) -> B,
    {
        let _: Vec<_> = self.0.read().unwrap().iter().map(f).collect();
    }
}
