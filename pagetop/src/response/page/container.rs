use crate::html::{Markup, html};
use crate::response::page::{PageAssets, ComponentTrait, render_component};

use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct PageContainer(Vec<Arc<RwLock<dyn ComponentTrait>>>);

impl PageContainer {
    pub fn new() -> Self {
        PageContainer(Vec::new())
    }

    pub fn new_with(component: impl ComponentTrait) -> Self {
        let mut container = PageContainer::new();
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
                (render_component(&mut *c.write().unwrap(), assets))
            }
        }
    }
}
