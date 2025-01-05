use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, Markup};
use crate::{fn_builder, UniqueId};

use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Child(Arc<RwLock<dyn ComponentTrait>>);

impl Child {
    pub fn with(component: impl ComponentTrait) -> Self {
        Child(Arc::new(RwLock::new(component)))
    }

    // Child GETTERS.

    pub fn id(&self) -> Option<String> {
        self.0.read().unwrap().id()
    }
    /*
    pub fn writable(&self) -> RwLockWriteGuard<'_, dyn ComponentTrait> {
        self.0.write().unwrap()
    } */

    // Child RENDER.

    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().render(cx)
    }

    // Child HELPERS.

    fn type_id(&self) -> UniqueId {
        self.0.read().unwrap().type_id()
    }

    fn child_id(&self) -> String {
        self.0.read().unwrap().id().unwrap_or_default()
    }
}

// *************************************************************************************************

pub struct Typed<C: ComponentTrait>(Arc<RwLock<C>>);

impl<C: ComponentTrait> Clone for Typed<C> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<C: ComponentTrait> Typed<C> {
    pub fn with(component: C) -> Self {
        Typed(Arc::new(RwLock::new(component)))
    }

    // Typed GETTERS.

    pub fn id(&self) -> Option<String> {
        self.0.read().unwrap().id()
    }
    /*
    pub fn writable(&self) -> RwLockWriteGuard<'_, C> {
        self.0.write().unwrap()
    } */

    // Typed RENDER.

    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().render(cx)
    }

    // Typed HELPERS.

    fn to_child(&self) -> Child {
        Child(self.0.clone())
    }
}

// *************************************************************************************************

pub enum ChildOp {
    Add(Child),
    InsertAfterId(&'static str, Child),
    InsertBeforeId(&'static str, Child),
    Prepend(Child),
    RemoveById(&'static str),
    ReplaceById(&'static str, Child),
    Reset,
}

pub enum TypedOp<C: ComponentTrait> {
    Add(Typed<C>),
    InsertAfterId(&'static str, Typed<C>),
    InsertBeforeId(&'static str, Typed<C>),
    Prepend(Typed<C>),
    RemoveById(&'static str),
    ReplaceById(&'static str, Typed<C>),
    Reset,
}

#[derive(Clone, Default)]
pub struct Children(Vec<Child>);

impl Children {
    pub fn new() -> Self {
        Children::default()
    }

    pub fn with(child: Child) -> Self {
        Children::default().with_child(ChildOp::Add(child))
    }

    pub(crate) fn merge(mixes: &[Option<&Children>]) -> Self {
        let mut opt = Children::default();
        for m in mixes.iter().flatten() {
            opt.0.append(&mut m.0.clone());
        }
        opt
    }

    // Children BUILDER.

    #[fn_builder]
    pub fn with_child(mut self, op: ChildOp) -> Self {
        match op {
            ChildOp::Add(any) => self.add(any),
            ChildOp::InsertAfterId(id, any) => self.insert_after_id(id, any),
            ChildOp::InsertBeforeId(id, any) => self.insert_before_id(id, any),
            ChildOp::Prepend(any) => self.prepend(any),
            ChildOp::RemoveById(id) => self.remove_by_id(id),
            ChildOp::ReplaceById(id, any) => self.replace_by_id(id, any),
            ChildOp::Reset => self.reset(),
        }
    }

    #[fn_builder]
    pub fn with_typed<C: ComponentTrait + Default>(mut self, op: TypedOp<C>) -> Self {
        match op {
            TypedOp::Add(typed) => self.add(typed.to_child()),
            TypedOp::InsertAfterId(id, typed) => self.insert_after_id(id, typed.to_child()),
            TypedOp::InsertBeforeId(id, typed) => self.insert_before_id(id, typed.to_child()),
            TypedOp::Prepend(typed) => self.prepend(typed.to_child()),
            TypedOp::RemoveById(id) => self.remove_by_id(id),
            TypedOp::ReplaceById(id, typed) => self.replace_by_id(id, typed.to_child()),
            TypedOp::Reset => self.reset(),
        }
    }

    #[inline]
    pub fn add(&mut self, child: Child) -> &mut Self {
        self.0.push(child);
        self
    }

    #[inline]
    fn insert_after_id(&mut self, id: &str, child: Child) -> &mut Self {
        match self.0.iter().position(|c| c.child_id() == id) {
            Some(index) => self.0.insert(index + 1, child),
            _ => self.0.push(child),
        };
        self
    }

    #[inline]
    fn insert_before_id(&mut self, id: &str, child: Child) -> &mut Self {
        match self.0.iter().position(|c| c.child_id() == id) {
            Some(index) => self.0.insert(index, child),
            _ => self.0.insert(0, child),
        };
        self
    }

    #[inline]
    fn prepend(&mut self, child: Child) -> &mut Self {
        self.0.insert(0, child);
        self
    }

    #[inline]
    fn remove_by_id(&mut self, id: &str) -> &mut Self {
        if let Some(index) = self.0.iter().position(|c| c.child_id() == id) {
            self.0.remove(index);
        }
        self
    }

    #[inline]
    fn replace_by_id(&mut self, id: &str, child: Child) -> &mut Self {
        for c in &mut self.0 {
            if c.child_id() == id {
                *c = child;
                break;
            }
        }
        self
    }

    #[inline]
    fn reset(&mut self) -> &mut Self {
        self.0.clear();
        self
    }

    // Children GETTERS.

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_by_id(&self, id: impl Into<String>) -> Option<&Child> {
        let id = id.into();
        self.0.iter().find(|c| c.child_id() == id)
    }

    pub fn iter_by_id(&self, id: impl Into<String>) -> impl Iterator<Item = &Child> {
        let id = id.into();
        self.0.iter().filter(move |&c| c.child_id() == id)
    }

    pub fn iter_by_type_id(&self, type_id: UniqueId) -> impl Iterator<Item = &Child> {
        self.0.iter().filter(move |&c| c.type_id() == type_id)
    }

    // Children RENDER.

    pub fn render(&self, cx: &mut Context) -> Markup {
        html! {
            @for c in &self.0 {
                (c.render(cx))
            }
        }
    }
}
