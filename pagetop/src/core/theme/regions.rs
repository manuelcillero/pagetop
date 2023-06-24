use crate::core::component::{ComponentTrait, PackComponents, PackOp};
use crate::LazyStatic;

use std::collections::HashMap;
use std::sync::RwLock;

static THEME_REGIONS: LazyStatic<RwLock<HashMap<&'static str, ComponentsRegions>>> =
    LazyStatic::new(|| RwLock::new(HashMap::new()));

#[derive(Default)]
pub struct ComponentsRegions(HashMap<&'static str, PackComponents>);

impl ComponentsRegions {
    pub fn new() -> Self {
        ComponentsRegions::default()
    }

    pub fn add_to(&mut self, region: &'static str, component: impl ComponentTrait) {
        if let Some(region) = self.0.get_mut(region) {
            region.alter_pack(PackOp::Add, component);
        } else {
            self.0.insert(region, PackComponents::new_with(component));
        }
    }

    pub fn get_extended_pack(&self, theme: &str, region: &str) -> PackComponents {
        if let Some(hm_theme) = THEME_REGIONS.read().unwrap().get(theme) {
            PackComponents::merge(self.0.get(region), hm_theme.0.get(region))
        } else {
            PackComponents::merge(self.0.get(region), None)
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
