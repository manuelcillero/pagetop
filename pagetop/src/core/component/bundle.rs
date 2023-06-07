use crate::core::component::{ComponentTrait, RenderContext};
use crate::html::{html, Markup};

use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub struct ComponentsBundle(Vec<Arc<RwLock<dyn ComponentTrait>>>);

impl ComponentsBundle {
    pub fn new() -> Self {
        ComponentsBundle::default()
    }

    pub fn new_with(component: impl ComponentTrait) -> Self {
        let mut bundle = ComponentsBundle::new();
        bundle.add(component);
        bundle
    }

    pub(crate) fn merge(one: Option<&ComponentsBundle>, other: Option<&ComponentsBundle>) -> Self {
        if let Some(one) = one {
            let mut components = one.0.clone();
            if let Some(other) = other {
                components.append(&mut other.0.clone());
            }
            ComponentsBundle(components)
        } else if let Some(other) = other {
            ComponentsBundle(other.0.clone())
        } else {
            ComponentsBundle::default()
        }
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
