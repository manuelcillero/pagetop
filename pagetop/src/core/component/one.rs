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

    // OneComponent RENDER.

    pub fn render(&self, rcx: &mut RenderContext) -> Markup {
        if let Some(component) = &self.0 {
            return component.write().unwrap().render(rcx);
        }
        html! {}
    }

    pub fn optional_render(&self, rcx: &mut RenderContext) -> Option<Markup> {
        if let Some(component) = &self.0 {
            let render = component.write().unwrap().render(rcx).into_string();
            if !render.trim().is_empty() {
                return Some(html! { (render) });
            }
        }
        None
    }
}
