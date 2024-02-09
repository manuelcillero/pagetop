use crate::core::action::ActionTrait;
use crate::SmartDefault;

use std::sync::{Arc, RwLock};

pub type Action = Box<dyn ActionTrait>;

#[derive(SmartDefault)]
pub struct ActionsList(Arc<RwLock<Vec<Action>>>);

impl ActionsList {
    pub fn new(action: Action) -> Self {
        let mut list = ActionsList::default();
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

#[macro_export]
macro_rules! actions {
    () => {
        Vec::<Action>::new()
    };
    ( $($action:expr),+ $(,)? ) => {{
        let mut v = Vec::<Action>::new();
        $(
            v.push(Box::new($action));
        )*
        v
    }};
}
