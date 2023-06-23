use crate::core::component::{ComponentTrait, RenderContext};
use crate::html::{html, Markup};
use crate::{fn_builder, Handle};

use std::sync::{Arc, RwLock};

pub enum BundleOp {
    Add,
    AddAfterId(&'static str),
    AddBeforeId(&'static str),
    AddFirst,
    RemoveById(&'static str),
    ReplaceById(&'static str),
    Reset,
}

pub type ArcLockComponent = Arc<RwLock<dyn ComponentTrait>>;

#[derive(Clone, Default)]
pub struct ComponentsBundle(Vec<ArcLockComponent>);

impl ComponentsBundle {
    pub fn new() -> Self {
        ComponentsBundle::default()
    }

    pub fn new_with(component: impl ComponentTrait) -> Self {
        let mut bundle = ComponentsBundle::new();
        bundle.alter_bundle(BundleOp::Add, component);
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

    // ComponentsBundle BUILDER.

    #[fn_builder]
    pub fn alter_bundle(&mut self, op: BundleOp, component: impl ComponentTrait) -> &mut Self {
        let arc = Arc::new(RwLock::new(component));
        match op {
            BundleOp::Add => self.0.push(arc),
            BundleOp::AddAfterId(id) => {
                match self
                    .0
                    .iter()
                    .position(|c| c.read().unwrap().id().as_deref() == Some(id))
                {
                    Some(index) => self.0.insert(index + 1, arc),
                    _ => self.0.push(arc),
                }
            }
            BundleOp::AddBeforeId(id) => {
                match self
                    .0
                    .iter()
                    .position(|c| c.read().unwrap().id().as_deref() == Some(id))
                {
                    Some(index) => self.0.insert(index, arc),
                    _ => self.0.insert(0, arc),
                }
            }
            BundleOp::AddFirst => self.0.insert(0, arc),
            BundleOp::RemoveById(id) => {
                if let Some(index) = self
                    .0
                    .iter()
                    .position(|c| c.read().unwrap().id().as_deref() == Some(id))
                {
                    self.0.remove(index);
                }
            }
            BundleOp::ReplaceById(id) => {
                for c in self.0.iter_mut() {
                    if c.read().unwrap().id().as_deref() == Some(id) {
                        *c = arc;
                        break;
                    }
                }
            }
            BundleOp::Reset => self.0.clear(),
        }
        self
    }

    // ComponentsBundle GETTERS.

    pub fn get_by_id(&self, id: &'static str) -> Option<&ArcLockComponent> {
        self.0
            .iter()
            .find(|&c| c.read().unwrap().id().as_deref() == Some(id))
    }

    pub fn iter_by_id(&self, id: &'static str) -> impl Iterator<Item = &ArcLockComponent> {
        self.0
            .iter()
            .filter(|&c| c.read().unwrap().id().as_deref() == Some(id))
    }

    pub fn iter_by_handle(&self, handle: Handle) -> impl Iterator<Item = &ArcLockComponent> {
        self.0
            .iter()
            .filter(move |&c| c.read().unwrap().handle() == handle)
    }

    // ComponentsBundle RENDER.

    pub fn prepare(&self, rcx: &mut RenderContext) -> Markup {
        let mut components = self.0.clone();
        components.sort_by_key(|c| c.read().unwrap().weight());
        html! {
            @for c in components.iter() {
                (" ")(c.write().unwrap().prepare(rcx))(" ")
            }
        }
    }
}
