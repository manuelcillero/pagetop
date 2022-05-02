use crate::html::{Markup, html};
use crate::core::response::page::PageAssets;
use crate::util::partial_type_name;

pub use std::any::Any as AnyComponent;

pub trait BaseComponent {
    fn type_name(&self) -> &'static str;

    fn single_name(&self) -> &'static str;

    fn qualified_name(&self, last: usize) -> &'static str;
}

pub trait ComponentTrait: AnyComponent + BaseComponent + Send + Sync {

    fn new() -> Self where Self: Sized;

    fn name(&self) -> String {
        self.single_name().to_owned()
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn is_renderable(&self) -> bool {
        true
    }

    fn weight(&self) -> i8 {
        0
    }

    #[allow(unused_variables)]
    fn before_render(&mut self, assets: &mut PageAssets) {
    }

    #[allow(unused_variables)]
    fn default_render(&self, assets: &mut PageAssets) -> Markup {
        html! {}
    }

    fn as_ref_any(&self) -> &dyn AnyComponent;

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent;
}

impl<C: ?Sized + ComponentTrait> BaseComponent for C {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn single_name(&self) -> &'static str {
        partial_type_name(std::any::type_name::<Self>(), 1)
    }

    fn qualified_name(&self, last: usize) -> &'static str {
        partial_type_name(std::any::type_name::<Self>(), last)
    }
}

pub fn component_ref<C: 'static>(component: &dyn ComponentTrait) -> &C {
    component.as_ref_any().downcast_ref::<C>().unwrap()
}

pub fn component_mut<C: 'static>(component: &mut dyn ComponentTrait) -> &mut C {
    component.as_mut_any().downcast_mut::<C>().unwrap()
}
