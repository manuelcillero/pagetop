use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, Markup};
use crate::{fn_builder, TypeId, Weight};

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct OneComponent<C: ComponentTrait>(Arc<RwLock<C>>);

impl<C: ComponentTrait> Clone for OneComponent<C> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<C: ComponentTrait> OneComponent<C> {
    pub fn with(component: C) -> Self {
        OneComponent(Arc::new(RwLock::new(component)))
    }

    // OneComponent BUILDER.

    pub fn set(&mut self, component: C) {
        self.0 = Arc::new(RwLock::new(component));
    }

    // OneComponent GETTERS.

    pub fn get(&self) -> RwLockReadGuard<'_, C> {
        self.0.read().unwrap()
    }

    pub fn get_mut(&self) -> RwLockWriteGuard<'_, C> {
        self.0.write().unwrap()
    }

    // OneComponent RENDER.

    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().render(cx)
    }

    // OneComponent HELPERS.

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
    Add(OneComponent<C>),
    InsertAfterId(&'static str, OneComponent<C>),
    InsertBeforeId(&'static str, OneComponent<C>),
    Prepend(OneComponent<C>),
    RemoveById(&'static str),
    ReplaceById(&'static str, OneComponent<C>),
    Reset,
}

#[derive(Clone, Default)]
pub struct TypedComponents<C: ComponentTrait>(Vec<OneComponent<C>>);

impl<C: ComponentTrait + Default> TypedComponents<C> {
    pub fn new(one: OneComponent<C>) -> Self {
        TypedComponents::default().with_value(TypedOp::Add(one))
    }

    // TypedComponents BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, op: TypedOp<C>) -> &mut Self {
        match op {
            TypedOp::Add(one) => self.0.push(one),
            TypedOp::InsertAfterId(id, one) => match self.0.iter().position(|c| c.id() == id) {
                Some(index) => self.0.insert(index + 1, one),
                _ => self.0.push(one),
            },
            TypedOp::InsertBeforeId(id, one) => match self.0.iter().position(|c| c.id() == id) {
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

    // TypedComponents GETTERS.

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_by_id(&self, id: impl Into<String>) -> Option<&OneComponent<C>> {
        let id = id.into();
        self.0.iter().find(|&c| c.id() == id)
    }

    pub fn iter_by_id(&self, id: impl Into<String>) -> impl Iterator<Item = &OneComponent<C>> {
        let id = id.into();
        self.0.iter().filter(move |&c| c.id() == id)
    }

    pub fn iter_by_type_id(&self, type_id: TypeId) -> impl Iterator<Item = &OneComponent<C>> {
        self.0.iter().filter(move |&c| c.type_id() == type_id)
    }

    // TypedComponents RENDER.

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
