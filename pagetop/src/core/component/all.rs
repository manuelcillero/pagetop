use crate::core::component::{ComponentTrait, ComponentsBundle};
use crate::LazyStatic;

use std::collections::HashMap;
use std::sync::RwLock;

static COMPONENTS: LazyStatic<RwLock<HashMap<&str, ComponentsBundle>>> =
    LazyStatic::new(|| RwLock::new(HashMap::new()));

pub fn add_component_to(region: &'static str, component: impl ComponentTrait) {
    let mut hmap = COMPONENTS.write().unwrap();
    if let Some(regions) = hmap.get_mut(region) {
        regions.add(component);
    } else {
        hmap.insert(region, ComponentsBundle::new_with(component));
    }
}

pub fn common_components() -> HashMap<&'static str, ComponentsBundle> {
    COMPONENTS.read().unwrap().clone()
}
