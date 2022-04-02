use crate::html::{Markup, html};
use crate::response::page::{ArcComponent, PageAssets, render_component};

#[derive(Clone)]
pub struct PageContainer(Vec<ArcComponent>);

impl PageContainer {
    pub fn new() -> Self {
        PageContainer(Vec::new())
    }

    pub fn new_with(component: ArcComponent) -> Self {
        let mut container = PageContainer::new();
        container.add(component);
        container
    }

    pub fn add(&mut self, component: ArcComponent) {
        self.0.push(component);
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