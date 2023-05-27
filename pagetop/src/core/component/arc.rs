use crate::core::component::{ComponentTrait, DefaultComponent, RenderContext};
use crate::html::{html, Markup};

use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct ComponentArc(Arc<RwLock<dyn ComponentTrait>>);

impl Default for ComponentArc {
    fn default() -> Self {
        ComponentArc(Arc::new(RwLock::new(DefaultComponent)))
    }
}

impl ComponentArc {
    pub fn new() -> Self {
        ComponentArc::default()
    }

    pub fn new_with(component: impl ComponentTrait) -> Self {
        ComponentArc(Arc::new(RwLock::new(component)))
    }

    pub fn set(&mut self, component: impl ComponentTrait) {
        self.0 = Arc::new(RwLock::new(component));
    }

    pub fn weight(&self) -> isize {
        self.0.read().unwrap().weight()
    }

    // ComponentArc RENDER.

    pub fn render(&self, rcx: &mut RenderContext) -> Markup {
        self.0.write().unwrap().render(rcx)
    }

    pub fn optional_render(&self, rcx: &mut RenderContext) -> Option<Markup> {
        let render = self.0.write().unwrap().render(rcx).into_string();
        if !render.trim().is_empty() {
            return Some(html! { (render) });
        }
        None
    }
}
