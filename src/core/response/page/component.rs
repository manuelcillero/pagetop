use crate::core::theme::{Markup, html};
use crate::core::response::page::PageAssets;

use downcast_rs::{Downcast, impl_downcast};

use std::any::type_name;

pub trait Component: Downcast + Send + Sync {

    fn prepare() -> Self where Self: Sized;

    fn name(&self) -> &'static str {
        let name = type_name::<Self>();
        match name.rfind("::") {
            Some(position) => &name[(position + 2)..],
            None => name
        }
    }

    fn fullname(&self) -> String {
        type_name::<Self>().to_string()
    }

    fn description(&self) -> String {
        "".to_string()
    }

    fn is_renderable(&self) -> bool {
        true
    }

    fn weight(&self) -> i8 {
        0
    }

    #[allow(unused_variables)]
    fn default_render(&self, assets: &mut PageAssets) -> Markup {
        html! {}
    }
}

impl_downcast!(Component);
