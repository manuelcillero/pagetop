use crate::prelude::*;

pub struct Basic;

impl PackageTrait for Basic {
    fn layout(&self) -> Option<LayoutRef> {
        Some(&Basic)
    }
}

impl LayoutTrait for Basic {}
