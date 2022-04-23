use crate::html::{Markup, html};
use crate::response::page::PageAssets;

use std::any::type_name;

pub use std::any::Any as AnyComponent;

pub trait PageComponent: AnyComponent + Send + Sync {

    fn new() -> Self where Self: Sized;

    fn name(&self) -> &'static str {
        let name = type_name::<Self>();
        match name.rfind("::") {
            Some(position) => &name[(position + 2)..],
            None => name
        }
    }

    fn fullname(&self) -> String {
        type_name::<Self>().to_owned()
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

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent;
}
