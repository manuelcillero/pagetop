use crate::core::component::{ComponentOne, ComponentTrait, Context};
use crate::html::{html, Markup};
use crate::Handle;

pub enum LisOp<T: ComponentTrait + Default> {
    Add(ComponentOne<T>),
    AddAfterId(&'static str, ComponentOne<T>),
    AddBeforeId(&'static str, ComponentOne<T>),
    AddFirst(ComponentOne<T>),
    RemoveById(&'static str),
    ReplaceById(&'static str, ComponentOne<T>),
    Reset,
}

#[derive(Clone, Default)]
pub struct LisComponents<T: ComponentTrait + Default>(Vec<ComponentOne<T>>);

impl<T: ComponentTrait + Default> LisComponents<T> {
    pub fn new() -> Self {
        LisComponents::<T>::default()
    }

    pub fn with(one: ComponentOne<T>) -> Self {
        let mut components = LisComponents::new();
        components.alter(LisOp::Add(one));
        components
    }

    // LisComponents BUILDER.

    pub fn alter(&mut self, op: LisOp<T>) -> &mut Self {
        match op {
            LisOp::Add(one) => self.0.push(one),
            LisOp::AddAfterId(id, one) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index + 1, one),
                    _ => self.0.push(one),
                }
            }
            LisOp::AddBeforeId(id, one) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index, one),
                    _ => self.0.insert(0, one),
                }
            }
            LisOp::AddFirst(one) => self.0.insert(0, one),
            LisOp::RemoveById(id) => {
                if let Some(index) = self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    self.0.remove(index);
                }
            }
            LisOp::ReplaceById(id, one) => {
                for c in self.0.iter_mut() {
                    if c.id().as_deref() == Some(id) {
                        *c = one;
                        break;
                    }
                }
            }
            LisOp::Reset => self.0.clear(),
        }
        self
    }

    // LisComponents GETTERS.

    pub fn get_by_id(&self, id: &'static str) -> Option<&ComponentOne<T>> {
        self.0.iter().find(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_id(&self, id: &'static str) -> impl Iterator<Item = &ComponentOne<T>> {
        self.0.iter().filter(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_handle(&self, handle: Handle) -> impl Iterator<Item = &ComponentOne<T>> {
        self.0.iter().filter(move |&c| c.handle() == handle)
    }

    // LisComponents PREPARE.

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
