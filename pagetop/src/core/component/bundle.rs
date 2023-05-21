use crate::core::component::ComponentTrait;
use crate::html::{html, Markup, RenderContext};

use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub struct ComponentsBundle(Vec<Arc<RwLock<dyn ComponentTrait>>>);

impl ComponentsBundle {
    pub fn new() -> Self {
        ComponentsBundle::default()
    }

    pub fn new_with(component: impl ComponentTrait) -> Self {
        let mut container = ComponentsBundle::new();
        container.add(component);
        container
    }

    pub fn add(&mut self, component: impl ComponentTrait) {
        self.0.push(Arc::new(RwLock::new(component)));
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn render(&self, rcx: &mut RenderContext) -> Markup {
        let mut components = self.0.clone();
        components.sort_by_key(|c| c.read().unwrap().weight());
        html! {
            @for c in components.iter() {
                (" ")(c.write().unwrap().render(rcx))(" ")
            }
        }
    }
}
