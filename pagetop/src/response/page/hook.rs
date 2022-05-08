use crate::core::hook::{HookTrait, AnyHook};
use super::Page;

pub const BEFORE_RENDER_PAGE_HOOK: &str = "pagetop::action::before_render_page";

pub struct BeforeRenderPageHook {
    hook: Option<fn(&mut Page)>,
    weight: isize,
}

impl HookTrait for BeforeRenderPageHook {
    fn new() -> Self {
        BeforeRenderPageHook {
            hook: None,
            weight: 0,
        }
    }

    fn handler(&self) -> &'static str {
        BEFORE_RENDER_PAGE_HOOK
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn as_ref_any(&self) -> &dyn AnyHook {
        self
    }
}

impl BeforeRenderPageHook {
    pub fn with_hook(mut self, hook: fn(&mut Page)) -> Self {
        self.hook = Some(hook);
        self
    }

    pub fn with_weight(mut self, weight: isize) -> Self {
        self.weight = weight;
        self
    }

    pub fn run(&self, page: &mut Page) {
        if let Some(hook) = self.hook {
            hook(page)
        }
    }
}
