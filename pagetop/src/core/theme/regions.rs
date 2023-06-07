use crate::core::component::{ComponentTrait, ComponentsBundle};
use crate::LazyStatic;

use std::collections::HashMap;
use std::sync::RwLock;

static THEME_REGIONS: LazyStatic<RwLock<HashMap<&'static str, ComponentsRegions>>> =
    LazyStatic::new(|| RwLock::new(HashMap::new()));

#[derive(Default)]
pub struct ComponentsRegions(HashMap<&'static str, ComponentsBundle>);

impl ComponentsRegions {
    pub fn new() -> Self {
        ComponentsRegions::default()
    }

    pub fn add_to(&mut self, region: &'static str, component: impl ComponentTrait) {
        if let Some(region) = self.0.get_mut(region) {
            region.add(component);
        } else {
            self.0.insert(region, ComponentsBundle::new_with(component));
        }
    }

    pub fn get_extended_bundle(&self, theme: &str, region: &str) -> ComponentsBundle {
        if let Some(hm_theme) = THEME_REGIONS.read().unwrap().get(theme) {
            ComponentsBundle::merge(self.0.get(region), hm_theme.0.get(region))
        } else {
            ComponentsBundle::merge(self.0.get(region), None)
        }
    }
}

pub fn add_component_to(theme: &'static str, region: &'static str, component: impl ComponentTrait) {
    let mut hm = THEME_REGIONS.write().unwrap();
    if let Some(hm_theme) = hm.get_mut(theme) {
        hm_theme.add_to(region, component);
    } else {
        hm.insert(theme, {
            let mut regions = ComponentsRegions::new();
            regions.add_to(region, component);
            regions
        });
    }
}
