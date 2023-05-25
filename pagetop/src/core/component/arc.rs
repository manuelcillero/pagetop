use crate::core::component::{ComponentTrait, RenderContext};
use crate::html::{html, Markup};

use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub struct ComponentArc(Option<Arc<RwLock<dyn ComponentTrait>>>);

impl ComponentArc {
    pub fn new(component: impl ComponentTrait) -> Self {
        ComponentArc(Some(Arc::new(RwLock::new(component))))
    }

    pub fn replace(&mut self, component: impl ComponentTrait) {
        self.0 = Some(Arc::new(RwLock::new(component)));
    }

    pub fn weight(&self) -> isize {
        match &self.0 {
            Some(component) => component.read().unwrap().weight(),
            _ => 0,
        }
    }

    pub fn render(&self, rcx: &mut RenderContext) -> Markup {
        html! {
            @if let Some(component) = &self.0 {
                (component.write().unwrap().render(rcx))
            }
        }
    }
}
