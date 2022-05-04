use crate::Lazy;
use super::{ComponentTrait, PageContainer};

use std::sync::RwLock;
use std::collections::HashMap;

static COMPONENTS: Lazy<RwLock<HashMap<&str, PageContainer>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

pub fn add_component_to(region: &'static str, component: impl ComponentTrait) {
    let mut hmap = COMPONENTS.write().unwrap();
    if let Some(regions) = hmap.get_mut(region) {
        regions.add(component);
    } else {
        hmap.insert(region, PageContainer::new_with(component));
    }
}

pub fn common_components() -> HashMap<&'static str, PageContainer> {
    COMPONENTS.read().unwrap().clone()
}
