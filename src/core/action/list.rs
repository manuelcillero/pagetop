use crate::core::action::{ActionBox, ActionDispatcher};
use crate::core::AnyCast;
use crate::trace;
use crate::AutoDefault;

use parking_lot::RwLock;

#[derive(AutoDefault)]
pub struct ActionsList(RwLock<Vec<ActionBox>>);

impl ActionsList {
    pub fn new() -> Self {
        ActionsList::default()
    }

    pub fn add(&mut self, action: ActionBox) {
        let mut list = self.0.write();
        list.push(action);
        list.sort_by_key(|a| a.weight());
    }

    pub fn iter_map<A, B, F>(&self, mut f: F)
    where
        Self: Sized,
        A: ActionDispatcher,
        F: FnMut(&A) -> B,
    {
        let _: Vec<_> = self
            .0
            .read()
            .iter()
            .rev()
            .map(|a| {
                if let Some(action) = (**a).downcast_ref::<A>() {
                    f(action);
                } else {
                    trace::error!("Failed to downcast action of type {}", (**a).type_name());
                }
            })
            .collect();
    }
}
