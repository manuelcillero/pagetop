use super::Page;
use crate::core::hook::{AnyHookAction, HookActionTrait};

pub const HOOK_BEFORE_RENDER_PAGE: &str = "pagetop::hook::before_render_page";

type Hook = fn(&mut Page);

pub struct BeforeRenderPageHook {
    hook: Option<Hook>,
    weight: isize,
}

impl HookActionTrait for BeforeRenderPageHook {
    fn new() -> Self {
        BeforeRenderPageHook {
            hook: None,
            weight: 0,
        }
    }

    fn handler(&self) -> &'static str {
        HOOK_BEFORE_RENDER_PAGE
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn as_ref_any(&self) -> &dyn AnyHookAction {
        self
    }
}

impl BeforeRenderPageHook {
    pub fn with_hook(mut self, hook: Hook) -> Self {
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
