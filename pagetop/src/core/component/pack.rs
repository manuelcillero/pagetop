use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, Markup};
use crate::{Handle, Weight};

use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct ComponentRef(Arc<RwLock<dyn ComponentTrait>>);

impl ComponentRef {
    pub fn to(component: impl ComponentTrait) -> Self {
        ComponentRef(Arc::new(RwLock::new(component)))
    }

    fn handle(&self) -> Handle {
        self.0.read().unwrap().handle()
    }

    fn id(&self) -> Option<String> {
        self.0.read().unwrap().id()
    }

    fn weight(&self) -> Weight {
        self.0.read().unwrap().weight()
    }

    fn prepare(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().prepare(cx)
    }
}

pub enum PackOp {
    Add,
    AddAfterId(&'static str),
    AddBeforeId(&'static str),
    AddFirst,
    RemoveById(&'static str),
    ReplaceById(&'static str),
    Reset,
}

#[derive(Clone, Default)]
pub struct PackComponents(Vec<ComponentRef>);

impl PackComponents {
    pub fn new() -> Self {
        PackComponents::default()
    }

    pub fn new_with(cref: ComponentRef) -> Self {
        let mut pack = PackComponents::new();
        pack.alter(PackOp::Add, cref);
        pack
    }

    pub(crate) fn merge(one: Option<&PackComponents>, other: Option<&PackComponents>) -> Self {
        if let Some(one) = one {
            let mut components = one.0.clone();
            if let Some(other) = other {
                components.append(&mut other.0.clone());
            }
            PackComponents(components)
        } else if let Some(other) = other {
            PackComponents(other.0.clone())
        } else {
            PackComponents::default()
        }
    }

    // PackComponents BUILDER.

    pub fn alter(&mut self, op: PackOp, cref: ComponentRef) -> &mut Self {
        match op {
            PackOp::Add => self.0.push(cref),
            PackOp::AddAfterId(id) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index + 1, cref),
                    _ => self.0.push(cref),
                }
            }
            PackOp::AddBeforeId(id) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index, cref),
                    _ => self.0.insert(0, cref),
                }
            }
            PackOp::AddFirst => self.0.insert(0, cref),
            PackOp::RemoveById(id) => {
                if let Some(index) = self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    self.0.remove(index);
                }
            }
            PackOp::ReplaceById(id) => {
                for c in self.0.iter_mut() {
                    if c.id().as_deref() == Some(id) {
                        *c = cref;
                        break;
                    }
                }
            }
            PackOp::Reset => self.0.clear(),
        }
        self
    }

    // PackComponents GETTERS.

    pub fn get_by_id(&self, id: &'static str) -> Option<&ComponentRef> {
        self.0.iter().find(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_id(&self, id: &'static str) -> impl Iterator<Item = &ComponentRef> {
        self.0.iter().filter(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_handle(&self, handle: Handle) -> impl Iterator<Item = &ComponentRef> {
        self.0.iter().filter(move |&c| c.handle() == handle)
    }

    // PackComponents PREPARE.

    pub fn prepare(&self, cx: &mut Context) -> Markup {
        let mut components = self.0.clone();
        components.sort_by_key(|c| c.weight());
        html! {
            @for c in components.iter() {
                (" ")(c.prepare(cx))(" ")
            }
        }
    }
}
