use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, Markup};
use crate::{fn_with, TypeId, Weight};

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct ArcTypedComponent<C: ComponentTrait>(Arc<RwLock<C>>);

impl<C: ComponentTrait> Clone for ArcTypedComponent<C> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<C: ComponentTrait> ArcTypedComponent<C> {
    pub fn new(component: C) -> Self {
        ArcTypedComponent(Arc::new(RwLock::new(component)))
    }

    // ArcTypedComponent BUILDER.

    pub fn set(&mut self, component: C) {
        self.0 = Arc::new(RwLock::new(component));
    }

    // ArcTypedComponent GETTERS.

    pub fn get(&self) -> RwLockReadGuard<'_, C> {
        self.0.read().unwrap()
    }

    pub fn get_mut(&self) -> RwLockWriteGuard<'_, C> {
        self.0.write().unwrap()
    }

    // ArcTypedComponent RENDER.

    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().render(cx)
    }

    // ArcTypedComponent HELPERS.

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

pub enum ArcTypedOp<C: ComponentTrait> {
    Add(ArcTypedComponent<C>),
    AddAfterId(&'static str, ArcTypedComponent<C>),
    AddBeforeId(&'static str, ArcTypedComponent<C>),
    Prepend(ArcTypedComponent<C>),
    RemoveById(&'static str),
    ReplaceById(&'static str, ArcTypedComponent<C>),
    Reset,
}

#[derive(Clone, Default)]
pub struct TypedComponents<C: ComponentTrait>(Vec<ArcTypedComponent<C>>);

impl<C: ComponentTrait + Default> TypedComponents<C> {
    pub fn new(arc: ArcTypedComponent<C>) -> Self {
        TypedComponents::default().with_value(ArcTypedOp::Add(arc))
    }

    // TypedComponents BUILDER.

    #[fn_with]
    pub fn alter_value(&mut self, op: ArcTypedOp<C>) -> &mut Self {
        match op {
            ArcTypedOp::Add(one) => self.0.push(one),
            ArcTypedOp::AddAfterId(id, one) => match self.0.iter().position(|c| c.id() == id) {
                Some(index) => self.0.insert(index + 1, one),
                _ => self.0.push(one),
            },
            ArcTypedOp::AddBeforeId(id, one) => match self.0.iter().position(|c| c.id() == id) {
                Some(index) => self.0.insert(index, one),
                _ => self.0.insert(0, one),
            },
            ArcTypedOp::Prepend(one) => self.0.insert(0, one),
            ArcTypedOp::RemoveById(id) => {
                if let Some(index) = self.0.iter().position(|c| c.id() == id) {
                    self.0.remove(index);
                }
            }
            ArcTypedOp::ReplaceById(id, one) => {
                for c in self.0.iter_mut() {
                    if c.id() == id {
                        *c = one;
                        break;
                    }
                }
            }
            ArcTypedOp::Reset => self.0.clear(),
        }
        self
    }

    // TypedComponents GETTERS.

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_by_id(&self, id: impl Into<String>) -> Option<&ArcTypedComponent<C>> {
        let id = id.into();
        self.0.iter().find(|&c| c.id() == id)
    }

    pub fn iter_by_id(&self, id: impl Into<String>) -> impl Iterator<Item = &ArcTypedComponent<C>> {
        let id = id.into();
        self.0.iter().filter(move |&c| c.id() == id)
    }

    pub fn iter_by_type_id(&self, type_id: TypeId) -> impl Iterator<Item = &ArcTypedComponent<C>> {
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
