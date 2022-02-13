use crate::core::theme::{Markup, html};
use crate::core::response::page::{PageAssets, PageComponent, render_component};

use std::sync::Arc;

#[derive(Clone)]
pub struct Container(Vec<Arc<dyn PageComponent>>);

impl Container {
    pub fn new() -> Self {
        Container(Vec::new())
    }

    pub fn new_with(component: impl PageComponent) -> Self {
        let mut container = Container::new();
        container.add(component);
        container
    }

    pub fn add(&mut self, component: impl PageComponent) {
        self.0.push(Arc::new(component));
    }

    pub fn render(&self, assets: &mut PageAssets) -> Markup {
        let mut components = self.0.clone();
        components.sort_by_key(|c| c.weight());
        html! {
            @for c in components.iter() {
                (render_component(&**c, assets))
            }
        }
    }
}