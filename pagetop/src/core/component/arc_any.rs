use crate::core::component::{ComponentTrait, Context};
use crate::html::{html, Markup};
use crate::{fn_builder, impl_handle, Handle, Weight};

use std::sync::{Arc, RwLock, RwLockReadGuard};

#[derive(Default)]
struct ComponentNull;

impl_handle!(COMPONENT_NULL for ComponentNull);

impl ComponentTrait for ComponentNull {
    fn new() -> Self {
        ComponentNull::default()
    }
}

#[derive(Clone)]
pub struct ArcAnyComponent(Arc<RwLock<dyn ComponentTrait>>);

impl Default for ArcAnyComponent {
    fn default() -> Self {
        ArcAnyComponent(Arc::new(RwLock::new(ComponentNull)))
    }
}

impl ArcAnyComponent {
    pub fn new(component: impl ComponentTrait) -> Self {
        ArcAnyComponent(Arc::new(RwLock::new(component)))
    }

    // ArcAnyComponent BUILDER.

    pub fn set(&mut self, component: impl ComponentTrait) {
        self.0 = Arc::new(RwLock::new(component));
    }

    // ArcAnyComponent GETTERS.

    pub fn get(&self) -> RwLockReadGuard<'_, dyn ComponentTrait> {
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

    // ArcAnyComponent RENDER.

    pub fn render(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().render(cx)
    }
}

// *************************************************************************************************

pub enum ArcAnyOp {
    Add(ArcAnyComponent),
    AddAfterId(&'static str, ArcAnyComponent),
    AddBeforeId(&'static str, ArcAnyComponent),
    AddFirst(ArcAnyComponent),
    RemoveById(&'static str),
    ReplaceById(&'static str, ArcAnyComponent),
    Reset,
}

#[derive(Clone, Default)]
pub struct AnyComponents(Vec<ArcAnyComponent>);

impl AnyComponents {
    pub fn new(arc: ArcAnyComponent) -> Self {
        AnyComponents::default().with_value(ArcAnyOp::Add(arc))
    }

    pub(crate) fn merge(mixes: &[Option<&AnyComponents>]) -> Self {
        let mut opt = AnyComponents::default();
        for m in mixes.iter().flatten() {
            opt.0.append(&mut m.0.clone());
        }
        opt
    }

    // AnyComponents BUILDER.

    #[fn_builder]
    pub fn alter_value(&mut self, op: ArcAnyOp) -> &mut Self {
        match op {
            ArcAnyOp::Add(arc) => self.0.push(arc),
            ArcAnyOp::AddAfterId(id, arc) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index + 1, arc),
                    _ => self.0.push(arc),
                }
            }
            ArcAnyOp::AddBeforeId(id, arc) => {
                match self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    Some(index) => self.0.insert(index, arc),
                    _ => self.0.insert(0, arc),
                }
            }
            ArcAnyOp::AddFirst(arc) => self.0.insert(0, arc),
            ArcAnyOp::RemoveById(id) => {
                if let Some(index) = self.0.iter().position(|c| c.id().as_deref() == Some(id)) {
                    self.0.remove(index);
                }
            }
            ArcAnyOp::ReplaceById(id, arc) => {
                for c in self.0.iter_mut() {
                    if c.id().as_deref() == Some(id) {
                        *c = arc;
                        break;
                    }
                }
            }
            ArcAnyOp::Reset => self.0.clear(),
        }
        self
    }

    // AnyComponents GETTERS.

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get_by_id(&self, id: &'static str) -> Option<&ArcAnyComponent> {
        self.0.iter().find(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_id(&self, id: &'static str) -> impl Iterator<Item = &ArcAnyComponent> {
        self.0.iter().filter(|&c| c.id().as_deref() == Some(id))
    }

    pub fn iter_by_handle(&self, handle: Handle) -> impl Iterator<Item = &ArcAnyComponent> {
        self.0.iter().filter(move |&c| c.handle() == handle)
    }

    // AnyComponents RENDER.

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
