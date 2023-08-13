use crate::core::component::{ComponentTrait, Context};
use crate::html::Markup;
use crate::{Handle, Weight};

use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct ComponentArc(Arc<RwLock<dyn ComponentTrait>>);

impl ComponentArc {
    pub fn new(component: impl ComponentTrait) -> Self {
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

    pub(crate) fn prepare(&self, cx: &mut Context) -> Markup {
        self.0.write().unwrap().prepare(cx)
    }
}
