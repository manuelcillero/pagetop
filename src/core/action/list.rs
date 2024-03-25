use crate::core::action::ActionTrait;
use crate::core::AnyTo;
use crate::trace;
use crate::AutoDefault;

use std::sync::{Arc, RwLock};

pub type Action = Box<dyn ActionTrait>;

#[derive(AutoDefault)]
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

    pub fn iter_map<A, B, F>(&self, mut f: F)
    where
        Self: Sized,
        A: ActionTrait,
        F: FnMut(&A) -> B,
    {
        let _: Vec<_> = self
            .0
            .read()
            .unwrap()
            .iter()
            .map(|a| {
                if let Some(action) = (&**a).downcast_ref::<A>() {
                    f(action);
                } else {
                    trace::error!("Failed to downcast action of type {}", (&**a).type_name());
                }
            })
            .collect();
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
