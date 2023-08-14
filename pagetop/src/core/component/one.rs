use crate::core::component::{ComponentTrait, Context};
use crate::html::Markup;
use crate::{Handle, Weight};

use std::sync::{Arc, RwLock};

#[derive(Default)]
pub struct ComponentOne<T: ComponentTrait + Default>(Arc<RwLock<T>>);

impl<T: ComponentTrait + Default> Clone for ComponentOne<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ComponentTrait + Default> ComponentOne<T> {
    pub fn new() -> Self {
        ComponentOne::<T>::default()
    }

    pub fn with(component: T) -> Self {
        ComponentOne(Arc::new(RwLock::new(component)))
    }

    pub fn set(&mut self, component: T) {
        self.0 = Arc::new(RwLock::new(component));
    }

    pub(crate) fn handle(&self) -> Handle {
        self.0.read().unwrap().handle()
    }

    pub(crate) fn id(&self) -> Option<String> {
        self.0.read().unwrap().id()
    }

    pub(crate) fn weight(&self) -> Weight {
        self.0.read().unwrap().weight()
    }

    // ComponentOne PREPARE.

    pub fn prepare(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().prepare(cx)
    }
}
