use crate::core::component::{ComponentTrait, Context};
use crate::html::Markup;
use crate::{new_handle, Handle, Weight};

use std::sync::{Arc, RwLock};

new_handle!(COMPONENT_NULL for Crate);

#[derive(Default)]
struct ComponentNull;

impl ComponentTrait for ComponentNull {
    fn new() -> Self {
        ComponentNull::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_NULL
    }
}

#[derive(Clone)]
pub struct ComponentArc(Arc<RwLock<dyn ComponentTrait>>);

impl Default for ComponentArc {
    fn default() -> Self {
        ComponentArc(Arc::new(RwLock::new(ComponentNull)))
    }
}

impl ComponentArc {
    pub fn new() -> Self {
        ComponentArc::default()
    }

    pub fn with(component: impl ComponentTrait) -> Self {
        ComponentArc(Arc::new(RwLock::new(component)))
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

    // ComponentArc PREPARE.

    pub fn prepare(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().prepare(cx)
    }
}
