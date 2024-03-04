use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, Markup};
use crate::{fn_builder, TypeId, Weight};

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Clone)]
pub struct OneComponent(Arc<RwLock<dyn ComponentTrait>>);

impl OneComponent {
    pub fn with(component: impl ComponentTrait) -> Self {
        OneComponent(Arc::new(RwLock::new(component)))
    }

    // OneComponent BUILDER.

    pub fn set(&mut self, component: impl ComponentTrait) {
        self.0 = Arc::new(RwLock::new(component));
    }

    // OneComponent GETTERS.

    pub fn get(&self) -> RwLockReadGuard<'_, dyn ComponentTrait> {
        self.0.read().unwrap()
    }

    pub fn get_mut(&self) -> RwLockWriteGuard<'_, dyn ComponentTrait> {
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

pub enum OneOp {
    Add(OneComponent),
    AddAfterId(&'static str, OneComponent),
    AddBeforeId(&'static str, OneComponent),
    Prepend(OneComponent),
    RemoveById(&'static str),
    ReplaceById(&'static str, OneComponent),
    Reset,
}

#[derive(Clone, Default)]
pub struct MixedComponents(Vec<OneComponent>);

impl MixedComponents {
    pub fn new(any: OneComponent) -> Self {
        MixedComponents::default().with_value(OneOp::Add(any))
    }

    pub(crate) fn merge(mixes: &[Option<&MixedComponents>]) -> Self {
        let mut opt = MixedComponents::default();
        for m in mixes.iter().flatten() {
            opt.0.append(&mut m.0.clone());
        }
        opt
    }

    // MixedComponents BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, op: OneOp) -> &mut Self {
        match op {
            OneOp::Add(any) => self.0.push(any),
            OneOp::AddAfterId(id, any) => match self.0.iter().position(|c| c.id() == id) {
                Some(index) => self.0.insert(index + 1, any),
                _ => self.0.push(any),
            },
            OneOp::AddBeforeId(id, any) => match self.0.iter().position(|c| c.id() == id) {
                Some(index) => self.0.insert(index, any),
                _ => self.0.insert(0, any),
            },
            OneOp::Prepend(any) => self.0.insert(0, any),
            OneOp::RemoveById(id) => {
                if let Some(index) = self.0.iter().position(|c| c.id() == id) {
                    self.0.remove(index);
                }
            }
            OneOp::ReplaceById(id, any) => {
                for c in self.0.iter_mut() {
                    if c.id() == id {
                        *c = any;
                        break;
                    }
                }
            }
            OneOp::Reset => self.0.clear(),
        }
        self
    }

    // MixedComponents GETTERS.

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_by_id(&self, id: impl Into<String>) -> Option<&OneComponent> {
        let id = id.into();
        self.0.iter().find(|c| c.id() == id)
    }

    pub fn iter_by_id(&self, id: impl Into<String>) -> impl Iterator<Item = &OneComponent> {
        let id = id.into();
        self.0.iter().filter(move |&c| c.id() == id)
    }

    pub fn iter_by_type_id(&self, type_id: TypeId) -> impl Iterator<Item = &OneComponent> {
        self.0.iter().filter(move |&c| c.type_id() == type_id)
    }

    // MixedComponents RENDER.

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
