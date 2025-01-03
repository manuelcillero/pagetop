use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, Markup};
use crate::{fn_builder, UniqueId};

use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct ChildComponent(Arc<RwLock<dyn ComponentTrait>>);

impl ChildComponent {
    pub fn with(component: impl ComponentTrait) -> Self {
        ChildComponent(Arc::new(RwLock::new(component)))
    }

    // ChildComponent RENDER.

    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().render(cx)
    }

    // ChildComponent HELPERS.

    fn type_id(&self) -> UniqueId {
        self.0.read().unwrap().type_id()
    }

    fn id(&self) -> String {
        self.0.read().unwrap().id().unwrap_or_default()
    }
}

// *************************************************************************************************

pub struct TypedComponent<C: ComponentTrait>(Arc<RwLock<C>>);

impl<C: ComponentTrait> Clone for TypedComponent<C> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<C: ComponentTrait> TypedComponent<C> {
    pub fn with(component: C) -> Self {
        TypedComponent(Arc::new(RwLock::new(component)))
    }

    // TypedComponent RENDER.

    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().render(cx)
    }

    // TypedComponent HELPERS.

    fn to_child(&self) -> ChildComponent {
        ChildComponent(self.0.clone())
    }
}

// *************************************************************************************************

pub enum ChildOp {
    Add(ChildComponent),
    InsertAfterId(&'static str, ChildComponent),
    InsertBeforeId(&'static str, ChildComponent),
    Prepend(ChildComponent),
    RemoveById(&'static str),
    ReplaceById(&'static str, ChildComponent),
    Reset,
}

pub enum TypedOp<C: ComponentTrait> {
    Add(TypedComponent<C>),
    InsertAfterId(&'static str, TypedComponent<C>),
    InsertBeforeId(&'static str, TypedComponent<C>),
    Prepend(TypedComponent<C>),
    RemoveById(&'static str),
    ReplaceById(&'static str, TypedComponent<C>),
    Reset,
}

#[derive(Clone, Default)]
pub struct Children(Vec<ChildComponent>);

impl Children {
    pub fn new() -> Self {
        Children::default()
    }

    pub fn with(child: ChildComponent) -> Self {
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
    pub fn add(&mut self, child: ChildComponent) -> &mut Self {
        self.0.push(child);
        self
    }

    #[inline]
    fn insert_after_id(&mut self, id: &str, child: ChildComponent) -> &mut Self {
        match self.0.iter().position(|c| c.id() == id) {
            Some(index) => self.0.insert(index + 1, child),
            _ => self.0.push(child),
        };
        self
    }

    #[inline]
    fn insert_before_id(&mut self, id: &str, child: ChildComponent) -> &mut Self {
        match self.0.iter().position(|c| c.id() == id) {
            Some(index) => self.0.insert(index, child),
            _ => self.0.insert(0, child),
        };
        self
    }

    #[inline]
    fn prepend(&mut self, child: ChildComponent) -> &mut Self {
        self.0.insert(0, child);
        self
    }

    #[inline]
    fn remove_by_id(&mut self, id: &str) -> &mut Self {
        if let Some(index) = self.0.iter().position(|c| c.id() == id) {
            self.0.remove(index);
        }
        self
    }

    #[inline]
    fn replace_by_id(&mut self, id: &str, child: ChildComponent) -> &mut Self {
        for c in &mut self.0 {
            if c.id() == id {
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

    pub fn get_by_id(&self, id: impl Into<String>) -> Option<&ChildComponent> {
        let id = id.into();
        self.0.iter().find(|c| c.id() == id)
    }

    pub fn iter_by_id(&self, id: impl Into<String>) -> impl Iterator<Item = &ChildComponent> {
        let id = id.into();
        self.0.iter().filter(move |&c| c.id() == id)
    }

    pub fn iter_by_type_id(&self, type_id: UniqueId) -> impl Iterator<Item = &ChildComponent> {
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
