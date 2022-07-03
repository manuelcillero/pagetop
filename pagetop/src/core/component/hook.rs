use crate::core::hook::{HookTrait, AnyHook};
use super::{ComponentTrait, InContext};

pub const BEFORE_RENDER_COMPONENT_HOOK: &str = "pagetop::hook::before_render_component";

pub struct BeforeRenderComponentHook {
    hook: Option<fn(&mut dyn ComponentTrait, &mut InContext)>,
    weight: isize,
}

impl HookTrait for BeforeRenderComponentHook {
    fn new() -> Self {
        BeforeRenderComponentHook {
            hook: None,
            weight: 0,
        }
    }

    fn handler(&self) -> &'static str {
        BEFORE_RENDER_COMPONENT_HOOK
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn as_ref_any(&self) -> &dyn AnyHook {
        self
    }
}

impl BeforeRenderComponentHook {
    pub fn with_hook(mut self, hook: fn(&mut dyn ComponentTrait, &mut InContext)) -> Self {
        self.hook = Some(hook);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.weight = weight;
        self
    }

    pub fn run(&self, component: &mut dyn ComponentTrait, context: &mut InContext) {
        if let Some(hook) = self.hook {
            hook(component, context)
        }
    }
}