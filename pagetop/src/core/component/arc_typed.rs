use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, Markup};
use crate::{fn_builder, Handle, Weight};

use std::sync::{Arc, RwLock, RwLockReadGuard};

#[derive(Default)]
pub struct ArcTypedComponent<T: ComponentTrait>(Arc<RwLock<T>>);

impl<T: ComponentTrait> Clone for ArcTypedComponent<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ComponentTrait> ArcTypedComponent<T> {
    pub fn new(component: T) -> Self {
        ArcTypedComponent(Arc::new(RwLock::new(component)))
    }

    // ArcTypedComponent BUILDER.

    pub fn set(&mut self, component: T) {
        self.0 = Arc::new(RwLock::new(component));
    }

    // ArcTypedComponent GETTERS.

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

    // ArcTypedComponent RENDER.

    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().render(cx)
    }
}

// *************************************************************************************************

pub enum ArcTypedOp<T: ComponentTrait + Default> {
    Add(ArcTypedComponent<T>),
    AddAfterId(&'static str, ArcTypedComponent<T>),
    AddBeforeId(&'static str, ArcTypedComponent<T>),
    AddFirst(ArcTypedComponent<T>),
    RemoveById(&'static str),
    ReplaceById(&'static str, ArcTypedComponent<T>),
    Reset,
}

#[derive(Clone, Default)]
pub struct TypedComponents<T: ComponentTrait + Default>(Vec<ArcTypedComponent<T>>);

impl<T: ComponentTrait + Default> TypedComponents<T> {
    pub fn new(arc: ArcTypedComponent<T>) -> Self {
        TypedComponents::default().with_value(ArcTypedOp::Add(arc))
    }

    // TypedComponents BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, op: ArcTypedOp<T>) -> &mut Self {
        match op {
            ArcTypedOp::Add(one) => self.0.push(one),
            ArcTypedOp::AddAfterId(id, one) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index + 1, one),
                    _ => self.0.push(one),
                }
            }
            ArcTypedOp::AddBeforeId(id, one) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index, one),
                    _ => self.0.insert(0, one),
                }
            }
            ArcTypedOp::AddFirst(one) => self.0.insert(0, one),
            ArcTypedOp::RemoveById(id) => {
                if let Some(index) = self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    self.0.remove(index);
                }
            }
            ArcTypedOp::ReplaceById(id, one) => {
                for c in self.0.iter_mut() {
                    if c.id().as_deref() == Some(id) {
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

    pub fn get_by_id(&self, id: &'static str) -> Option<&ArcTypedComponent<T>> {
        self.0.iter().find(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_id(&self, id: &'static str) -> impl Iterator<Item = &ArcTypedComponent<T>> {
        self.0.iter().filter(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_handle(&self, handle: Handle) -> impl Iterator<Item = &ArcTypedComponent<T>> {
        self.0.iter().filter(move |&c| c.handle() == handle)
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
