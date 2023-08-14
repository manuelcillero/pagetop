use crate::core::component::{ComponentOne, ComponentTrait, Context};
use crate::html::{html, Markup};
use crate::Handle;

pub enum VeckOp<T: ComponentTrait + Default> {
    Add(ComponentOne<T>),
    AddAfterId(&'static str, ComponentOne<T>),
    AddBeforeId(&'static str, ComponentOne<T>),
    AddFirst(ComponentOne<T>),
    RemoveById(&'static str),
    ReplaceById(&'static str, ComponentOne<T>),
    Reset,
}

#[derive(Clone, Default)]
pub struct VeckComponents<T: ComponentTrait + Default>(Vec<ComponentOne<T>>);

impl<T: ComponentTrait + Default> VeckComponents<T> {
    pub fn new() -> Self {
        VeckComponents::<T>::default()
    }

    pub fn with(one: ComponentOne<T>) -> Self {
        let mut veck = VeckComponents::new();
        veck.alter(VeckOp::Add(one));
        veck
    }

    // VeckComponents BUILDER.

    pub fn alter(&mut self, op: VeckOp<T>) -> &mut Self {
        match op {
            VeckOp::Add(one) => self.0.push(one),
            VeckOp::AddAfterId(id, one) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index + 1, one),
                    _ => self.0.push(one),
                }
            }
            VeckOp::AddBeforeId(id, one) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index, one),
                    _ => self.0.insert(0, one),
                }
            }
            VeckOp::AddFirst(one) => self.0.insert(0, one),
            VeckOp::RemoveById(id) => {
                if let Some(index) = self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    self.0.remove(index);
                }
            }
            VeckOp::ReplaceById(id, one) => {
                for c in self.0.iter_mut() {
                    if c.id().as_deref() == Some(id) {
                        *c = one;
                        break;
                    }
                }
            }
            VeckOp::Reset => self.0.clear(),
        }
        self
    }

    // VeckComponents GETTERS.

    pub fn get_by_id(&self, id: &'static str) -> Option<&ComponentOne<T>> {
        self.0.iter().find(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_id(&self, id: &'static str) -> impl Iterator<Item = &ComponentOne<T>> {
        self.0.iter().filter(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_handle(&self, handle: Handle) -> impl Iterator<Item = &ComponentOne<T>> {
        self.0.iter().filter(move |&c| c.handle() == handle)
    }

    // VeckComponents PREPARE.

    pub fn prepare(&self, cx: &mut Context) -> Markup {
        let mut components = self.0.clone();
        components.sort_by_key(|c| c.weight());
        html! {
            @for c in components.iter() {
                " " (c.prepare(cx)) " "
            }
        }
    }
}
