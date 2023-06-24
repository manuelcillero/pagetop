use crate::core::component::{ComponentTrait, RenderContext};
use crate::html::{html, Markup};

use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub struct OneComponent<T: ComponentTrait + Default>(Option<Arc<RwLock<T>>>);

impl<T: ComponentTrait + Default> OneComponent<T> {
    pub fn new() -> Self {
        OneComponent::<T>::default()
    }

    pub fn new_with(component: T) -> Self {
        OneComponent(Some(Arc::new(RwLock::new(component))))
    }

    pub fn set(&mut self, component: T) {
        self.0 = Some(Arc::new(RwLock::new(component)));
    }

    // OneComponent PREPARE.

    pub fn prepare(&self, rcx: &mut RenderContext) -> Markup {
        if let Some(component) = &self.0 {
            return component.write().unwrap().prepare(rcx);
        }
        html! {}
    }
}
