use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, Markup};
use crate::{impl_handle, Handle, Weight};

use std::sync::{Arc, RwLock, RwLockReadGuard};

#[derive(Default)]
struct ComponentNull;

impl_handle!(COMPONENT_NULL for ComponentNull);

impl ComponentTrait for ComponentNull {
    fn new() -> Self {
        ComponentNull::default()
    }
}

#[derive(Clone)]
pub struct ArcComponent(Arc<RwLock<dyn ComponentTrait>>);

impl Default for ArcComponent {
    fn default() -> Self {
        ArcComponent(Arc::new(RwLock::new(ComponentNull)))
    }
}

impl ArcComponent {
    pub fn new() -> Self {
        ArcComponent::default()
    }

    pub fn with(component: impl ComponentTrait) -> Self {
        ArcComponent(Arc::new(RwLock::new(component)))
    }

    // ArcComponent BUILDER.

    pub fn set(&mut self, component: impl ComponentTrait) {
        self.0 = Arc::new(RwLock::new(component));
    }

    // ArcComponent GETTERS.

    pub fn get(&self) -> RwLockReadGuard<'_, dyn ComponentTrait> {
        self.0.read().unwrap()
    }

    pub(crate) fn handle(&self) -> Handle {
        self.0.read().unwrap().handle()
    }

    pub(crate) fn id(&self) -> Option<String> {
        self.0.read().unwrap().id()
    }

    pub(crate) fn weight(&self) -> Weight {
        self.0.read().unwrap().weight()
    }

    // ArcComponent RENDER.

    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().render(cx)
    }
}

pub enum ArcOp {
    Add(ArcComponent),
    AddAfterId(&'static str, ArcComponent),
    AddBeforeId(&'static str, ArcComponent),
    AddFirst(ArcComponent),
    RemoveById(&'static str),
    ReplaceById(&'static str, ArcComponent),
    Reset,
}

#[derive(Clone, Default)]
pub struct ArcComponents(Vec<ArcComponent>);

impl ArcComponents {
    pub fn new() -> Self {
        ArcComponents::default()
    }

    pub fn with(arc: ArcComponent) -> Self {
        let mut components = ArcComponents::new();
        components.alter(ArcOp::Add(arc));
        components
    }

    pub(crate) fn merge(mixes: &[Option<&ArcComponents>]) -> Self {
        let mut components = ArcComponents::default();
        for m in mixes.iter().flatten() {
            components.0.append(&mut m.0.clone());
        }
        components
    }

    // ArcComponents BUILDER.

    pub fn alter(&mut self, op: ArcOp) -> &mut Self {
        match op {
            ArcOp::Add(arc) => self.0.push(arc),
            ArcOp::AddAfterId(id, arc) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index + 1, arc),
                    _ => self.0.push(arc),
                }
            }
            ArcOp::AddBeforeId(id, arc) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index, arc),
                    _ => self.0.insert(0, arc),
                }
            }
            ArcOp::AddFirst(arc) => self.0.insert(0, arc),
            ArcOp::RemoveById(id) => {
                if let Some(index) = self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    self.0.remove(index);
                }
            }
            ArcOp::ReplaceById(id, arc) => {
                for c in self.0.iter_mut() {
                    if c.id().as_deref() == Some(id) {
                        *c = arc;
                        break;
                    }
                }
            }
            ArcOp::Reset => self.0.clear(),
        }
        self
    }

    // ArcComponents GETTERS.

    pub fn get_by_id(&self, id: &'static str) -> Option<&ArcComponent> {
        self.0.iter().find(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_id(&self, id: &'static str) -> impl Iterator<Item = &ArcComponent> {
        self.0.iter().filter(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_handle(&self, handle: Handle) -> impl Iterator<Item = &ArcComponent> {
        self.0.iter().filter(move |&c| c.handle() == handle)
    }

    // ArcComponents RENDER.

    pub fn render(&self, cx: &mut Context) -> Markup {
        let mut components = self.0.clone();
        components.sort_by_key(|c| c.weight());
        html! {
            @for c in components.iter() {
                " " (c.render(cx)) " "
            }
        }
    }
}
