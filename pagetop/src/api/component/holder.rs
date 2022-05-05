use crate::html::{Markup, html};
use super::{ComponentTrait, PageAssets};

use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct ComponentsHolder(Vec<Arc<RwLock<dyn ComponentTrait>>>);

impl ComponentsHolder {
    pub fn new() -> Self {
        ComponentsHolder(Vec::new())
    }

    pub fn new_with(component: impl ComponentTrait) -> Self {
        let mut container = ComponentsHolder::new();
        container.add(component);
        container
    }

    pub fn add(&mut self, component: impl ComponentTrait) {
        self.0.push(Arc::new(RwLock::new(component)));
    }

    pub fn render(&self, assets: &mut PageAssets) -> Markup {
        let mut components = self.0.clone();
        components.sort_by_key(|c| c.read().unwrap().weight());
        html! {
            @for c in components.iter() {
                (super::render_component(&mut *c.write().unwrap(), assets))
            }
        }
    }
}
