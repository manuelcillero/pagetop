use crate::Lazy;
use super::{ComponentsHolder, ComponentTrait};

use std::sync::RwLock;
use std::collections::HashMap;

static COMPONENTS: Lazy<RwLock<HashMap<&str, ComponentsHolder>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

pub fn add_component_to(region: &'static str, component: impl ComponentTrait) {
    let mut hmap = COMPONENTS.write().unwrap();
    if let Some(regions) = hmap.get_mut(region) {
        regions.add(component);
    } else {
        hmap.insert(region, ComponentsHolder::new_with(component));
    }
}

pub fn common_components() -> HashMap<&'static str, ComponentsHolder> {
    COMPONENTS.read().unwrap().clone()
}
