use crate::core::component::{ComponentArc, ComponentTrait, RenderContext};
use crate::html::{html, Markup};

#[derive(Clone, Default)]
pub struct ComponentsBundle(Vec<ComponentArc>);

impl ComponentsBundle {
    pub fn new() -> Self {
        ComponentsBundle::default()
    }

    pub fn new_with(component: impl ComponentTrait) -> Self {
        let mut bundle = ComponentsBundle::new();
        bundle.add(component);
        bundle
    }

    pub fn add(&mut self, component: impl ComponentTrait) {
        self.0.push(ComponentArc::new_with(component));
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn render(&self, rcx: &mut RenderContext) -> Markup {
        let mut components = self.0.clone();
        components.sort_by_key(|c| c.weight());
        html! {
            @for c in components.iter() {
                (" ")(c.render(rcx))(" ")
            }
        }
    }
}
