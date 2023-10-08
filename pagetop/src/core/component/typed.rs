use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, Markup};
use crate::{Handle, Weight};

use std::sync::{Arc, RwLock, RwLockReadGuard};

#[derive(Default)]
pub struct TypedComponent<T: ComponentTrait + Default>(Arc<RwLock<T>>);

impl<T: ComponentTrait + Default> Clone for TypedComponent<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ComponentTrait + Default> TypedComponent<T> {
    pub fn new() -> Self {
        TypedComponent::<T>::default()
    }

    pub fn with(component: T) -> Self {
        TypedComponent(Arc::new(RwLock::new(component)))
    }

    // TypedComponent BUILDER.

    pub fn set(&mut self, component: T) {
        self.0 = Arc::new(RwLock::new(component));
    }

    // TypedComponent GETTERS.

    pub fn get(&self) -> RwLockReadGuard<'_, T> {
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

    // TypedComponent PREPARE.

    pub fn prepare(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().prepare(cx)
    }
}

pub enum TypedOp<T: ComponentTrait + Default> {
    Add(TypedComponent<T>),
    AddAfterId(&'static str, TypedComponent<T>),
    AddBeforeId(&'static str, TypedComponent<T>),
    AddFirst(TypedComponent<T>),
    RemoveById(&'static str),
    ReplaceById(&'static str, TypedComponent<T>),
    Reset,
}

#[derive(Clone, Default)]
pub struct TypedComponents<T: ComponentTrait + Default>(Vec<TypedComponent<T>>);

impl<T: ComponentTrait + Default> TypedComponents<T> {
    pub fn new() -> Self {
        TypedComponents::<T>::default()
    }

    pub fn with(one: TypedComponent<T>) -> Self {
        let mut components = TypedComponents::new();
        components.alter(TypedOp::Add(one));
        components
    }

    // TypedComponents BUILDER.

    pub fn alter(&mut self, op: TypedOp<T>) -> &mut Self {
        match op {
            TypedOp::Add(one) => self.0.push(one),
            TypedOp::AddAfterId(id, one) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index + 1, one),
                    _ => self.0.push(one),
                }
            }
            TypedOp::AddBeforeId(id, one) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index, one),
                    _ => self.0.insert(0, one),
                }
            }
            TypedOp::AddFirst(one) => self.0.insert(0, one),
            TypedOp::RemoveById(id) => {
                if let Some(index) = self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    self.0.remove(index);
                }
            }
            TypedOp::ReplaceById(id, one) => {
                for c in self.0.iter_mut() {
                    if c.id().as_deref() == Some(id) {
                        *c = one;
                        break;
                    }
                }
            }
            TypedOp::Reset => self.0.clear(),
        }
        self
    }

    // TypedComponents GETTERS.

    pub fn get_by_id(&self, id: &'static str) -> Option<&TypedComponent<T>> {
        self.0.iter().find(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_id(&self, id: &'static str) -> impl Iterator<Item = &TypedComponent<T>> {
        self.0.iter().filter(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_handle(&self, handle: Handle) -> impl Iterator<Item = &TypedComponent<T>> {
        self.0.iter().filter(move |&c| c.handle() == handle)
    }

    // TypedComponents PREPARE.

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
