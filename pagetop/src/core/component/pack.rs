use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, Markup};
use crate::{fn_builder, Handle};

use std::sync::{Arc, RwLock};

pub enum PackOp {
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
pub struct PackComponents(Vec<ArcLockComponent>);

impl PackComponents {
    pub fn new() -> Self {
        PackComponents::default()
    }

    pub fn new_with(component: impl ComponentTrait) -> Self {
        let mut pack = PackComponents::new();
        pack.alter_pack(PackOp::Add, component);
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

    #[fn_builder]
    pub fn alter_pack(&mut self, op: PackOp, component: impl ComponentTrait) -> &mut Self {
        let arc = Arc::new(RwLock::new(component));
        match op {
            PackOp::Add => self.0.push(arc),
            PackOp::AddAfterId(id) => {
                match self
                    .0
                    .iter()
                    .position(|c| c.read().unwrap().id().as_deref() == Some(id))
                {
                    Some(index) => self.0.insert(index + 1, arc),
                    _ => self.0.push(arc),
                }
            }
            PackOp::AddBeforeId(id) => {
                match self
                    .0
                    .iter()
                    .position(|c| c.read().unwrap().id().as_deref() == Some(id))
                {
                    Some(index) => self.0.insert(index, arc),
                    _ => self.0.insert(0, arc),
                }
            }
            PackOp::AddFirst => self.0.insert(0, arc),
            PackOp::RemoveById(id) => {
                if let Some(index) = self
                    .0
                    .iter()
                    .position(|c| c.read().unwrap().id().as_deref() == Some(id))
                {
                    self.0.remove(index);
                }
            }
            PackOp::ReplaceById(id) => {
                for c in self.0.iter_mut() {
                    if c.read().unwrap().id().as_deref() == Some(id) {
                        *c = arc;
                        break;
                    }
                }
            }
            PackOp::Reset => self.0.clear(),
        }
        self
    }

    // PackComponents GETTERS.

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

    // PackComponents PREPARE.

    pub fn prepare(&self, cx: &mut Context) -> Markup {
        let mut components = self.0.clone();
        components.sort_by_key(|c| c.read().unwrap().weight());
        html! {
            @for c in components.iter() {
                (" ")(c.write().unwrap().prepare(cx))(" ")
            }
        }
    }
}
