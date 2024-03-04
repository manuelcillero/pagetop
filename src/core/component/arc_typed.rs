use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, Markup};
use crate::{fn_builder, TypeId, Weight};

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

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

    // TypedComponent BUILDER.

    pub fn set(&mut self, component: C) {
        self.0 = Arc::new(RwLock::new(component));
    }

    // TypedComponent GETTERS.

    pub fn get(&self) -> RwLockReadGuard<'_, C> {
        self.0.read().unwrap()
    }

    pub fn get_mut(&self) -> RwLockWriteGuard<'_, C> {
        self.0.write().unwrap()
    }

    // TypedComponent RENDER.

    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().render(cx)
    }

    // TypedComponent HELPERS.

    fn type_id(&self) -> TypeId {
        self.0.read().unwrap().type_id()
    }

    fn id(&self) -> String {
        self.0.read().unwrap().id().unwrap_or_default()
    }

    fn weight(&self) -> Weight {
        self.0.read().unwrap().weight()
    }
}

// *************************************************************************************************

pub enum TypedOp<C: ComponentTrait> {
    Add(TypedComponent<C>),
    AddAfterId(&'static str, TypedComponent<C>),
    AddBeforeId(&'static str, TypedComponent<C>),
    Prepend(TypedComponent<C>),
    RemoveById(&'static str),
    ReplaceById(&'static str, TypedComponent<C>),
    Reset,
}

#[derive(Clone, Default)]
pub struct VectorComponents<C: ComponentTrait>(Vec<TypedComponent<C>>);

impl<C: ComponentTrait + Default> VectorComponents<C> {
    pub fn new(one: TypedComponent<C>) -> Self {
        VectorComponents::default().with_value(TypedOp::Add(one))
    }

    // VectorComponents BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, op: TypedOp<C>) -> &mut Self {
        match op {
            TypedOp::Add(one) => self.0.push(one),
            TypedOp::AddAfterId(id, one) => match self.0.iter().position(|c| c.id() == id) {
                Some(index) => self.0.insert(index + 1, one),
                _ => self.0.push(one),
            },
            TypedOp::AddBeforeId(id, one) => match self.0.iter().position(|c| c.id() == id) {
                Some(index) => self.0.insert(index, one),
                _ => self.0.insert(0, one),
            },
            TypedOp::Prepend(one) => self.0.insert(0, one),
            TypedOp::RemoveById(id) => {
                if let Some(index) = self.0.iter().position(|c| c.id() == id) {
                    self.0.remove(index);
                }
            }
            TypedOp::ReplaceById(id, one) => {
                for c in self.0.iter_mut() {
                    if c.id() == id {
                        *c = one;
                        break;
                    }
                }
            }
            TypedOp::Reset => self.0.clear(),
        }
        self
    }

    // VectorComponents GETTERS.

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_by_id(&self, id: impl Into<String>) -> Option<&TypedComponent<C>> {
        let id = id.into();
        self.0.iter().find(|&c| c.id() == id)
    }

    pub fn iter_by_id(&self, id: impl Into<String>) -> impl Iterator<Item = &TypedComponent<C>> {
        let id = id.into();
        self.0.iter().filter(move |&c| c.id() == id)
    }

    pub fn iter_by_type_id(&self, type_id: TypeId) -> impl Iterator<Item = &TypedComponent<C>> {
        self.0.iter().filter(move |&c| c.type_id() == type_id)
    }

    // VectorComponents RENDER.

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
