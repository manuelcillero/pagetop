use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, Markup};
use crate::{fn_builder, TypeId, Weight};

use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Clone)]
pub struct AnyComponent(Arc<RwLock<dyn ComponentTrait>>);

impl AnyComponent {
    pub fn with(component: impl ComponentTrait) -> Self {
        AnyComponent(Arc::new(RwLock::new(component)))
    }

    // AnyComponent BUILDER.

    pub fn set(&mut self, component: impl ComponentTrait) {
        self.0 = Arc::new(RwLock::new(component));
    }

    // AnyComponent GETTERS.

    pub fn get(&self) -> RwLockReadGuard<'_, dyn ComponentTrait> {
        self.0.read().unwrap()
    }

    pub fn get_mut(&self) -> RwLockWriteGuard<'_, dyn ComponentTrait> {
        self.0.write().unwrap()
    }

    // AnyComponent RENDER.

    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().render(cx)
    }

    // AnyComponent HELPERS.

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

pub enum MixedOp {
    Add(AnyComponent),
    InsertAfterId(&'static str, AnyComponent),
    InsertBeforeId(&'static str, AnyComponent),
    Prepend(AnyComponent),
    RemoveById(&'static str),
    ReplaceById(&'static str, AnyComponent),
    Reset,
}

#[derive(Clone, Default)]
pub struct MixedComponents(Vec<AnyComponent>);

impl MixedComponents {
    pub fn new() -> Self {
        MixedComponents::default()
    }

    pub fn with(any: AnyComponent) -> Self {
        MixedComponents::default().with_value(MixedOp::Add(any))
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
    pub fn alter_value(&mut self, op: MixedOp) -> &mut Self {
        match op {
            MixedOp::Add(any) => self.0.push(any),
            MixedOp::InsertAfterId(id, any) => match self.0.iter().position(|c| c.id() == id) {
                Some(index) => self.0.insert(index + 1, any),
                _ => self.0.push(any),
            },
            MixedOp::InsertBeforeId(id, any) => match self.0.iter().position(|c| c.id() == id) {
                Some(index) => self.0.insert(index, any),
                _ => self.0.insert(0, any),
            },
            MixedOp::Prepend(any) => self.0.insert(0, any),
            MixedOp::RemoveById(id) => {
                if let Some(index) = self.0.iter().position(|c| c.id() == id) {
                    self.0.remove(index);
                }
            }
            MixedOp::ReplaceById(id, any) => {
                for c in self.0.iter_mut() {
                    if c.id() == id {
                        *c = any;
                        break;
                    }
                }
            }
            MixedOp::Reset => self.0.clear(),
        }
        self
    }

    // MixedComponents GETTERS.

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_by_id(&self, id: impl Into<String>) -> Option<&AnyComponent> {
        let id = id.into();
        self.0.iter().find(|c| c.id() == id)
    }

    pub fn iter_by_id(&self, id: impl Into<String>) -> impl Iterator<Item = &AnyComponent> {
        let id = id.into();
        self.0.iter().filter(move |&c| c.id() == id)
    }

    pub fn iter_by_type_id(&self, type_id: TypeId) -> impl Iterator<Item = &AnyComponent> {
        self.0.iter().filter(move |&c| c.type_id() == type_id)
    }

    // MixedComponents RENDER.

    pub fn render(&self, cx: &mut Context) -> Markup {
        let mut components = self.0.clone();
        components.sort_by_key(|c| c.weight());
        html! {
            @for c in components.iter() {
                (c.render(cx))
            }
        }
    }
}
